use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::state::AppState;
use crate::dto::response::ApiError;
use mediarescue_core::device::enumerator::{DeviceEnumerator, SystemDeviceEnumerator};
use mediarescue_core::types::DeviceInfo;

pub async fn list_devices(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeviceInfo>>, Json<ApiError>> {
    let enumerator = SystemDeviceEnumerator::new();
    match enumerator.list_devices() {
        Ok(devices) => Ok(Json(devices)),
        Err(e) => Err(Json(ApiError {
            error: e.to_string(),
            code: "DEVICE_ENUM_FAILED".to_string(),
        })),
    }
}

pub async fn get_device(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeviceInfo>, Json<ApiError>> {
    let enumerator = SystemDeviceEnumerator::new();
    match enumerator.list_devices() {
        Ok(devices) => {
            devices
                .into_iter()
                .find(|d| d.id == id)
                .map(Json)
                .ok_or_else(|| {
                    Json(ApiError {
                        error: format!("Device {} not found", id),
                        code: "NOT_FOUND".to_string(),
                    })
                })
        }
        Err(e) => Err(Json(ApiError {
            error: e.to_string(),
            code: "DEVICE_ENUM_FAILED".to_string(),
        })),
    }
}
