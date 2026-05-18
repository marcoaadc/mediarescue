use crate::device::reader::DeviceReader;
use crate::error::FormatError;
use crate::types::{MagicBytes, RepairAction, ValidationResult};
use super::traits::FormatHandler;

pub struct AviHandler;

impl AviHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AviHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatHandler for AviHandler {
    fn format_name(&self) -> &str {
        "avi"
    }

    fn signatures(&self) -> &[MagicBytes] {
        &[]
    }

    fn estimate_file_size(
        &self,
        _header: &[u8],
        reader: &dyn DeviceReader,
        offset: u64,
    ) -> Result<u64, FormatError> {
        let sector_size = reader.sector_size() as u64;
        let start_sector = offset / sector_size;
        let sectors_to_read = 2.min(reader.total_sectors() - start_sector);

        let data = reader.read_sectors(start_sector, sectors_to_read).map_err(|e| {
            FormatError::CorruptedStructure {
                format: "avi".to_string(),
                message: e.to_string(),
            }
        })?;

        if data.len() >= 8 && &data[0..4] == b"RIFF" {
            let size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as u64;
            Ok(size + 8)
        } else {
            Err(FormatError::InvalidHeader {
                format: "avi".to_string(),
                offset,
            })
        }
    }

    fn validate(&self, data: &[u8]) -> ValidationResult {
        let mut issues = Vec::new();
        let mut score: f32 = 0.0;

        if data.len() >= 4 && &data[0..4] == b"RIFF" {
            score += 0.3;
        } else {
            issues.push("Missing RIFF header".to_string());
        }

        if data.len() >= 12 && &data[8..12] == b"AVI " {
            score += 0.3;
        } else {
            issues.push("Missing AVI signature".to_string());
        }

        if data.len() >= 4 {
            let declared_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize + 8;
            if data.len() >= declared_size {
                score += 0.4;
            } else {
                issues.push("File truncated".to_string());
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
        // TODO: Implement AVI index (idx1) reconstruction
        Ok(data.to_vec())
    }

    fn generate_thumbnail(&self, _data: &[u8], _max_dim: u32) -> Result<Vec<u8>, FormatError> {
        Err(FormatError::RepairFailed {
            format: "avi".to_string(),
            reason: "AVI thumbnail generation not yet implemented".to_string(),
        })
    }
}
