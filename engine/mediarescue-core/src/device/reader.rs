use crate::error::DeviceError;

pub trait DeviceReader: Send + Sync {
    fn read_sectors(&self, start: u64, count: u64) -> Result<Vec<u8>, DeviceError>;
    fn sector_size(&self) -> u32;
    fn total_sectors(&self) -> u64;
    fn total_bytes(&self) -> u64 {
        self.total_sectors() * self.sector_size() as u64
    }
    fn is_connected(&self) -> bool;
}

pub struct RawDeviceReader {
    path: String,
    sector_size: u32,
    total_sectors: u64,
}

impl RawDeviceReader {
    pub fn open(path: &str) -> Result<Self, DeviceError> {
        #[cfg(target_os = "windows")]
        {
            Self::open_windows(path)
        }
        #[cfg(not(target_os = "windows"))]
        {
            Self::open_unix(path)
        }
    }

    #[cfg(target_os = "windows")]
    fn open_windows(path: &str) -> Result<Self, DeviceError> {
        use std::fs::File;
        use std::io::{Read, Seek, SeekFrom};

        let mut file = File::open(path).map_err(|e| DeviceError::OpenFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

        let size = file.seek(SeekFrom::End(0)).map_err(|e| DeviceError::OpenFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

        let sector_size = 512u32;
        let total_sectors = size / sector_size as u64;

        Ok(Self {
            path: path.to_string(),
            sector_size,
            total_sectors,
        })
    }

    #[cfg(not(target_os = "windows"))]
    fn open_unix(path: &str) -> Result<Self, DeviceError> {
        use std::fs::metadata;

        let meta = metadata(path).map_err(|e| DeviceError::OpenFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

        let sector_size = 512u32;
        let total_sectors = meta.len() / sector_size as u64;

        Ok(Self {
            path: path.to_string(),
            sector_size,
            total_sectors,
        })
    }
}

impl DeviceReader for RawDeviceReader {
    fn read_sectors(&self, start: u64, count: u64) -> Result<Vec<u8>, DeviceError> {
        use std::fs::File;
        use std::io::{Read, Seek, SeekFrom};

        let offset = start * self.sector_size as u64;
        let bytes_to_read = (count * self.sector_size as u64) as usize;

        let mut file = File::open(&self.path).map_err(|e| DeviceError::ReadFailed {
            sector: start,
            reason: e.to_string(),
        })?;

        file.seek(SeekFrom::Start(offset)).map_err(|e| DeviceError::ReadFailed {
            sector: start,
            reason: e.to_string(),
        })?;

        let mut buffer = vec![0u8; bytes_to_read];
        file.read_exact(&mut buffer).map_err(|e| DeviceError::ReadFailed {
            sector: start,
            reason: e.to_string(),
        })?;

        Ok(buffer)
    }

    fn sector_size(&self) -> u32 {
        self.sector_size
    }

    fn total_sectors(&self) -> u64 {
        self.total_sectors
    }

    fn is_connected(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDeviceReader {
        data: Vec<u8>,
        sector_size: u32,
    }

    impl MockDeviceReader {
        fn new(data: Vec<u8>, sector_size: u32) -> Self {
            Self { data, sector_size }
        }
    }

    impl DeviceReader for MockDeviceReader {
        fn read_sectors(&self, start: u64, count: u64) -> Result<Vec<u8>, DeviceError> {
            let offset = (start * self.sector_size as u64) as usize;
            let end = offset + (count * self.sector_size as u64) as usize;
            if end > self.data.len() {
                return Err(DeviceError::ReadFailed {
                    sector: start,
                    reason: "read past end of device".to_string(),
                });
            }
            Ok(self.data[offset..end].to_vec())
        }

        fn sector_size(&self) -> u32 {
            self.sector_size
        }

        fn total_sectors(&self) -> u64 {
            self.data.len() as u64 / self.sector_size as u64
        }

        fn is_connected(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_mock_reader_read_sectors() {
        let data: Vec<u8> = (0..2048).map(|i| (i % 256) as u8).collect();
        let reader = MockDeviceReader::new(data.clone(), 512);

        assert_eq!(reader.total_sectors(), 4);
        assert_eq!(reader.total_bytes(), 2048);

        let sector = reader.read_sectors(0, 1).unwrap();
        assert_eq!(sector.len(), 512);
        assert_eq!(sector[0], 0);
    }

    #[test]
    fn test_mock_reader_out_of_bounds() {
        let data = vec![0u8; 1024];
        let reader = MockDeviceReader::new(data, 512);

        let result = reader.read_sectors(3, 1);
        assert!(result.is_err());
    }
}
