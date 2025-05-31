use axum::{
    extract::{State, Path},
    response::{IntoResponse, Json},
};

use crate::AppState;

pub async fn list_recipes(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "recipes": [] }))
}

pub async fn create_recipe(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Recipe created" }))
}

pub async fn get_recipe(State(_state): State<AppState>, Path(_id): Path<String>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Get recipe endpoint" }))
}

pub async fn update_recipe(State(_state): State<AppState>, Path(_id): Path<String>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Update recipe endpoint" }))
}

pub async fn delete_recipe(State(_state): State<AppState>, Path(_id): Path<String>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Delete recipe endpoint" }))
}
