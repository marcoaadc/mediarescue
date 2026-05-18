use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::state::AppState;
use crate::dto::response::ApiError;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn list_scan_files(
    State(_state): State<Arc<AppState>>,
    Path(scan_id): Path<Uuid>,
) -> Result<Json<Value>, Json<ApiError>> {
    // TODO: Return files from the scan session
    Ok(Json(json!({
        "files": [],
        "total": 0,
        "scan_id": scan_id.to_string()
    })))
}

pub async fn get_file(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, Json<ApiError>> {
    // TODO: Look up file by ID
    Err(Json(ApiError {
        error: format!("File {} not found", id),
        code: "NOT_FOUND".to_string(),
    }))
}

pub async fn get_thumbnail(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Vec<u8>, Json<ApiError>> {
    // TODO: Generate and return thumbnail
    Err(Json(ApiError {
        error: "Thumbnail not available".to_string(),
        code: "NOT_FOUND".to_string(),
    }))
}

pub async fn download_file(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Vec<u8>, Json<ApiError>> {
    // TODO: Return the recovered file
    Err(Json(ApiError {
        error: "File not available".to_string(),
        code: "NOT_FOUND".to_string(),
    }))
}
