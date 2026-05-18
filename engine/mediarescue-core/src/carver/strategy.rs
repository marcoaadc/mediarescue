use crate::device::reader::DeviceReader;
use crate::error::CarveError;
use crate::types::SignatureMatch;

pub trait CarvingStrategy: Send + Sync {
    fn carve(
        &self,
        signature: &SignatureMatch,
        reader: &dyn DeviceReader,
    ) -> Result<Vec<u8>, CarveError>;
}

pub struct HeaderFooterStrategy {
    footer: Option<Vec<u8>>,
    max_size: u64,
}

impl HeaderFooterStrategy {
    pub fn new(footer: Option<Vec<u8>>, max_size: u64) -> Self {
        Self { footer, max_size }
    }
}

impl CarvingStrategy for HeaderFooterStrategy {
    fn carve(
        &self,
        signature: &SignatureMatch,
        reader: &dyn DeviceReader,
    ) -> Result<Vec<u8>, CarveError> {
        let sector_size = reader.sector_size() as u64;
        let start_sector = signature.disk_offset / sector_size;
        let max_sectors = self.max_size / sector_size;
        let sectors_available = reader.total_sectors() - start_sector;
        let sectors_to_read = max_sectors.min(sectors_available);

        let data = reader.read_sectors(start_sector, sectors_to_read).map_err(|_| {
            CarveError::NoSignature(signature.disk_offset)
        })?;

        if let Some(ref footer) = self.footer {
            if let Some(pos) = data.windows(footer.len()).position(|w| w == footer.as_slice()) {
                let end = pos + footer.len();
                return Ok(data[..end].to_vec());
            }
        }

        // If no footer found, return data up to max_size
        Ok(data)
    }
}

pub struct StructureAwareStrategy;

impl CarvingStrategy for StructureAwareStrategy {
    fn carve(
        &self,
        signature: &SignatureMatch,
        reader: &dyn DeviceReader,
    ) -> Result<Vec<u8>, CarveError> {
        // TODO: Use format-specific knowledge to determine exact boundaries
        let sector_size = reader.sector_size() as u64;
        let start_sector = signature.disk_offset / sector_size;
        let default_sectors = (10 * 1024 * 1024) / sector_size; // 10MB default
        let sectors = default_sectors.min(reader.total_sectors() - start_sector);

        reader.read_sectors(start_sector, sectors).map_err(|_| {
            CarveError::NoSignature(signature.disk_offset)
        })
    }
}
