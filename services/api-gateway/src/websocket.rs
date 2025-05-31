use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

use crate::AppState;

pub struct WebSocketManager {
    // TODO: Implement WebSocket management
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, _state: AppState) {
    info!("WebSocket connection established");

    // Create a broadcast channel for this connection
    let (tx, mut rx) = broadcast::channel(100);

    // Send welcome message
    if let Err(e) = socket
        .send(axum::extract::ws::Message::Text(
            serde_json::json!({
                "type": "connection",
                "message": "Connected to Meal Prep Pro API Gateway",
                "timestamp": chrono::Utc::now()
            })
            .to_string(),
        ))
        .await
    {
        error!("Failed to send welcome message: {}", e);
        return;
    }

    // Handle incoming messages
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                info!("Received text message: {}", text);

                // Echo the message back for now
                let response = serde_json::json!({
                    "type": "echo",
                    "original": text,
                    "timestamp": chrono::Utc::now()
                });

                if let Err(e) = socket
                    .send(axum::extract::ws::Message::Text(response.to_string()))
                    .await
                {
                    error!("Failed to send echo message: {}", e);
                    break;
                }
            }
            Ok(axum::extract::ws::Message::Binary(_)) => {
                info!("Received binary message");
            }
            Ok(axum::extract::ws::Message::Close(_)) => {
                info!("WebSocket connection closed");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    info!("WebSocket connection ended");
}
