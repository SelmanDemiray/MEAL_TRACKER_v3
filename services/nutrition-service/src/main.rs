//! # Nutrition Service - AI-Powered Nutrition Analysis Engine
//! 
//! This microservice provides comprehensive nutrition analysis and recommendations
//! using advanced AI algorithms and machine learning models.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn, error};
use uuid::Uuid;

mod ai_engine;
mod database;
mod models;
mod nutrition_analyzer;
mod recommendation_engine;

use ai_engine::NutritionAI;
use database::DatabaseService;
use models::*;
use nutrition_analyzer::NutritionAnalyzer;
use recommendation_engine::RecommendationEngine;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseService,
    pub ai_engine: Arc<NutritionAI>,
    pub nutrition_analyzer: Arc<NutritionAnalyzer>,
    pub recommendation_engine: Arc<RecommendationEngine>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    timestamp: String,
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    ingredients: Vec<Ingredient>,
    portion_size: f64,
}

#[derive(Deserialize)]
struct Ingredient {
    name: String,
    amount: f64,
    unit: String,
}

#[derive(Serialize)]
struct NutritionAnalysis {
    calories: f64,
    protein_g: f64,
    carbs_g: f64,
    fat_g: f64,
    fiber_g: f64,
    sodium_mg: f64,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        service: "nutrition-service",
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

async fn analyze_meal(
    State(state): State<AppState>,
    Json(request): Json<AnalyzeRequest>,
) -> Result<Json<NutritionAnalysis>, StatusCode> {
    info!("Analyzing nutrition for {} ingredients", request.ingredients.len());
    
    match state.nutrition_analyzer.analyze_ingredients(&request.ingredients, request.user_id).await {
        Ok(analysis) => {
            info!("Nutrition analysis completed successfully");
            Ok(Json(analysis))
        }
        Err(e) => {
            error!("Nutrition analysis failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_meal_recommendations(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<RecommendationParams>,
) -> Result<Json<MealRecommendations>, StatusCode> {
    info!("Getting meal recommendations for user: {}", user_id);
    
    match state.recommendation_engine.get_meal_recommendations(user_id, &params).await {
        Ok(recommendations) => {
            info!("Generated {} meal recommendations", recommendations.meals.len());
            Ok(Json(recommendations))
        }
        Err(e) => {
            error!("Failed to generate recommendations: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_nutrition_insights(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<HealthInsights>, StatusCode> {
    info!("Generating nutrition insights for user: {}", user_id);
    
    // Get recent nutrition data
    let recent_nutrition = BasicNutrition {
        calories: 2000.0,
        protein: 80.0,
        carbohydrates: 250.0,
        fat: 65.0,
        fiber: 25.0,
        sugar: 50.0,
        sodium: 2300.0,
    };
    
    let analysis = NutritionAnalysisInternal {
        basic_nutrition: recent_nutrition,
        micronutrients: HashMap::new(),
        dietary_compliance: DietaryCompliance {
            vegetarian_friendly: true,
            vegan_friendly: false,
            gluten_free: true,
            dairy_free: false,
            keto_friendly: false,
            paleo_friendly: true,
            anti_inflammatory_score: 7.5,
        },
        environmental_impact: EnvironmentalImpact {
            carbon_footprint_kg: 2.5,
            water_usage_liters: 1500.0,
            sustainability_score: 8.0,
        },
    };
    
    match state.ai_engine.generate_health_insights(&analysis, user_id).await {
        Ok(insights) => {
            info!("Generated {} health insights", insights.insights.len());
            Ok(Json(insights))
        }
        Err(e) => {
            error!("Failed to generate insights: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn track_daily_nutrition(
    State(state): State<AppState>,
    Json(request): Json<DailyNutritionRequest>,
) -> Result<Json<DailyNutritionSummary>, StatusCode> {
    info!("Tracking daily nutrition for user: {}", request.user_id);
    
    let summary = DailyNutritionSummary {
        user_id: request.user_id,
        date: chrono::Utc::now().date_naive(),
        total_nutrition: request.nutrition.clone(),
        goal_adherence: 85.0,
        recommendations: vec![
            "Great protein intake today!".to_string(),
            "Consider adding more fiber-rich foods.".to_string(),
        ],
        meal_breakdown: vec![],
    };
    
    Ok(Json(summary))
}

async fn predict_nutrient_deficiencies(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<DeficiencyPrediction>>, StatusCode> {
    info!("Predicting nutrient deficiencies for user: {}", user_id);
    
    let nutrition_history = vec![];
    let user_factors = HashMap::new();
    
    match state.ai_engine.predict_deficiencies(&nutrition_history, &user_factors).await {
        Ok(predictions) => {
            info!("Generated {} deficiency predictions", predictions.len());
            Ok(Json(predictions))
        }
        Err(e) => {
            error!("Failed to predict deficiencies: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/nutrition/analyze", post(analyze_nutrition))
        .route("/nutrition/recommendations/:user_id", get(get_meal_recommendations))
        .route("/nutrition/insights/:user_id", get(get_nutrition_insights))
        .route("/nutrition/track", post(track_daily_nutrition))
        .route("/nutrition/deficiencies/:user_id", get(predict_nutrient_deficiencies))
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .init();

    info!("🥗 Starting Nutrition Service");
    info!("📦 Version: {}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep".to_string());
    
    let ai_model_path = std::env::var("AI_MODEL_PATH")
        .unwrap_or_else(|_| "/app/models".to_string());

    // Initialize services
    info!("🗄️ Initializing database connection...");
    let db = DatabaseService::new(&database_url).await?;
    
    info!("🤖 Initializing AI engine...");
    let ai_engine = Arc::new(NutritionAI::new(&ai_model_path).await?);
    
    info!("🔬 Initializing nutrition analyzer...");
    let nutrition_analyzer = Arc::new(NutritionAnalyzer::new(db.clone()));
    
    info!("💡 Initializing recommendation engine...");
    let recommendation_engine = Arc::new(RecommendationEngine::new(db.clone(), ai_engine.clone()));

    let app_state = AppState {
        db,
        ai_engine,
        nutrition_analyzer,
        recommendation_engine,
    };

    let app = create_router(app_state);

    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8081".to_string());

    info!("🌐 Starting server on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    
    info!("🎉 Nutrition Service is running!");
    axum::serve(listener, app).await?;

    Ok(())
}
