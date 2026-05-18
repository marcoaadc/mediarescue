use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod config;
mod state;
mod routes;
mod ws;
mod middleware;
mod dto;

use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mediarescue_api=debug,tower_http=debug".into()),
        )
        .init();

    let config = config::Config::from_env();
    let state = Arc::new(AppState::new(config.clone()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(routes::health::health_check))
        .route("/api/devices", get(routes::devices::list_devices))
        .route("/api/devices/{id}", get(routes::devices::get_device))
        .route("/api/scans", post(routes::scans::create_scan))
        .route("/api/scans/{id}", get(routes::scans::get_scan))
        .route("/api/scans/{id}/pause", post(routes::scans::pause_scan))
        .route("/api/scans/{id}/resume", post(routes::scans::resume_scan))
        .route("/api/scans/{id}/cancel", post(routes::scans::cancel_scan))
        .route("/api/scans/{id}/files", get(routes::files::list_scan_files))
        .route("/api/files/{id}", get(routes::files::get_file))
        .route("/api/files/{id}/thumbnail", get(routes::files::get_thumbnail))
        .route("/api/files/{id}/download", get(routes::files::download_file))
        .route("/api/files/{id}/repair", post(routes::recovery::repair_file))
        .route("/api/recovery/export", post(routes::recovery::export_files))
        .route("/api/settings", get(routes::settings::get_settings))
        .route("/api/settings", put(routes::settings::update_settings))
        .route("/ws/scan/{scan_id}", get(ws::handler::ws_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("MediaRescue API starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
