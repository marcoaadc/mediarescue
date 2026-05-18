use crate::device::reader::DeviceReader;
use crate::error::FormatError;
use crate::types::{MagicBytes, ValidationResult};

pub trait FormatHandler: Send + Sync {
    fn format_name(&self) -> &str;

    fn signatures(&self) -> &[MagicBytes];

    fn estimate_file_size(
        &self,
        header: &[u8],
        reader: &dyn DeviceReader,
        offset: u64,
    ) -> Result<u64, FormatError>;

    fn validate(&self, data: &[u8]) -> ValidationResult;

    fn repair(&self, data: &[u8]) -> Result<Vec<u8>, FormatError>;

    fn generate_thumbnail(&self, data: &[u8], max_dim: u32) -> Result<Vec<u8>, FormatError>;
}
