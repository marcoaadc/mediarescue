use thiserror::Error;

#[derive(Error, Debug)]
pub enum MrError {
    #[error("Scan error: {0}")]
    Scan(#[from] ScanError),

    #[error("Carve error: {0}")]
    Carve(#[from] CarveError),

    #[error("Format error: {0}")]
    Format(#[from] FormatError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Device error: {0}")]
    Device(#[from] DeviceError),

    #[error("Pipeline error: {0}")]
    Pipeline(String),
}

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Device disconnected during scan at sector {sector}")]
    DeviceDisconnected { sector: u64 },

    #[error("Scan cancelled by user")]
    Cancelled,

    #[error("Bad sector at offset {offset}: {message}")]
    BadSector { offset: u64, message: String },
}

#[derive(Error, Debug)]
pub enum CarveError {
    #[error("No valid signature found at offset {0}")]
    NoSignature(u64),

    #[error("File truncated at offset {offset}, expected {expected} bytes")]
    Truncated { offset: u64, expected: u64 },

    #[error("Fragment assembly failed: {0}")]
    FragmentAssembly(String),
}

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid {format} header at offset {offset}")]
    InvalidHeader { format: String, offset: u64 },

    #[error("Corrupted {format} structure: {message}")]
    CorruptedStructure { format: String, message: String },

    #[error("Repair failed for {format}: {reason}")]
    RepairFailed { format: String, reason: String },

    #[error("Unsupported format variant: {0}")]
    UnsupportedVariant(String),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Cannot open device: {path} - {reason}")]
    OpenFailed { path: String, reason: String },

    #[error("Read failed at sector {sector}: {reason}")]
    ReadFailed { sector: u64, reason: String },

    #[error("Device removed")]
    Removed,

    #[error("Insufficient permissions to access {0}")]
    PermissionDenied(String),
}

pub type Result<T> = std::result::Result<T, MrError>;
