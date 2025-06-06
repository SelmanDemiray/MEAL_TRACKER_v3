use axum::extract::ws::{Message, WebSocket};
use crate::AppState;

pub struct WebSocketManager {
    // TODO: Implement WebSocket management
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn handle_socket(_socket: WebSocket, _state: AppState) {
    // TODO: Implement WebSocket handler
}
