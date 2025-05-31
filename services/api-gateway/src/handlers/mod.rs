use axum::{extract::State, http::StatusCode, response::Json, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod auth;
pub mod users;
pub mod recipes;
pub mod meal_plans;
pub mod nutrition;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: crate::models::User,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
}
