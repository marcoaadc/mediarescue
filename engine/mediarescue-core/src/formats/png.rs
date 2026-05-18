use crate::device::reader::DeviceReader;
use crate::error::FormatError;
use crate::types::{MagicBytes, MediaFormat, RepairAction, ValidationResult};
use super::traits::FormatHandler;

pub struct PngHandler;

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const IEND_CHUNK: [u8; 8] = [0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

impl PngHandler {
    pub fn new() -> Self {
        Self
    }

    fn walk_chunks(data: &[u8]) -> Vec<PngChunk> {
        let mut chunks = Vec::new();
        let mut pos = 8; // skip PNG signature

        while pos + 12 <= data.len() {
            let length = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            let chunk_type = &data[pos + 4..pos + 8];
            let chunk_name = String::from_utf8_lossy(chunk_type).to_string();

            let crc_offset = pos + 8 + length;
            let stored_crc = if crc_offset + 4 <= data.len() {
                Some(u32::from_be_bytes([
                    data[crc_offset],
                    data[crc_offset + 1],
                    data[crc_offset + 2],
                    data[crc_offset + 3],
                ]))
            } else {
                None
            };

            chunks.push(PngChunk {
                name: chunk_name.clone(),
                offset: pos,
                data_length: length,
                stored_crc,
            });

            if chunk_name == "IEND" {
                break;
            }

            pos = crc_offset + 4;
        }

        chunks
    }
}

impl Default for PngHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatHandler for PngHandler {
    fn format_name(&self) -> &str {
        "png"
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
        let search_size = 20 * 1024 * 1024;
        let sector_size = reader.sector_size() as u64;
        let start_sector = offset / sector_size;
        let sectors_to_read = (search_size / sector_size).min(reader.total_sectors() - start_sector);

        let data = reader.read_sectors(start_sector, sectors_to_read).map_err(|e| {
            FormatError::CorruptedStructure {
                format: "png".to_string(),
                message: e.to_string(),
            }
        })?;

        if let Some(pos) = data.windows(IEND_CHUNK.len()).position(|w| w == IEND_CHUNK) {
            Ok((pos + IEND_CHUNK.len() + 4) as u64) // +4 for CRC after IEND
        } else {
            Ok(data.len() as u64)
        }
    }

    fn validate(&self, data: &[u8]) -> ValidationResult {
        let mut issues = Vec::new();
        let mut suggested_repairs = Vec::new();
        let mut score: f32 = 0.0;

        if data.len() < 8 || data[..8] != PNG_SIGNATURE {
            issues.push("Missing PNG signature".to_string());
            suggested_repairs.push(RepairAction::RebuildHeader);
        } else {
            score += 0.2;
        }

        let chunks = Self::walk_chunks(data);

        if chunks.iter().any(|c| c.name == "IHDR") {
            score += 0.3;
        } else {
            issues.push("Missing IHDR chunk".to_string());
            suggested_repairs.push(RepairAction::RebuildHeader);
        }

        let idat_count = chunks.iter().filter(|c| c.name == "IDAT").count();
        if idat_count > 0 {
            score += 0.3;
        } else {
            issues.push("No IDAT chunks found".to_string());
        }

        if chunks.iter().any(|c| c.name == "IEND") {
            score += 0.2;
        } else {
            issues.push("Missing IEND chunk".to_string());
            suggested_repairs.push(RepairAction::PadTruncatedData);
        }

        let is_valid = issues.is_empty();
        let can_repair = !suggested_repairs.is_empty();

        ValidationResult {
            is_valid,
            score,
            issues,
            can_repair,
            suggested_repairs,
        }
    }

    fn repair(&self, data: &[u8]) -> Result<Vec<u8>, FormatError> {
        let mut repaired = data.to_vec();

        // Ensure PNG signature
        if repaired.len() < 8 || repaired[..8] != PNG_SIGNATURE {
            let mut fixed = PNG_SIGNATURE.to_vec();
            if repaired.len() >= 8 && repaired[..8] == PNG_SIGNATURE {
                fixed.extend_from_slice(&repaired[8..]);
            } else {
                fixed.extend_from_slice(&repaired);
            }
            repaired = fixed;
        }

        // Add IEND if missing
        if !repaired.windows(4).any(|w| w == b"IEND") {
            // IEND chunk: length(0) + "IEND" + CRC
            repaired.extend_from_slice(&[
                0x00, 0x00, 0x00, 0x00, // length = 0
                0x49, 0x45, 0x4E, 0x44, // IEND
                0xAE, 0x42, 0x60, 0x82, // CRC of IEND
            ]);
        }

        Ok(repaired)
    }

    fn generate_thumbnail(&self, data: &[u8], max_dim: u32) -> Result<Vec<u8>, FormatError> {
        use image::io::Reader as ImageReader;
        use std::io::Cursor;

        let reader = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .map_err(|e| FormatError::CorruptedStructure {
                format: "png".to_string(),
                message: e.to_string(),
            })?;

        let img = reader.decode().map_err(|e| FormatError::CorruptedStructure {
            format: "png".to_string(),
            message: format!("decode failed: {}", e),
        })?;

        let thumbnail = img.thumbnail(max_dim, max_dim);
        let mut buffer = Vec::new();
        thumbnail
            .write_to(
                &mut Cursor::new(&mut buffer),
                image::ImageFormat::Jpeg,
            )
            .map_err(|e| FormatError::RepairFailed {
                format: "png".to_string(),
                reason: e.to_string(),
            })?;

        Ok(buffer)
    }
}

#[derive(Debug)]
struct PngChunk {
    name: String,
    offset: usize,
    data_length: usize,
    stored_crc: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_png_signature() {
        let handler = PngHandler::new();
        let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let result = handler.validate(&data);
        assert!(result.score >= 0.2);
    }

    #[test]
    fn test_validate_missing_signature() {
        let handler = PngHandler::new();
        let data = vec![0x00; 100];
        let result = handler.validate(&data);
        assert!(!result.is_valid);
        assert!(result.issues.iter().any(|i| i.contains("signature")));
    }

    #[test]
    fn test_repair_adds_iend() {
        let handler = PngHandler::new();
        let mut data = PNG_SIGNATURE.to_vec();
        data.extend_from_slice(&[0x00; 20]);
        let repaired = handler.repair(&data).unwrap();
        assert!(repaired.windows(4).any(|w| w == b"IEND"));
    }
}
