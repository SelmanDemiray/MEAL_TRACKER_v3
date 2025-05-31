use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

use crate::AppState;

pub async fn get_profile(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Get profile endpoint" }))
}

pub async fn update_profile(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Update profile endpoint" }))
}

pub async fn get_preferences(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Get preferences endpoint" }))
}

pub async fn update_preferences(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Update preferences endpoint" }))
}
