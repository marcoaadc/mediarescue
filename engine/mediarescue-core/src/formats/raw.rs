use crate::device::reader::DeviceReader;
use crate::error::FormatError;
use crate::types::{MagicBytes, ValidationResult};
use super::traits::FormatHandler;

pub struct RawHandler;

impl RawHandler {
    pub fn new() -> Self {
        Self
    }

    fn detect_raw_variant(data: &[u8]) -> Option<&'static str> {
        if data.len() < 10 {
            return None;
        }

        // TIFF Little Endian (II)
        if data[0] == 0x49 && data[1] == 0x49 && data[2] == 0x2A && data[3] == 0x00 {
            // Canon CR2 has specific pattern at offset 8-9
            if data.len() > 9 && data[8] == 0x43 && data[9] == 0x52 {
                return Some("cr2");
            }
            // Sony ARW is also TIFF LE
            return Some("arw");
        }

        // TIFF Big Endian (MM) - Nikon NEF
        if data[0] == 0x4D && data[1] == 0x4D && data[2] == 0x00 && data[3] == 0x2A {
            return Some("nef");
        }

        None
    }
}

impl Default for RawHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatHandler for RawHandler {
    fn format_name(&self) -> &str {
        "raw"
    }

    fn signatures(&self) -> &[MagicBytes] {
        &[]
    }

    fn estimate_file_size(
        &self,
        _header: &[u8],
        _reader: &dyn DeviceReader,
        _offset: u64,
    ) -> Result<u64, FormatError> {
        // TODO: Parse TIFF IFD chain to determine file size
        // RAW files are typically 10-60MB
        Ok(25 * 1024 * 1024) // default estimate: 25MB
    }

    fn validate(&self, data: &[u8]) -> ValidationResult {
        let mut issues = Vec::new();
        let mut score: f32 = 0.0;

        match Self::detect_raw_variant(data) {
            Some(variant) => {
                score += 0.4;
                // Check TIFF IFD pointer
                if data.len() >= 8 {
                    let ifd_offset = if data[0] == 0x49 {
                        u32::from_le_bytes([data[4], data[5], data[6], data[7]])
                    } else {
                        u32::from_be_bytes([data[4], data[5], data[6], data[7]])
                    };

                    if (ifd_offset as usize) < data.len() {
                        score += 0.3;
                    } else {
                        issues.push(format!("IFD offset {} points beyond file", ifd_offset));
                    }
                }

                if data.len() > 1024 {
                    score += 0.3;
                }
            }
            None => {
                issues.push("Not a recognized RAW format".to_string());
            }
        }

        ValidationResult {
            is_valid: issues.is_empty(),
            score,
            issues,
            can_repair: false,
            suggested_repairs: vec![],
        }
    }

    fn repair(&self, data: &[u8]) -> Result<Vec<u8>, FormatError> {
        Ok(data.to_vec())
    }

    fn generate_thumbnail(&self, _data: &[u8], _max_dim: u32) -> Result<Vec<u8>, FormatError> {
        // TODO: Extract embedded JPEG thumbnail from EXIF
        Err(FormatError::RepairFailed {
            format: "raw".to_string(),
            reason: "RAW thumbnail extraction not yet implemented".to_string(),
        })
    }
}
