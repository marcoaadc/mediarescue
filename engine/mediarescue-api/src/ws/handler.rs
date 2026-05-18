use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::state::AppState;
use crate::dto::response::ApiError;
use super::events::WsEvent;
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(scan_id): Path<Uuid>,
) -> Result<impl IntoResponse, Json<ApiError>> {
    let scans = state.scans.read().await;
    let session = scans.get(&scan_id).ok_or_else(|| {
        Json(ApiError {
            error: "Scan not found".to_string(),
            code: "NOT_FOUND".to_string(),
        })
    })?;

    let mut rx = session.event_tx.subscribe();
    drop(scans);

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, rx)))
}

async fn handle_socket(
    mut socket: WebSocket,
    mut rx: tokio::sync::broadcast::Receiver<mediarescue_core::types::RecoveryEvent>,
) {
    let mut ping_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Ok(recovery_event) => {
                        let ws_event: WsEvent = recovery_event.into();
                        let json = serde_json::to_string(&ws_event).unwrap_or_default();
                        if socket.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("WebSocket client lagged by {} events", n);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            _ = ping_interval.tick() => {
                let ping = serde_json::to_string(&WsEvent::Ping).unwrap_or_default();
                if socket.send(Message::Text(ping.into())).await.is_err() {
                    break;
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Handle client commands (pause, resume, cancel)
                        tracing::debug!("Client command: {}", text);
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}
