use mediarescue_core::types::{MediaFormat, ScanDepth};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateScanRequest {
    pub device_id: String,
    pub formats: Option<Vec<MediaFormat>>,
    pub depth: Option<ScanDepth>,
}

#[derive(Debug, Deserialize)]
pub struct RepairRequest {
    pub actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub file_ids: Vec<String>,
    pub output_dir: String,
}
