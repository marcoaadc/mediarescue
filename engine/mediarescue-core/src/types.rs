use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub sector_size: u32,
    pub total_sectors: u64,
    pub device_type: DeviceType,
    pub removable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    HardDrive,
    UsbDrive,
    DiskImage,
    SsdDrive,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaFormat {
    Jpeg,
    Png,
    Mp4,
    Mov,
    Avi,
    Mkv,
    Cr2,
    Nef,
    Arw,
}

impl MediaFormat {
    pub fn extension(&self) -> &str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Mp4 => "mp4",
            Self::Mov => "mov",
            Self::Avi => "avi",
            Self::Mkv => "mkv",
            Self::Cr2 => "cr2",
            Self::Nef => "nef",
            Self::Arw => "arw",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Mp4 => "video/mp4",
            Self::Mov => "video/quicktime",
            Self::Avi => "video/x-msvideo",
            Self::Mkv => "video/x-matroska",
            Self::Cr2 | Self::Nef | Self::Arw => "image/x-raw",
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Jpeg | Self::Png | Self::Cr2 | Self::Nef | Self::Arw)
    }

    pub fn is_video(&self) -> bool {
        matches!(self, Self::Mp4 | Self::Mov | Self::Avi | Self::Mkv)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicBytes {
    pub format: MediaFormat,
    pub header: Vec<u8>,
    pub header_offset: u64,
    pub footer: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureMatch {
    pub format: MediaFormat,
    pub disk_offset: u64,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveredFile {
    pub id: Uuid,
    pub format: MediaFormat,
    pub disk_offset: u64,
    pub size_bytes: u64,
    pub recovery_score: f32,
    pub status: FileStatus,
    pub can_repair: bool,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    Discovered,
    Carving,
    Carved,
    Reconstructing,
    Reconstructed,
    Validating,
    Valid,
    Invalid,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub score: f32,
    pub issues: Vec<String>,
    pub can_repair: bool,
    pub suggested_repairs: Vec<RepairAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairAction {
    RebuildHeader,
    FixChunkCrc,
    ReconstructMoov,
    RebuildIndex,
    PadTruncatedData,
    RemoveCorruptedSegment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub formats: Vec<MediaFormat>,
    pub depth: ScanDepth,
    pub max_workers: u32,
    pub sector_retry_count: u32,
    pub sector_read_timeout_ms: u64,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            formats: vec![
                MediaFormat::Jpeg,
                MediaFormat::Png,
                MediaFormat::Mp4,
                MediaFormat::Mov,
                MediaFormat::Avi,
                MediaFormat::Mkv,
            ],
            depth: ScanDepth::Standard,
            max_workers: 4,
            sector_retry_count: 3,
            sector_read_timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanDepth {
    Quick,
    Standard,
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanState {
    Idle,
    Preparing,
    Scanning,
    Paused,
    Analyzing,
    Completed,
    Cancelled,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub state: ScanState,
    pub sectors_scanned: u64,
    pub total_sectors: u64,
    pub bad_sectors: u64,
    pub signatures_found: u32,
    pub speed_bytes_per_sec: u64,
    pub eta_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryEvent {
    ScanProgress {
        sectors_done: u64,
        sectors_total: u64,
        signatures_found: u32,
        speed_mbps: f64,
    },
    FileDiscovered {
        id: Uuid,
        format: MediaFormat,
        offset: u64,
        estimated_size: u64,
    },
    CarvingProgress {
        file_id: Uuid,
        bytes_carved: u64,
        bytes_total: u64,
    },
    FileRecovered {
        id: Uuid,
        score: f32,
    },
    RepairAttempt {
        file_id: Uuid,
        action: RepairAction,
        success: bool,
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
}
