use crate::carver::engine::CarvingEngine;
use crate::device::reader::DeviceReader;
use crate::error::MrError;
use crate::reconstruct::repairer::Repairer;
use crate::reconstruct::validator::IntegrityValidator;
use crate::scanner::surface::SurfaceScanner;
use crate::types::{
    FileStatus, RecoveredFile, RecoveryEvent, ScanConfig, ScanState,
};
use super::session::RecoverySession;
use std::path::PathBuf;
use tokio::sync::broadcast;

pub struct RecoveryOrchestrator {
    config: ScanConfig,
    output_dir: PathBuf,
}

impl RecoveryOrchestrator {
    pub fn new(config: ScanConfig, output_dir: PathBuf) -> Self {
        Self { config, output_dir }
    }

    pub async fn run(
        &self,
        reader: Box<dyn DeviceReader>,
        event_tx: broadcast::Sender<RecoveryEvent>,
    ) -> Result<RecoverySession, MrError> {
        let mut session = RecoverySession::new(
            "device".to_string(),
            self.config.clone(),
            self.output_dir.clone(),
        );

        // Phase 1: Surface Scan
        session.state = ScanState::Scanning;
        let mut scanner = SurfaceScanner::new(self.config.clone(), reader.total_sectors());
        let signatures = scanner.scan(reader.as_ref(), &event_tx).await?;

        // Phase 2: File Carving
        let carver = CarvingEngine::default();
        let validator = IntegrityValidator::new();
        let repairer = Repairer::new();
        let mut carved_files: Vec<(RecoveredFile, Vec<u8>)> = Vec::new();

        for sig in signatures.iter() {
            match carver.carve_file(sig, reader.as_ref()) {
                Ok((file, data)) => {
                    let _ = event_tx.send(RecoveryEvent::CarvingProgress {
                        file_id: file.id,
                        bytes_carved: data.len() as u64,
                        bytes_total: data.len() as u64,
                    });
                    carved_files.push((file, data));
                }
                Err(e) => {
                    tracing::warn!("Failed to carve file at offset {}: {}", sig.disk_offset, e);
                }
            }
        }

        // Phase 3: Validate and Repair
        for (file, data) in &mut carved_files {
            let validation = validator.validate(&file.format, data);
            file.recovery_score = validation.score;
            file.can_repair = validation.can_repair;

            if validation.is_valid {
                file.status = FileStatus::Valid;
            } else if validation.can_repair {
                file.status = FileStatus::Reconstructing;
                match repairer.repair(&file.format, data) {
                    Ok(repaired) => {
                        *data = repaired;
                        let re_validation = validator.validate(&file.format, data);
                        file.recovery_score = re_validation.score;
                        file.status = if re_validation.is_valid {
                            FileStatus::Valid
                        } else {
                            FileStatus::Invalid
                        };
                    }
                    Err(_) => {
                        file.status = FileStatus::Failed;
                    }
                }
            } else {
                file.status = FileStatus::Invalid;
            }

            let _ = event_tx.send(RecoveryEvent::FileRecovered {
                id: file.id,
                score: file.recovery_score,
            });
        }

        // Phase 4: Save recovered files
        std::fs::create_dir_all(&self.output_dir)?;

        for (file, data) in &carved_files {
            if matches!(file.status, FileStatus::Valid | FileStatus::Invalid) {
                let filename = format!(
                    "recovered_{}_{}.{}",
                    file.format.extension(),
                    &file.id.to_string()[..8],
                    file.format.extension()
                );
                let path = self.output_dir.join(&filename);
                std::fs::write(&path, data)?;
            }
        }

        session.recovered_files = carved_files.into_iter().map(|(f, _)| f).collect();
        session.state = ScanState::Completed;
        session.completed_at = Some(chrono::Utc::now());

        let total_recovered = session
            .recovered_files
            .iter()
            .filter(|f| f.status == FileStatus::Valid)
            .count() as u32;

        let _ = event_tx.send(RecoveryEvent::ScanComplete {
            total_found: session.recovered_files.len() as u32,
            total_recovered,
            duration_secs: 0.0, // TODO: calculate from session timestamps
        });

        Ok(session)
    }
}
