use crate::types::{MagicBytes, MediaFormat, SignatureMatch};

pub struct SignatureDB {
    signatures: Vec<MagicBytes>,
}

impl SignatureDB {
    pub fn new() -> Self {
        Self {
            signatures: Self::build_database(),
        }
    }

    pub fn with_formats(formats: &[MediaFormat]) -> Self {
        let all = Self::build_database();
        let filtered = all
            .into_iter()
            .filter(|sig| formats.contains(&sig.format))
            .collect();
        Self { signatures: filtered }
    }

    pub fn scan_buffer(&self, buffer: &[u8], base_offset: u64) -> Vec<SignatureMatch> {
        let mut matches = Vec::new();

        for i in 0..buffer.len() {
            for sig in &self.signatures {
                let header_start = sig.header_offset as usize;
                if i < header_start {
                    continue;
                }
                let check_pos = i - header_start;
                if check_pos + sig.header.len() > buffer.len() {
                    continue;
                }
                if buffer[check_pos..check_pos + sig.header.len()] == sig.header[..] {
                    matches.push(SignatureMatch {
                        format: sig.format,
                        disk_offset: base_offset + check_pos as u64,
                        confidence: Self::calculate_confidence(sig, buffer, check_pos),
                    });
                }
            }
        }

        matches
    }

    fn calculate_confidence(sig: &MagicBytes, buffer: &[u8], pos: usize) -> f32 {
        let mut score: f32 = 0.5;

        if sig.header.len() >= 4 {
            score += 0.2;
        }

        if let Some(ref footer) = sig.footer {
            let remaining = &buffer[pos..];
            if remaining.windows(footer.len()).any(|w| w == footer.as_slice()) {
                score += 0.3;
            }
        }

        score.min(1.0)
    }

    fn build_database() -> Vec<MagicBytes> {
        vec![
            // JPEG (JFIF)
            MagicBytes {
                format: MediaFormat::Jpeg,
                header: vec![0xFF, 0xD8, 0xFF, 0xE0],
                header_offset: 0,
                footer: Some(vec![0xFF, 0xD9]),
            },
            // JPEG (EXIF)
            MagicBytes {
                format: MediaFormat::Jpeg,
                header: vec![0xFF, 0xD8, 0xFF, 0xE1],
                header_offset: 0,
                footer: Some(vec![0xFF, 0xD9]),
            },
            // JPEG (generic)
            MagicBytes {
                format: MediaFormat::Jpeg,
                header: vec![0xFF, 0xD8, 0xFF],
                header_offset: 0,
                footer: Some(vec![0xFF, 0xD9]),
            },
            // PNG
            MagicBytes {
                format: MediaFormat::Png,
                header: vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
                header_offset: 0,
                footer: Some(vec![0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82]),
            },
            // MP4 (ftyp at offset 4)
            MagicBytes {
                format: MediaFormat::Mp4,
                header: vec![0x66, 0x74, 0x79, 0x70],
                header_offset: 4,
                footer: None,
            },
            // MOV (ftyp qt)
            MagicBytes {
                format: MediaFormat::Mov,
                header: vec![0x66, 0x74, 0x79, 0x70, 0x71, 0x74],
                header_offset: 4,
                footer: None,
            },
            // AVI (RIFF....AVI )
            MagicBytes {
                format: MediaFormat::Avi,
                header: vec![0x52, 0x49, 0x46, 0x46],
                header_offset: 0,
                footer: None,
            },
            // MKV/WebM (EBML)
            MagicBytes {
                format: MediaFormat::Mkv,
                header: vec![0x1A, 0x45, 0xDF, 0xA3],
                header_offset: 0,
                footer: None,
            },
            // Canon CR2 (TIFF LE + CR2)
            MagicBytes {
                format: MediaFormat::Cr2,
                header: vec![0x49, 0x49, 0x2A, 0x00, 0x10, 0x00, 0x00, 0x00, 0x43, 0x52],
                header_offset: 0,
                footer: None,
            },
            // Nikon NEF (TIFF BE)
            MagicBytes {
                format: MediaFormat::Nef,
                header: vec![0x4D, 0x4D, 0x00, 0x2A],
                header_offset: 0,
                footer: None,
            },
            // Sony ARW (TIFF LE)
            MagicBytes {
                format: MediaFormat::Arw,
                header: vec![0x49, 0x49, 0x2A, 0x00],
                header_offset: 0,
                footer: None,
            },
        ]
    }
}

impl Default for SignatureDB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jpeg_jfif() {
        let db = SignatureDB::new();
        let mut buffer = vec![0u8; 100];
        buffer[0] = 0xFF;
        buffer[1] = 0xD8;
        buffer[2] = 0xFF;
        buffer[3] = 0xE0;

        let matches = db.scan_buffer(&buffer, 0);
        assert!(!matches.is_empty());
        assert!(matches.iter().any(|m| m.format == MediaFormat::Jpeg));
    }

    #[test]
    fn test_detect_png() {
        let db = SignatureDB::new();
        let buffer = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0];

        let matches = db.scan_buffer(&buffer, 0);
        assert!(!matches.is_empty());
        assert!(matches.iter().any(|m| m.format == MediaFormat::Png));
    }

    #[test]
    fn test_detect_mp4_ftyp() {
        let db = SignatureDB::new();
        let mut buffer = vec![0u8; 20];
        // Size bytes at offset 0-3, then "ftyp" at offset 4
        buffer[0] = 0x00;
        buffer[1] = 0x00;
        buffer[2] = 0x00;
        buffer[3] = 0x18;
        buffer[4] = 0x66; // f
        buffer[5] = 0x74; // t
        buffer[6] = 0x79; // y
        buffer[7] = 0x70; // p

        let matches = db.scan_buffer(&buffer, 0);
        assert!(!matches.is_empty());
        assert!(matches.iter().any(|m| m.format == MediaFormat::Mp4));
    }

    #[test]
    fn test_no_false_positive_on_empty() {
        let db = SignatureDB::new();
        let buffer = vec![0u8; 1024];

        let matches = db.scan_buffer(&buffer, 0);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_format_filter() {
        let db = SignatureDB::with_formats(&[MediaFormat::Jpeg]);
        let buffer = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0];

        let matches = db.scan_buffer(&buffer, 0);
        assert!(matches.is_empty()); // PNG should not match when filtering for JPEG only
    }

    #[test]
    fn test_jpeg_with_footer_higher_confidence() {
        let db = SignatureDB::new();
        let mut buffer = vec![0u8; 100];
        buffer[0] = 0xFF;
        buffer[1] = 0xD8;
        buffer[2] = 0xFF;
        buffer[3] = 0xE0;
        buffer[98] = 0xFF;
        buffer[99] = 0xD9;

        let matches = db.scan_buffer(&buffer, 0);
        let jpeg_match = matches.iter().find(|m| m.format == MediaFormat::Jpeg).unwrap();
        assert!(jpeg_match.confidence > 0.7);
    }
}
