use crate::error::DeviceError;
use super::reader::DeviceReader;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;

pub struct DiskImage {
    file: Mutex<File>,
    sector_size: u32,
    total_sectors: u64,
    path: String,
}

impl DiskImage {
    pub fn open(path: &str) -> Result<Self, DeviceError> {
        let file = File::open(path).map_err(|e| DeviceError::OpenFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

        let size = file.metadata().map_err(|e| DeviceError::OpenFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?.len();

        let sector_size = 512u32;

        Ok(Self {
            file: Mutex::new(file),
            sector_size,
            total_sectors: size / sector_size as u64,
            path: path.to_string(),
        })
    }
}

impl DeviceReader for DiskImage {
    fn read_sectors(&self, start: u64, count: u64) -> Result<Vec<u8>, DeviceError> {
        let offset = start * self.sector_size as u64;
        let bytes_to_read = (count * self.sector_size as u64) as usize;

        let mut file = self.file.lock().map_err(|_| DeviceError::ReadFailed {
            sector: start,
            reason: "lock poisoned".to_string(),
        })?;

        file.seek(SeekFrom::Start(offset)).map_err(|e| DeviceError::ReadFailed {
            sector: start,
            reason: e.to_string(),
        })?;

        let mut buffer = vec![0u8; bytes_to_read];
        let bytes_read = file.read(&mut buffer).map_err(|e| DeviceError::ReadFailed {
            sector: start,
            reason: e.to_string(),
        })?;

        buffer.truncate(bytes_read);
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
