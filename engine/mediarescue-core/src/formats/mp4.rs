use crate::device::reader::DeviceReader;
use crate::error::FormatError;
use crate::types::{MagicBytes, MediaFormat, RepairAction, ValidationResult};
use super::traits::FormatHandler;

pub struct Mp4Handler;

impl Mp4Handler {
    pub fn new() -> Self {
        Self
    }

    fn find_box(data: &[u8], box_type: &[u8; 4]) -> Option<(usize, u64)> {
        let mut pos = 0;
        while pos + 8 <= data.len() {
            let size = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as u64;
            let btype = &data[pos + 4..pos + 8];

            if btype == box_type {
                return Some((pos, size));
            }

            let advance = if size == 0 {
                break;
            } else if size == 1 && pos + 16 <= data.len() {
                u64::from_be_bytes([
                    data[pos + 8], data[pos + 9], data[pos + 10], data[pos + 11],
                    data[pos + 12], data[pos + 13], data[pos + 14], data[pos + 15],
                ])
            } else {
                size
            };

            pos += advance as usize;
        }
        None
    }

    fn walk_boxes(data: &[u8]) -> Vec<Mp4Box> {
        let mut boxes = Vec::new();
        let mut pos = 0;

        while pos + 8 <= data.len() {
            let size = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as u64;
            let btype = String::from_utf8_lossy(&data[pos + 4..pos + 8]).to_string();

            let actual_size = if size == 0 {
                (data.len() - pos) as u64
            } else if size == 1 && pos + 16 <= data.len() {
                u64::from_be_bytes([
                    data[pos + 8], data[pos + 9], data[pos + 10], data[pos + 11],
                    data[pos + 12], data[pos + 13], data[pos + 14], data[pos + 15],
                ])
            } else {
                size
            };

            boxes.push(Mp4Box {
                box_type: btype.clone(),
                offset: pos,
                size: actual_size,
            });

            if size == 0 || actual_size == 0 {
                break;
            }
            pos += actual_size as usize;
        }

        boxes
    }
}

impl Default for Mp4Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatHandler for Mp4Handler {
    fn format_name(&self) -> &str {
        "mp4"
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
        let sectors_to_read = 16.min(reader.total_sectors() - start_sector);

        let header_data = reader.read_sectors(start_sector, sectors_to_read).map_err(|e| {
            FormatError::CorruptedStructure {
                format: "mp4".to_string(),
                message: e.to_string(),
            }
        })?;

        let boxes = Self::walk_boxes(&header_data);
        let total_size: u64 = boxes.iter().map(|b| b.size).sum();

        if total_size > 0 {
            Ok(total_size)
        } else {
            Err(FormatError::CorruptedStructure {
                format: "mp4".to_string(),
                message: "cannot determine file size from box structure".to_string(),
            })
        }
    }

    fn validate(&self, data: &[u8]) -> ValidationResult {
        let mut issues = Vec::new();
        let mut suggested_repairs = Vec::new();
        let mut score: f32 = 0.0;

        if data.len() < 8 {
            return ValidationResult {
                is_valid: false,
                score: 0.0,
                issues: vec!["File too small to be MP4".to_string()],
                can_repair: false,
                suggested_repairs: vec![],
            };
        }

        let boxes = Self::walk_boxes(data);

        let has_ftyp = boxes.iter().any(|b| b.box_type == "ftyp");
        let has_moov = boxes.iter().any(|b| b.box_type == "moov");
        let has_mdat = boxes.iter().any(|b| b.box_type == "mdat");

        if has_ftyp {
            score += 0.3;
        } else {
            issues.push("Missing ftyp box".to_string());
            suggested_repairs.push(RepairAction::RebuildHeader);
        }

        if has_moov {
            score += 0.4;
        } else {
            issues.push("Missing moov box (video metadata)".to_string());
            suggested_repairs.push(RepairAction::ReconstructMoov);
        }

        if has_mdat {
            score += 0.3;
        } else {
            issues.push("Missing mdat box (media data)".to_string());
        }

        let is_valid = has_ftyp && has_moov && has_mdat;
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
        let boxes = Self::walk_boxes(data);

        if boxes.iter().any(|b| b.box_type == "moov") {
            return Ok(data.to_vec());
        }

        // TODO: Implement moov atom reconstruction from mdat analysis
        // This is the most complex recovery operation and requires:
        // 1. Analyzing mdat content to identify codec type
        // 2. Detecting frame boundaries (sync samples)
        // 3. Building sample table (stts, stsc, stsz, stco)
        // 4. Constructing trak + mdia + minf + stbl hierarchy
        Err(FormatError::RepairFailed {
            format: "mp4".to_string(),
            reason: "moov reconstruction not yet implemented".to_string(),
        })
    }

    fn generate_thumbnail(&self, _data: &[u8], _max_dim: u32) -> Result<Vec<u8>, FormatError> {
        // TODO: Extract first keyframe from mdat and decode
        Err(FormatError::RepairFailed {
            format: "mp4".to_string(),
            reason: "video thumbnail generation not yet implemented".to_string(),
        })
    }
}

#[derive(Debug)]
struct Mp4Box {
    box_type: String,
    offset: usize,
    size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_mp4() -> Vec<u8> {
        let mut data = Vec::new();
        // ftyp box
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x14]); // size = 20
        data.extend_from_slice(b"ftyp");
        data.extend_from_slice(b"isom"); // major brand
        data.extend_from_slice(&[0x00, 0x00, 0x02, 0x00]); // minor version
        data.extend_from_slice(b"isom"); // compatible brand
        // moov box (minimal)
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x08]); // size = 8
        data.extend_from_slice(b"moov");
        // mdat box (minimal)
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x08]); // size = 8
        data.extend_from_slice(b"mdat");
        data
    }

    #[test]
    fn test_validate_valid_mp4() {
        let handler = Mp4Handler::new();
        let data = minimal_mp4();
        let result = handler.validate(&data);
        assert!(result.is_valid);
        assert!(result.score > 0.9);
    }

    #[test]
    fn test_validate_missing_moov() {
        let handler = Mp4Handler::new();
        let mut data = Vec::new();
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x14]);
        data.extend_from_slice(b"ftyp");
        data.extend_from_slice(b"isom");
        data.extend_from_slice(&[0x00, 0x00, 0x02, 0x00]);
        data.extend_from_slice(b"isom");
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x08]);
        data.extend_from_slice(b"mdat");

        let result = handler.validate(&data);
        assert!(!result.is_valid);
        assert!(result.issues.iter().any(|i| i.contains("moov")));
        assert!(result.suggested_repairs.iter().any(|r| matches!(r, RepairAction::ReconstructMoov)));
    }

    #[test]
    fn test_too_small() {
        let handler = Mp4Handler::new();
        let result = handler.validate(&[0x00, 0x01]);
        assert!(!result.is_valid);
    }
}
