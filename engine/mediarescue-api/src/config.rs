use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub output_dir: PathBuf,
    pub max_file_size: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("MR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("MR_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3001),
            output_dir: std::env::var("MR_OUTPUT_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./recovered")),
            max_file_size: std::env::var("MR_MAX_FILE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100 * 1024 * 1024),
        }
    }
}
