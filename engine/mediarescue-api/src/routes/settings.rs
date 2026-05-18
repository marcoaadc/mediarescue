use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;
use crate::state::AppState;
use mediarescue_core::types::ScanConfig;

pub async fn get_settings(
    State(state): State<Arc<AppState>>,
) -> Json<ScanConfig> {
    let settings = state.settings.read().await;
    Json(settings.clone())
}

pub async fn update_settings(
    State(state): State<Arc<AppState>>,
    Json(new_settings): Json<ScanConfig>,
) -> Json<ScanConfig> {
    let mut settings = state.settings.write().await;
    *settings = new_settings.clone();
    Json(new_settings)
}
