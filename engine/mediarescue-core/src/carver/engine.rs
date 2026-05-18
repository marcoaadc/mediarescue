use crate::device::reader::DeviceReader;
use crate::error::CarveError;
use crate::types::{MediaFormat, RecoveredFile, FileStatus, SignatureMatch};
use super::strategy::{CarvingStrategy, HeaderFooterStrategy};
use chrono::Utc;
use uuid::Uuid;

pub struct CarvingEngine {
    max_file_size: u64,
}

impl CarvingEngine {
    pub fn new(max_file_size: u64) -> Self {
        Self { max_file_size }
    }

    pub fn carve_file(
        &self,
        signature: &SignatureMatch,
        reader: &dyn DeviceReader,
    ) -> Result<(RecoveredFile, Vec<u8>), CarveError> {
        let strategy = self.strategy_for_format(&signature.format);
        let data = strategy.carve(signature, reader)?;

        let file = RecoveredFile {
            id: Uuid::new_v4(),
            format: signature.format,
            disk_offset: signature.disk_offset,
            size_bytes: data.len() as u64,
            recovery_score: signature.confidence,
            status: FileStatus::Carved,
            can_repair: false,
            discovered_at: Utc::now(),
        };

        Ok((file, data))
    }

    fn strategy_for_format(&self, format: &MediaFormat) -> Box<dyn CarvingStrategy> {
        match format {
            MediaFormat::Jpeg => Box::new(HeaderFooterStrategy::new(
                Some(vec![0xFF, 0xD9]),
                self.max_file_size,
            )),
            MediaFormat::Png => Box::new(HeaderFooterStrategy::new(
                Some(vec![0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82]),
                self.max_file_size,
            )),
            _ => Box::new(HeaderFooterStrategy::new(
                None,
                self.max_file_size,
            )),
        }
    }
}

impl Default for CarvingEngine {
    fn default() -> Self {
        Self::new(100 * 1024 * 1024) // 100MB default max
    }
}
