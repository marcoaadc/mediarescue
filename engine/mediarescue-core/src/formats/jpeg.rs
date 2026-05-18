use crate::device::reader::DeviceReader;
use crate::error::FormatError;
use crate::types::{MagicBytes, MediaFormat, RepairAction, ValidationResult};
use super::traits::FormatHandler;

pub struct JpegHandler;

impl JpegHandler {
    pub fn new() -> Self {
        Self
    }

    fn find_marker(data: &[u8], marker: &[u8]) -> Option<usize> {
        data.windows(marker.len())
            .position(|w| w == marker)
    }

    fn find_all_markers(data: &[u8], marker: &[u8]) -> Vec<usize> {
        data.windows(marker.len())
            .enumerate()
            .filter_map(|(i, w)| if w == marker { Some(i) } else { None })
            .collect()
    }

    fn find_eoi(data: &[u8]) -> Option<usize> {
        Self::find_marker(data, &[0xFF, 0xD9])
    }

    fn find_last_eoi(data: &[u8]) -> Option<usize> {
        Self::find_all_markers(data, &[0xFF, 0xD9]).last().copied()
    }

    fn walk_markers(data: &[u8]) -> Vec<JpegMarker> {
        let mut markers = Vec::new();
        let mut pos = 0;

        while pos < data.len() - 1 {
            if data[pos] != 0xFF {
                pos += 1;
                continue;
            }

            let marker_byte = data[pos + 1];

            if marker_byte == 0x00 || marker_byte == 0xFF {
                pos += 1;
                continue;
            }

            let marker_type = match marker_byte {
                0xD8 => MarkerType::SOI,
                0xD9 => MarkerType::EOI,
                0xDA => MarkerType::SOS,
                0xC0..=0xC3 => MarkerType::SOF,
                0xC4 => MarkerType::DHT,
                0xDB => MarkerType::DQT,
                0xDD => MarkerType::DRI,
                0xE0 => MarkerType::APP0,
                0xE1 => MarkerType::APP1,
                0xFE => MarkerType::COM,
                _ => MarkerType::Unknown(marker_byte),
            };

            let length = if matches!(marker_type, MarkerType::SOI | MarkerType::EOI) {
                0
            } else if pos + 3 < data.len() {
                let len = u16::from_be_bytes([data[pos + 2], data[pos + 3]]) as usize;
                len.saturating_sub(2)
            } else {
                0
            };

            markers.push(JpegMarker {
                marker_type,
                offset: pos,
                data_length: length,
            });

            if matches!(marker_type, MarkerType::SOS) {
                break;
            }

            pos += 2 + if length > 0 { length + 2 } else { 0 };
        }

        markers
    }

    fn score_jpeg(data: &[u8]) -> f32 {
        let mut score: f32 = 0.0;
        let markers = Self::walk_markers(data);

        let has_soi = markers.iter().any(|m| matches!(m.marker_type, MarkerType::SOI));
        let has_eoi = Self::find_last_eoi(data).is_some();
        let has_sof = markers.iter().any(|m| matches!(m.marker_type, MarkerType::SOF));
        let has_dht = markers.iter().any(|m| matches!(m.marker_type, MarkerType::DHT));
        let has_dqt = markers.iter().any(|m| matches!(m.marker_type, MarkerType::DQT));
        let has_sos = markers.iter().any(|m| matches!(m.marker_type, MarkerType::SOS));

        if has_soi { score += 0.15; }
        if has_eoi { score += 0.15; }
        if has_sof { score += 0.2; }
        if has_dht { score += 0.15; }
        if has_dqt { score += 0.15; }
        if has_sos { score += 0.2; }

        score
    }
}

impl Default for JpegHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatHandler for JpegHandler {
    fn format_name(&self) -> &str {
        "jpeg"
    }

    fn signatures(&self) -> &[MagicBytes] {
        &[]
    }

    fn estimate_file_size(
        &self,
        header: &[u8],
        reader: &dyn DeviceReader,
        offset: u64,
    ) -> Result<u64, FormatError> {
        let search_size = 10 * 1024 * 1024; // 10MB max search
        let sector_size = reader.sector_size() as u64;
        let sectors_to_read = (search_size / sector_size).min(reader.total_sectors() - offset / sector_size);

        let data = reader.read_sectors(offset / sector_size, sectors_to_read).map_err(|e| {
            FormatError::CorruptedStructure {
                format: "jpeg".to_string(),
                message: e.to_string(),
            }
        })?;

        if let Some(eoi_pos) = Self::find_last_eoi(&data) {
            Ok((eoi_pos + 2) as u64)
        } else {
            Ok(data.len() as u64)
        }
    }

