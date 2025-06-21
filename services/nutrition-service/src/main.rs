use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info};
use tracing_subscriber;
use uuid::Uuid;
use chrono::{DateTime, Utc};

mod ai_engine;
mod nutrition_analyzer;
mod recommendation_engine;
mod models;
mod database;

use ai_engine::NutritionAI;
use nutrition_analyzer::NutritionalAnalyzer;
use recommendation_engine::RecommendationEngine;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::Client,
    pub ai_engine: Arc<NutritionAI>,
    pub analyzer: Arc<NutritionalAnalyzer>,
    pub recommendation_engine: Arc<RecommendationEngine>,
}

// Add missing struct definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealIngredient {
    pub ingredient_id: Uuid,
    pub name: String,
    pub amount: f32,
    pub unit: String,
    pub preparation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicNutrition {
    pub calories: f32,
    pub protein: f32,
    pub carbohydrates: f32,
    pub fat: f32,
    pub fiber: f32,
    pub sugar: f32,
    pub sodium: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Micronutrient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
    pub daily_value_percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietaryCompliance {
    pub keto_friendly: bool,
    pub vegan: bool,
    pub vegetarian: bool,
    pub gluten_free: bool,
    pub dairy_free: bool,
    pub low_sodium: bool,
    pub anti_inflammatory_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub carbon_footprint: f32,
    pub water_usage: f32,
    pub sustainability_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub suggestion_type: String,
    pub description: String,
    pub impact_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedMeal {
    pub meal_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub ingredients: Vec<MealIngredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealNutritionBreakdown {
    pub meal_type: String,
    pub nutrition: BasicNutrition,
    pub percentage_of_daily_goals: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAdherence {
    pub overall_score: f32,
    pub protein_adherence: f32,
    pub carb_adherence: f32,
    pub fat_adherence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealSuggestion {
    pub meal_name: String,
    pub estimated_nutrition: BasicNutrition,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepNutritionImpact {
    pub sleep_quality_score: f32,
    pub recommendations: Vec<String>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct MealAnalysisRequest {
    pub user_id: Uuid,
    pub ingredients: Vec<MealIngredient>,
    pub portion_size: f32,
    pub cooking_method: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MealAnalysisResponse {
    pub nutrition_analysis: models::NutritionAnalysisInternal,
    pub health_insights: models::HealthInsights,
}

#[derive(Debug, Deserialize)]
pub struct DailyNutritionRequest {
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub meals: Vec<LoggedMeal>,
}

#[derive(Debug, Serialize)]
pub struct DailyNutritionResponse {
    pub analysis: models::DailyNutritionAnalysisInternal,
    pub insights: models::DailyInsights,
}

#[derive(Debug, Serialize)]
pub struct HealthInsights {
    pub overall_score: f32,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "nutrition-service".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Nutrition service...");

    let database_url = std::env::var("DATABASE_URL")?;
    let redis_url = std::env::var("REDIS_URL")?;

    let db = sqlx::PgPool::connect(&database_url).await?;
    let redis = redis::Client::open(redis_url)?;

    // Initialize AI components
    let ai_engine = Arc::new(NutritionAI::new().await?);
    let analyzer = Arc::new(NutritionalAnalyzer::new());
    let recommendation_engine = Arc::new(RecommendationEngine::new(ai_engine.clone()));

    let app_state = AppState {
        db,
        redis,
        ai_engine,
        analyzer,
        recommendation_engine,
    };

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/analyze/meal", post(analyze_meal))
        .route("/analyze/daily", post(analyze_daily_nutrition))
        .route("/analyze/trends", get(analyze_nutrition_trends))
        .route("/recommendations/meals", post(recommend_meals))
        .route("/recommendations/supplements", post(recommend_supplements))
        .route("/insights/health", get(generate_health_insights))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Run it with hyper on localhost:8081
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("Nutrition service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Handler functions
async fn analyze_meal(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(request): axum::Json<MealAnalysisRequest>,
) -> Result<axum::Json<MealAnalysisResponse>, axum::http::StatusCode> {
    let analysis = state.analyzer
        .analyze_meal(&request.ingredients, request.portion_size, request.cooking_method.as_deref())
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let insights = state.ai_engine
        .generate_health_insights(&analysis, request.user_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::Json(MealAnalysisResponse {
        nutrition_analysis: analysis,
        health_insights: insights,
    }))
}

async fn analyze_daily_nutrition(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(request): axum::Json<DailyNutritionRequest>,
) -> Result<axum::Json<DailyNutritionResponse>, axum::http::StatusCode> {
    // Calculate total nutrition from all meals
    let mut total_nutrition = BasicNutrition {
        calories: 0.0,
        protein: 0.0,
        carbohydrates: 0.0,
        fat: 0.0,
        fiber: 0.0,
        sugar: 0.0,
        sodium: 0.0,
    };

    let mut meal_breakdown = Vec::new();

    for meal in &request.meals {
        let meal_analysis = state.analyzer
            .analyze_meal(&meal.ingredients, 1.0, None)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        total_nutrition.calories += meal_analysis.basic_nutrition.calories;
        total_nutrition.protein += meal_analysis.basic_nutrition.protein;
        total_nutrition.carbohydrates += meal_analysis.basic_nutrition.carbohydrates;
        total_nutrition.fat += meal_analysis.basic_nutrition.fat;
        total_nutrition.fiber += meal_analysis.basic_nutrition.fiber;
        total_nutrition.sugar += meal_analysis.basic_nutrition.sugar;
        total_nutrition.sodium += meal_analysis.basic_nutrition.sodium;

        meal_breakdown.push(MealNutritionBreakdown {
            meal_type: "main".to_string(),
            nutrition: meal_analysis.basic_nutrition.clone(),
            percentage_of_daily_goals: 25.0, // Placeholder
        });
    }

    // Generate mock user goals for demonstration
    let user_goals = BasicNutrition {
        calories: 2000.0,
        protein: 150.0,
        carbohydrates: 250.0,
        fat: 65.0,
        fiber: 25.0,
        sugar: 50.0,
        sodium: 2300.0,
    };

    let analysis = models::DailyNutritionAnalysisInternal {
        total_nutrition: total_nutrition.clone(),
        meal_breakdown,
        goal_adherence: GoalAdherence {
            overall_score: 85.0,
            protein_adherence: total_nutrition.protein / user_goals.protein * 100.0,
            carb_adherence: total_nutrition.carbohydrates / user_goals.carbohydrates * 100.0,
            fat_adherence: total_nutrition.fat / user_goals.fat * 100.0,
        },
        next_meal_suggestions: generate_meal_suggestions(&total_nutrition, &user_goals),
        hydration_reminder: total_nutrition.sodium > 1500.0,
        sleep_nutrition_impact: SleepNutritionImpact {
            sleep_quality_score: 7.5,
            recommendations: vec!["Avoid caffeine after 2 PM".to_string()],
        },
    };

    let insights = state.ai_engine
        .generate_daily_insights(&analysis, request.user_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::Json(DailyNutritionResponse {
        analysis,
        insights,
    }))
}

async fn analyze_nutrition_trends(
    axum::extract::State(_state): axum::extract::State<AppState>,
    axum::extract::Query(_params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::Json<serde_json::Value>, axum::http::StatusCode> {
    Ok(axum::Json(serde_json::json!({
        "trends": [],
        "insights": []
    })))
}

async fn recommend_meals(
    axum::extract::State(_state): axum::extract::State<AppState>,
    axum::Json(_request): axum::Json<serde_json::Value>,
) -> Result<axum::Json<Vec<MealRecommendation>>, axum::http::StatusCode> {
    Ok(axum::Json(vec![]))
}

async fn recommend_supplements(
    axum::extract::State(_state): axum::extract::State<AppState>,
    axum::Json(_request): axum::Json<serde_json::Value>,
) -> Result<axum::Json<Vec<String>>, axum::http::StatusCode> {
    Ok(axum::Json(vec![]))
}

async fn generate_health_insights(
    axum::extract::State(_state): axum::extract::State<AppState>,
    axum::extract::Query(_params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::Json<HealthInsights>, axum::http::StatusCode> {
    Ok(axum::Json(HealthInsights {
        overall_score: 85.0,
        insights: vec!["Good protein intake".to_string()],
        recommendations: vec!["Add more vegetables".to_string()],
    }))
}

// Helper functions
fn generate_meal_suggestions(current_nutrition: &BasicNutrition, goals: &BasicNutrition) -> Vec<MealSuggestion> {
    let mut suggestions = Vec::new();
    
    if current_nutrition.protein < goals.protein * 0.8 {
        suggestions.push(MealSuggestion {
            meal_name: "Grilled Chicken Salad".to_string(),
            estimated_nutrition: BasicNutrition {
                calories: 350.0,
                protein: 30.0,
                carbohydrates: 15.0,
                fat: 12.0,
                fiber: 8.0,
                sugar: 8.0,
                sodium: 450.0,
            },
            reasoning: "High protein content to meet daily goals".to_string(),
        });
    }
    
    suggestions
}

#[derive(Debug, Serialize, Clone)]
pub struct MealRecommendation {
    pub meal_id: Uuid,
    pub name: String,
    pub ingredients: Vec<MealIngredient>,
    pub nutrition: BasicNutrition,
    pub estimated_nutrition: BasicNutrition,
    pub confidence_score: f32,
    pub prep_time: i32,
    pub difficulty: String,
    pub cuisine_type: String,
}

// Add missing types
#[derive(Debug, Deserialize)]
pub struct MealRecommendationRequest {
    pub user_id: Uuid,
    pub dietary_restrictions: Vec<String>,
    pub preferences: Vec<String>,
    pub target_nutrition: Option<BasicNutrition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionTrends {
    pub trends: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalAnalysis {
    pub analysis: Vec<String>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}
