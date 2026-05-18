use crate::error::FormatError;
use crate::types::MediaFormat;
use super::validator::IntegrityValidator;

pub struct Repairer {
    validator: IntegrityValidator,
}

impl Repairer {
    pub fn new() -> Self {
        Self {
            validator: IntegrityValidator::new(),
        }
    }

    pub fn repair(&self, format: &MediaFormat, data: &[u8]) -> Result<Vec<u8>, FormatError> {
        let handler = self.validator.handler_for(format).ok_or_else(|| {
            FormatError::UnsupportedVariant(format!("{:?}", format))
        })?;

        handler.repair(data)
    }

    pub fn can_repair(&self, format: &MediaFormat, data: &[u8]) -> bool {
        let result = self.validator.validate(format, data);
        result.can_repair
    }
}

impl Default for Repairer {
    fn default() -> Self {
        Self::new()
    }
}
