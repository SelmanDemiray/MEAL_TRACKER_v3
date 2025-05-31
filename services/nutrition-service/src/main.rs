use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;

mod analytics_engine;
mod trend_analyzer;
mod predictive_model;
mod models;
mod database;
mod nutrition_analyzer;
mod recommendation_engine;
mod ai_engine;

use analytics_engine::AnalyticsEngine;
use trend_analyzer::TrendAnalyzer;
use predictive_model::PredictiveModel;
use nutrition_analyzer::NutritionalAnalyzer;
use recommendation_engine::RecommendationEngine;
use ai_engine::AIEngine;
use models::*;

#[derive(Clone)]
pub struct AppState {
    pub analytics_engine: Arc<AnalyticsEngine>,
    pub trend_analyzer: Arc<TrendAnalyzer>,
    pub predictive_model: Arc<PredictiveModel>,
    pub nutrition_analyzer: Arc<NutritionalAnalyzer>,
    pub recommendation_engine: Arc<RecommendationEngine>,
    pub ai_engine: Arc<AIEngine>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://mealprep:mealprep_secure_2024@postgres:5432/mealprep".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://redis:6379".to_string());

    let db = sqlx::PgPool::connect(&database_url).await?;
    let redis = redis::Client::open(redis_url)?;

    let analytics_engine = Arc::new(AnalyticsEngine::new());
    let trend_analyzer = Arc::new(TrendAnalyzer::new());
    let predictive_model = Arc::new(PredictiveModel::new());
    let nutrition_analyzer = Arc::new(NutritionalAnalyzer::new(ai_engine.clone()));
    let recommendation_engine = Arc::new(RecommendationEngine::new());
    let ai_engine = Arc::new(AIEngine::new());

    let state = AppState {
        analytics_engine,
        trend_analyzer,
        predictive_model,
        nutrition_analyzer,
        recommendation_engine,
        ai_engine,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/nutrition/analyze/meal", post(analyze_meal))
        .route("/nutrition/analyze/daily", post(analyze_daily_nutrition))
        .route("/nutrition/recommendations/meals", post(get_meal_recommendations))
        .route("/nutrition/goals/calculate", post(calculate_nutrition_goals))
        .route("/nutrition/goals/track", post(track_nutrition_progress))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    println!("🥗 Nutrition Service running on http://0.0.0.0:8081");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "Nutrition Service is healthy! 🥗"
}

#[derive(Deserialize)]
struct MealAnalysisRequest {
    ingredients: Vec<String>,
    portion_size: f64,
    cooking_method: Option<String>,
    user_id: String,
}

async fn analyze_meal(
    State(_state): State<AppState>,
    Json(_request): Json<MealAnalysisRequest>,
) -> Result<Json<MealAnalysisResponse>, StatusCode> {
    // Mock response
    let response = MealAnalysisResponse {
        nutrition: BasicNutrition {
            calories: 450.0,
            protein_g: 32.0,
            carbohydrates_g: 45.0,
            fat_g: 18.0,
            fiber_g: 8.0,
            sodium_mg: 680.0,
            sugar_g: 12.0,
            cholesterol_mg: 0.0,
        },
        environmental_impact: EnvironmentalImpact {
            carbon_footprint_kg: 2.1,
            water_usage_liters: 120.0,
            sustainability_score: 0.8,
        },
        health_score: 0.85,
        recommendations: vec![
            "Great protein content!".to_string(),
            "Consider adding more vegetables".to_string(),
        ],
    };
    
    Ok(Json(response))
}

async fn analyze_daily_nutrition(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "daily_total": {
            "calories": 2150,
            "protein_g": 98,
            "carbohydrates_g": 245,
            "fat_g": 85
        },
        "goal_progress": {
            "calories": 0.96,
            "protein": 0.82,
            "carbs": 0.88,
            "fat": 1.21
        }
    });
    
    Ok(Json(response))
}

async fn get_meal_recommendations(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "recommendations": [
            {
                "meal_id": "meal_001",
                "name": "Grilled Salmon Bowl",
                "match_score": 0.92,
                "prep_time": 25
            }
        ]
    });
    
    Ok(Json(response))
}

async fn calculate_nutrition_goals(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "goals": {
            "daily_calories": 2200,
            "daily_protein_g": 120,
            "daily_carbs_g": 275,
            "daily_fat_g": 73
        }
    });
    
    Ok(Json(response))
}

async fn track_nutrition_progress(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "progress": {
            "calories_remaining": 450,
            "protein_progress": 0.75,
            "overall_score": 0.82
        }
    });
    
    Ok(Json(response))
}
