use mediarescue_core::types::RecoveryEvent;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    ScanProgress {
        sectors_done: u64,
        sectors_total: u64,
        signatures_found: u32,
        speed_mbps: f64,
    },
    FileDiscovered {
        file_id: String,
        format: String,
        offset: u64,
        estimated_size: u64,
    },
    CarvingProgress {
        file_id: String,
        bytes_carved: u64,
        bytes_total: u64,
    },
    FileRecovered {
        file_id: String,
        score: f32,
    },
    ScanComplete {
        total_found: u32,
        total_recovered: u32,
        duration_secs: f64,
    },
    ScanError {
        message: String,
        recoverable: bool,
    },
    ScanPaused,
    ScanResumed,
    ScanCancelled,
    Ping,
}

impl From<RecoveryEvent> for WsEvent {
    fn from(event: RecoveryEvent) -> Self {
        match event {
            RecoveryEvent::ScanProgress {
                sectors_done,
                sectors_total,
                signatures_found,
                speed_mbps,
            } => WsEvent::ScanProgress {
                sectors_done,
                sectors_total,
                signatures_found,
                speed_mbps,
            },
            RecoveryEvent::FileDiscovered {
                id,
                format,
                offset,
                estimated_size,
            } => WsEvent::FileDiscovered {
                file_id: id.to_string(),
                format: format!("{:?}", format),
                offset,
                estimated_size,
            },
            RecoveryEvent::CarvingProgress {
                file_id,
                bytes_carved,
                bytes_total,
            } => WsEvent::CarvingProgress {
                file_id: file_id.to_string(),
                bytes_carved,
                bytes_total,
            },
            RecoveryEvent::FileRecovered { id, score } => WsEvent::FileRecovered {
                file_id: id.to_string(),
                score,
            },
            RecoveryEvent::ScanComplete {
                total_found,
                total_recovered,
                duration_secs,
            } => WsEvent::ScanComplete {
                total_found,
                total_recovered,
                duration_secs,
            },
            RecoveryEvent::ScanError {
                message,
                recoverable,
            } => WsEvent::ScanError {
                message,
                recoverable,
            },
            RecoveryEvent::ScanPaused => WsEvent::ScanPaused,
            RecoveryEvent::ScanResumed => WsEvent::ScanResumed,
            RecoveryEvent::ScanCancelled => WsEvent::ScanCancelled,
            _ => WsEvent::Ping,
        }
    }
}
