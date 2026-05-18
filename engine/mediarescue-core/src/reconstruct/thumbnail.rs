use crate::error::FormatError;
use crate::types::MediaFormat;
use super::validator::IntegrityValidator;

pub struct ThumbnailGenerator {
    validator: IntegrityValidator,
    default_max_dim: u32,
}

impl ThumbnailGenerator {
    pub fn new(max_dim: u32) -> Self {
        Self {
            validator: IntegrityValidator::new(),
            default_max_dim: max_dim,
        }
    }

    pub fn generate(&self, format: &MediaFormat, data: &[u8]) -> Result<Vec<u8>, FormatError> {
        self.generate_with_size(format, data, self.default_max_dim)
    }

    pub fn generate_with_size(
        &self,
        format: &MediaFormat,
        data: &[u8],
        max_dim: u32,
    ) -> Result<Vec<u8>, FormatError> {
        let handler = self.validator.handler_for(format).ok_or_else(|| {
            FormatError::UnsupportedVariant(format!("{:?}", format))
        })?;

        handler.generate_thumbnail(data, max_dim)
    }
}

impl Default for ThumbnailGenerator {
    fn default() -> Self {
        Self::new(256)
    }
}
