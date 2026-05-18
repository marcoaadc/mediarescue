use crate::formats::traits::FormatHandler;
use crate::types::{MediaFormat, ValidationResult};
use crate::formats::jpeg::JpegHandler;
use crate::formats::png::PngHandler;
use crate::formats::mp4::Mp4Handler;
use crate::formats::avi::AviHandler;
use crate::formats::mkv::MkvHandler;
use crate::formats::raw::RawHandler;

pub struct IntegrityValidator {
    handlers: Vec<(MediaFormat, Box<dyn FormatHandler>)>,
}

impl IntegrityValidator {
    pub fn new() -> Self {
        Self {
            handlers: vec![
                (MediaFormat::Jpeg, Box::new(JpegHandler::new())),
                (MediaFormat::Png, Box::new(PngHandler::new())),
                (MediaFormat::Mp4, Box::new(Mp4Handler::new())),
                (MediaFormat::Mov, Box::new(Mp4Handler::new())),
                (MediaFormat::Avi, Box::new(AviHandler::new())),
                (MediaFormat::Mkv, Box::new(MkvHandler::new())),
                (MediaFormat::Cr2, Box::new(RawHandler::new())),
                (MediaFormat::Nef, Box::new(RawHandler::new())),
                (MediaFormat::Arw, Box::new(RawHandler::new())),
            ],
        }
    }

    pub fn validate(&self, format: &MediaFormat, data: &[u8]) -> ValidationResult {
        for (fmt, handler) in &self.handlers {
            if fmt == format {
                return handler.validate(data);
            }
        }

        ValidationResult {
            is_valid: false,
            score: 0.0,
            issues: vec![format!("No handler for format {:?}", format)],
            can_repair: false,
            suggested_repairs: vec![],
        }
    }

    pub fn handler_for(&self, format: &MediaFormat) -> Option<&dyn FormatHandler> {
        self.handlers
            .iter()
            .find(|(fmt, _)| fmt == format)
            .map(|(_, handler)| handler.as_ref())
    }
}

impl Default for IntegrityValidator {
    fn default() -> Self {
        Self::new()
    }
}
