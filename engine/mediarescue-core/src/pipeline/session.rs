use crate::types::{RecoveredFile, ScanConfig, ScanState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySession {
    pub id: Uuid,
    pub device_path: String,
    pub config: ScanConfig,
    pub state: ScanState,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub recovered_files: Vec<RecoveredFile>,
    pub output_dir: PathBuf,
}

impl RecoverySession {
    pub fn new(device_path: String, config: ScanConfig, output_dir: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            device_path,
            config,
            state: ScanState::Idle,
            started_at: Utc::now(),
            completed_at: None,
            recovered_files: Vec::new(),
            output_dir,
        }
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)
    }

    pub fn load(path: &std::path::Path) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
