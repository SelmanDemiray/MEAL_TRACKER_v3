use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

use crate::AppState;

pub async fn analyze_nutrition(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Nutrition analysis endpoint" }))
}

pub async fn get_goals(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "goals": [] }))
}

pub async fn update_goals(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Goals updated" }))
}
