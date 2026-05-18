use crate::error::DeviceError;
use crate::types::{DeviceInfo, DeviceType};

pub trait DeviceEnumerator: Send + Sync {
    fn list_devices(&self) -> Result<Vec<DeviceInfo>, DeviceError>;
}

pub struct SystemDeviceEnumerator;

impl SystemDeviceEnumerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemDeviceEnumerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceEnumerator for SystemDeviceEnumerator {
    fn list_devices(&self) -> Result<Vec<DeviceInfo>, DeviceError> {
        #[cfg(target_os = "windows")]
        {
            list_windows_devices()
        }
        #[cfg(not(target_os = "windows"))]
        {
            list_unix_devices()
        }
    }
}

#[cfg(target_os = "windows")]
fn list_windows_devices() -> Result<Vec<DeviceInfo>, DeviceError> {
    let mut devices = Vec::new();

    for i in 0..16 {
        let path = format!(r"\\.\PhysicalDrive{}", i);
        if let Ok(reader) = crate::device::reader::RawDeviceReader::open(&path) {
            use crate::device::reader::DeviceReader;
            devices.push(DeviceInfo {
                id: format!("drive-{}", i),
                name: format!("Physical Drive {}", i),
                path: path.clone(),
                size_bytes: reader.total_bytes(),
                sector_size: reader.sector_size(),
                total_sectors: reader.total_sectors(),
                device_type: DeviceType::Unknown,
                removable: false,
            });
        }
    }

    Ok(devices)
}

#[cfg(not(target_os = "windows"))]
fn list_unix_devices() -> Result<Vec<DeviceInfo>, DeviceError> {
    Ok(Vec::new())
}

pub struct DiskImageEnumerator;

impl DiskImageEnumerator {
    pub fn from_path(path: &str) -> Result<DeviceInfo, DeviceError> {
        let metadata = std::fs::metadata(path).map_err(|e| DeviceError::OpenFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

        let size = metadata.len();
        let sector_size = 512u32;

        Ok(DeviceInfo {
            id: format!("image-{}", uuid::Uuid::new_v4()),
            name: std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("disk.img")
                .to_string(),
            path: path.to_string(),
            size_bytes: size,
            sector_size,
            total_sectors: size / sector_size as u64,
            device_type: DeviceType::DiskImage,
            removable: false,
        })
    }
}
