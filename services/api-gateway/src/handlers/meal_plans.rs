use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

use crate::AppState;

pub async fn list_meal_plans(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "meal_plans": [] }))
}

pub async fn create_meal_plan(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Meal plan created" }))
}