    fn validate(&self, data: &[u8]) -> ValidationResult {
        if data.len() < 4 {
            return ValidationResult {
                is_valid: false,
                score: 0.0,
                issues: vec!["File too small to be JPEG".to_string()],
                can_repair: false,
                suggested_repairs: vec![],
            };
        }

        let mut issues = Vec::new();
        let mut suggested_repairs = Vec::new();

        if data[0] != 0xFF || data[1] != 0xD8 {
            issues.push("Missing SOI marker (FF D8)".to_string());
            suggested_repairs.push(RepairAction::RebuildHeader);
        }

        if Self::find_last_eoi(data).is_none() {
            issues.push("Missing EOI marker (FF D9)".to_string());
            suggested_repairs.push(RepairAction::PadTruncatedData);
        }

        let markers = Self::walk_markers(data);
        if !markers.iter().any(|m| matches!(m.marker_type, MarkerType::SOF)) {
            issues.push("Missing SOF marker (start of frame)".to_string());
            suggested_repairs.push(RepairAction::RebuildHeader);
        }

        if !markers.iter().any(|m| matches!(m.marker_type, MarkerType::DHT)) {
            issues.push("Missing DHT marker (Huffman table)".to_string());
        }

        let score = Self::score_jpeg(data);
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

        // Ensure SOI marker
        if repaired.len() < 2 || repaired[0] != 0xFF || repaired[1] != 0xD8 {
            let mut fixed = vec![0xFF, 0xD8];
            if repaired.len() >= 2 && repaired[0] == 0xFF && repaired[1] == 0xD8 {
                fixed.extend_from_slice(&repaired[2..]);
            } else {
                fixed.extend_from_slice(&repaired);
            }
            repaired = fixed;
        }

        // Ensure EOI marker
        if Self::find_last_eoi(&repaired).is_none() {
            repaired.extend_from_slice(&[0xFF, 0xD9]);
        }

        // Remove trailing garbage after EOI
        if let Some(eoi_pos) = Self::find_last_eoi(&repaired) {
            repaired.truncate(eoi_pos + 2);
        }

        Ok(repaired)
    }

    fn generate_thumbnail(&self, data: &[u8], max_dim: u32) -> Result<Vec<u8>, FormatError> {
        use image::io::Reader as ImageReader;
        use std::io::Cursor;

        let reader = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .map_err(|e| FormatError::CorruptedStructure {
                format: "jpeg".to_string(),
                message: e.to_string(),
            })?;

        let img = reader.decode().map_err(|e| FormatError::CorruptedStructure {
            format: "jpeg".to_string(),
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
                format: "jpeg".to_string(),
                reason: e.to_string(),
            })?;

        Ok(buffer)
    }
}

#[derive(Debug)]
struct JpegMarker {
    marker_type: MarkerType,
    offset: usize,
    data_length: usize,
}

#[derive(Debug)]
enum MarkerType {
    SOI,
    EOI,
    SOS,
    SOF,
    DHT,
    DQT,
    DRI,
    APP0,
    APP1,
    COM,
    Unknown(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_jpeg() -> Vec<u8> {
        vec![
            0xFF, 0xD8, 0xFF, 0xE0, // SOI + APP0
            0x00, 0x10, // APP0 length
            0x4A, 0x46, 0x49, 0x46, 0x00, // "JFIF\0"
            0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0xFF, 0xDB, // DQT
            0x00, 0x43, // DQT length
            0x00, // table 0
            // 64 bytes of quantization data (simplified)
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            0xFF, 0xC0, // SOF0
            0x00, 0x0B, // length
            0x08, // precision
            0x00, 0x01, // height
            0x00, 0x01, // width
            0x01, // components
            0x01, 0x11, 0x00, // component 1
            0xFF, 0xC4, // DHT
            0x00, 0x1F, // length
            0x00, // table class + id
            0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
            0xFF, 0xDA, // SOS
            0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00,
            0x7B, 0x40, // scan data
            0xFF, 0xD9, // EOI
        ]
    }

    #[test]
    fn test_validate_valid_jpeg() {
        let handler = JpegHandler::new();
        let data = minimal_jpeg();
        let result = handler.validate(&data);
        assert!(result.score > 0.5);
    }

    #[test]
    fn test_validate_missing_eoi() {
        let handler = JpegHandler::new();
        let mut data = minimal_jpeg();
        // Remove last 2 bytes (EOI)
        data.truncate(data.len() - 2);
        let result = handler.validate(&data);
        assert!(!result.is_valid);
        assert!(result.issues.iter().any(|i| i.contains("EOI")));
    }

    #[test]
    fn test_validate_missing_soi() {
        let handler = JpegHandler::new();
        let data = vec![0x00, 0x00, 0xFF, 0xD9];
        let result = handler.validate(&data);
        assert!(!result.is_valid);
        assert!(result.issues.iter().any(|i| i.contains("SOI")));
    }

    #[test]
    fn test_repair_adds_eoi() {
        let handler = JpegHandler::new();
        let mut data = minimal_jpeg();
        data.truncate(data.len() - 2); // remove EOI
        let repaired = handler.repair(&data).unwrap();
        assert_eq!(&repaired[repaired.len() - 2..], &[0xFF, 0xD9]);
    }

    #[test]
    fn test_repair_adds_soi() {
        let handler = JpegHandler::new();
        let data = vec![0x00, 0x01, 0x02, 0xFF, 0xD9];
        let repaired = handler.repair(&data).unwrap();
        assert_eq!(repaired[0], 0xFF);
        assert_eq!(repaired[1], 0xD8);
    }

    #[test]
    fn test_too_small() {
        let handler = JpegHandler::new();
        let result = handler.validate(&[0xFF]);
        assert!(!result.is_valid);
        assert_eq!(result.score, 0.0);
    }
}
