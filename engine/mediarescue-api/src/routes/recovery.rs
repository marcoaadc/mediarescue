use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::state::AppState;
use crate::dto::request::RepairRequest;
use crate::dto::response::ApiError;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn repair_file(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(_req): Json<RepairRequest>,
) -> Result<Json<Value>, Json<ApiError>> {
    // TODO: Dispatch repair to the repairer
    Ok(Json(json!({
        "file_id": id.to_string(),
        "status": "repairing"
    })))
}

pub async fn export_files(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<Value>,
) -> Result<Json<Value>, Json<ApiError>> {
    // TODO: Batch export recovered files
    let export_id = Uuid::new_v4();
    Ok(Json(json!({
        "export_id": export_id.to_string(),
        "status": "exporting"
    })))
}
