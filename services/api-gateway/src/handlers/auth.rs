use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, models};
use super::{ErrorResponse};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: serde_json::Value,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // TODO: Implement actual registration logic
    let user = models::User {
        id: Uuid::new_v4(),
        username: payload.username,
        email: payload.email,
        password_hash: "hashed_password".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        is_active: true,
        role: "user".to_string(),
        profile: None,
        preferences: None,
    };

    let token = "mock_jwt_token".to_string();

    Json(AuthResponse { token, user })
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // TODO: Implement actual login logic
    let user = models::User {
        id: Uuid::new_v4(),
        username: payload.username,
        email: "user@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        is_active: true,
        role: "user".to_string(),
        profile: None,
        preferences: None,
    };

    let token = "mock_jwt_token".to_string();

    Json(AuthResponse { token, user })
}

pub async fn logout(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Logged out successfully" }))
}

pub async fn get_current_user(State(_state): State<AppState>) -> impl IntoResponse {
    // TODO: Implement actual current user logic
    Json(serde_json::json!({ "message": "Current user endpoint" }))
}
