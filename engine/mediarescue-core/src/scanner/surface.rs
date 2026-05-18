use crate::device::reader::DeviceReader;
use crate::error::ScanError;
use crate::types::{MediaFormat, RecoveryEvent, ScanConfig, ScanState, SignatureMatch};
use super::sector_map::{SectorMap, SectorStatus};
use super::signatures::SignatureDB;
use tokio::sync::broadcast;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub struct SurfaceScanner {
    config: ScanConfig,
    signature_db: SignatureDB,
    sector_map: SectorMap,
    state: ScanState,
    signatures_found: Vec<SignatureMatch>,
    cancelled: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
}

impl SurfaceScanner {
    pub fn new(config: ScanConfig, total_sectors: u64) -> Self {
        let signature_db = SignatureDB::with_formats(&config.formats);
        Self {
            config,
            signature_db,
            sector_map: SectorMap::new(total_sectors),
            state: ScanState::Idle,
            signatures_found: Vec::new(),
            cancelled: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancelled)
    }

    pub fn pause_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.paused)
    }

    pub fn state(&self) -> ScanState {
        self.state
    }

    pub fn sector_map(&self) -> &SectorMap {
        &self.sector_map
    }

    pub fn signatures_found(&self) -> &[SignatureMatch] {
        &self.signatures_found
    }

    pub async fn scan(
        &mut self,
        reader: &dyn DeviceReader,
        event_tx: &broadcast::Sender<RecoveryEvent>,
    ) -> Result<Vec<SignatureMatch>, ScanError> {
        self.state = ScanState::Preparing;

        if !reader.is_connected() {
            self.state = ScanState::Error;
            return Err(ScanError::DeviceNotFound("device not connected".to_string()));
        }

        self.state = ScanState::Scanning;
        let total_sectors = reader.total_sectors();
        let sector_size = reader.sector_size();
        let sectors_per_read = self.sectors_per_read();
        let start_time = Instant::now();
        let mut sectors_scanned: u64 = 0;

        let mut sector = 0u64;
        while sector < total_sectors {
            if self.cancelled.load(Ordering::Relaxed) {
                self.state = ScanState::Cancelled;
                let _ = event_tx.send(RecoveryEvent::ScanCancelled);
                return Err(ScanError::Cancelled);
            }

            while self.paused.load(Ordering::Relaxed) {
                self.state = ScanState::Paused;
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if self.cancelled.load(Ordering::Relaxed) {
                    self.state = ScanState::Cancelled;
                    let _ = event_tx.send(RecoveryEvent::ScanCancelled);
                    return Err(ScanError::Cancelled);
                }
            }
            self.state = ScanState::Scanning;

            if !reader.is_connected() {
                self.state = ScanState::Error;
                return Err(ScanError::DeviceDisconnected { sector });
            }

            let count = sectors_per_read.min(total_sectors - sector);

            match reader.read_sectors(sector, count) {
                Ok(buffer) => {
                    let base_offset = sector * sector_size as u64;
                    let matches = self.signature_db.scan_buffer(&buffer, base_offset);

                    for m in &matches {
                        self.sector_map.set(
                            m.disk_offset / sector_size as u64,
                            SectorStatus::SignatureFound,
                        );
                    }
                    self.signatures_found.extend(matches);

                    for s in sector..sector + count {
                        if self.sector_map.get(s) == SectorStatus::Unread {
                            self.sector_map.set(s, SectorStatus::Good);
                        }
                    }
                    sectors_scanned += count;
                }
                Err(_) => {
                    for s in sector..sector + count {
                        self.sector_map.set(s, SectorStatus::Bad);
                    }
                    sectors_scanned += count;
                }
            }

            let elapsed = start_time.elapsed().as_secs_f64();
            let speed_mbps = if elapsed > 0.0 {
                (sectors_scanned as f64 * sector_size as f64) / (elapsed * 1_000_000.0)
            } else {
                0.0
            };

            if sectors_scanned % (sectors_per_read * 10) == 0 || sector + count >= total_sectors {
                let _ = event_tx.send(RecoveryEvent::ScanProgress {
                    sectors_done: sectors_scanned,
                    sectors_total: total_sectors,
                    signatures_found: self.signatures_found.len() as u32,
                    speed_mbps,
                });
            }

            sector += count;
        }

        self.state = ScanState::Analyzing;

        self.deduplicate_signatures();

        self.state = ScanState::Completed;
        let elapsed = start_time.elapsed().as_secs_f64();
        let _ = event_tx.send(RecoveryEvent::ScanComplete {
            total_found: self.signatures_found.len() as u32,
            total_recovered: 0,
            duration_secs: elapsed,
        });

        Ok(self.signatures_found.clone())
    }

    fn sectors_per_read(&self) -> u64 {
        match self.config.depth {
            crate::types::ScanDepth::Quick => 128,
            crate::types::ScanDepth::Standard => 64,
            crate::types::ScanDepth::Deep => 1,
        }
    }

    fn deduplicate_signatures(&mut self) {
        self.signatures_found.sort_by_key(|s| s.disk_offset);
        self.signatures_found.dedup_by(|a, b| {
            a.format == b.format && a.disk_offset == b.disk_offset
        });
    }
}
