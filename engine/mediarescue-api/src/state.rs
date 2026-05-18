use crate::config::Config;
use mediarescue_core::types::{RecoveryEvent, ScanConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub struct ScanSession {
    pub id: Uuid,
    pub event_tx: broadcast::Sender<RecoveryEvent>,
    pub cancel_handle: Arc<std::sync::atomic::AtomicBool>,
    pub pause_handle: Arc<std::sync::atomic::AtomicBool>,
}

pub struct AppState {
    pub config: Config,
    pub scans: RwLock<HashMap<Uuid, ScanSession>>,
    pub settings: RwLock<ScanConfig>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            scans: RwLock::new(HashMap::new()),
            settings: RwLock::new(ScanConfig::default()),
        }
    }
}
