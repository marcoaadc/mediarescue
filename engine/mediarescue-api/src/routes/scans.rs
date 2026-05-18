use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::state::{AppState, ScanSession};
use crate::dto::request::CreateScanRequest;
use crate::dto::response::ApiError;
use mediarescue_core::types::RecoveryEvent;
use serde_json::{json, Value};
use tokio::sync::broadcast;
use uuid::Uuid;

pub async fn create_scan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateScanRequest>,
) -> Result<Json<Value>, Json<ApiError>> {
    let scan_id = Uuid::new_v4();
    let (event_tx, _) = broadcast::channel::<RecoveryEvent>(256);

    let cancel = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let pause = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let session = ScanSession {
        id: scan_id,
        event_tx: event_tx.clone(),
        cancel_handle: Arc::clone(&cancel),
        pause_handle: Arc::clone(&pause),
    };

    state.scans.write().await.insert(scan_id, session);

    // TODO: Spawn the actual scan task using the orchestrator
    // For now, return the scan ID immediately

    Ok(Json(json!({
        "scan_id": scan_id.to_string(),
        "status": "preparing"
    })))
}

pub async fn get_scan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, Json<ApiError>> {
    let scans = state.scans.read().await;
    if scans.contains_key(&id) {
        Ok(Json(json!({
            "scan_id": id.to_string(),
            "status": "active"
        })))
    } else {
        Err(Json(ApiError {
            error: "Scan not found".to_string(),
            code: "NOT_FOUND".to_string(),
        }))
    }
}

pub async fn pause_scan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, Json<ApiError>> {
    let scans = state.scans.read().await;
    if let Some(session) = scans.get(&id) {
        session.pause_handle.store(true, Ordering::Relaxed);
        Ok(Json(json!({
            "scan_id": id.to_string(),
            "status": "paused"
        })))
    } else {
        Err(Json(ApiError {
            error: "Scan not found".to_string(),
            code: "NOT_FOUND".to_string(),
        }))
    }
}

pub async fn resume_scan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, Json<ApiError>> {
    let scans = state.scans.read().await;
    if let Some(session) = scans.get(&id) {
        session.pause_handle.store(false, Ordering::Relaxed);
        Ok(Json(json!({
            "scan_id": id.to_string(),
            "status": "scanning"
        })))
    } else {
        Err(Json(ApiError {
            error: "Scan not found".to_string(),
            code: "NOT_FOUND".to_string(),
        }))
    }
}

pub async fn cancel_scan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, Json<ApiError>> {
    let scans = state.scans.read().await;
    if let Some(session) = scans.get(&id) {
        session.cancel_handle.store(true, Ordering::Relaxed);
        Ok(Json(json!({
            "scan_id": id.to_string(),
            "status": "cancelled"
        })))
    } else {
        Err(Json(ApiError {
            error: "Scan not found".to_string(),
            code: "NOT_FOUND".to_string(),
        }))
    }
}
