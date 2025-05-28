use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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

    let app = Router::new()
        .route("/analyze/meal", post(analyze_meal))
        .route("/analyze/daily", post(analyze_daily_nutrition))
        .route("/analyze/trends", get(analyze_nutrition_trends))
        .route("/recommendations/meals", post(recommend_meals))
        .route("/recommendations/supplements", post(recommend_supplements))
        .route("/goals/calculate", post(calculate_nutrition_goals))
        .route("/goals/track", post(track_goal_progress))
        .route("/insights/health", get(generate_health_insights))
        .route("/predict/deficiencies", post(predict_deficiencies))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    tracing::info!("🧠 AI-Powered Nutrition Service running on port 8081");
    
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
struct AnalyzeMealRequest {
    ingredients: Vec<MealIngredient>,
    portion_size: f32,
    cooking_method: Option<String>,
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct MealIngredient {
    name: String,
    amount: f32,
    unit: String,
    preparation: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
struct NutritionAnalysis {
    basic_nutrition: BasicNutrition,
    micronutrients: Vec<Micronutrient>,
    health_score: f32,
    dietary_compliance: DietaryCompliance,
    optimization_suggestions: Vec<OptimizationSuggestion>,
    environmental_impact: EnvironmentalImpact,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicNutrition {
    pub calories: f32,
    pub protein: f32,
    pub carbohydrates: f32,
    pub fat: f32,
    pub fiber: f32,
    pub sugar: f32,
    pub sodium: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Micronutrient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
    pub daily_value_percentage: f32,
    pub bioavailability: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DietaryCompliance {
    pub keto_friendly: bool,
    pub vegan: bool,
    pub vegetarian: bool,
    pub gluten_free: bool,
    pub dairy_free: bool,
    pub low_sodium: bool,
    pub anti_inflammatory_score: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptimizationSuggestion {
    pub category: String,
    pub suggestion: String,
    pub impact: String,
    pub difficulty: String,
    pub estimated_improvement: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentalImpact {
    pub carbon_footprint: f32,
    pub water_usage: f32,
    pub sustainability_score: f32,
}

async fn analyze_meal(
    State(state): State<AppState>,
    Json(request): Json<AnalyzeMealRequest>,
) -> Result<Json<NutritionAnalysis>, StatusCode> {
    let analysis = state
        .analyzer
        .analyze_meal(&request.ingredients, request.portion_size, request.cooking_method.as_deref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let health_insights = state
        .ai_engine
        .generate_health_insights(&analysis, request.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let optimization = state
        .recommendation_engine
        .optimize_meal(&request.ingredients, request.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(NutritionAnalysis {
        basic_nutrition: analysis.basic_nutrition,
        micronutrients: analysis.micronutrients,
        health_score: health_insights.overall_score,
        dietary_compliance: analysis.dietary_compliance,
        optimization_suggestions: optimization.suggestions,
        environmental_impact: analysis.environmental_impact,
    }))
}

#[derive(Deserialize)]
struct DailyNutritionRequest {
    user_id: Uuid,
    date: DateTime<Utc>,
    meals: Vec<LoggedMeal>,
}

#[derive(Deserialize)]
pub struct LoggedMeal {
    meal_type: String,
    ingredients: Vec<MealIngredient>,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Debug, Clone)]
struct DailyNutritionAnalysis {
    total_nutrition: BasicNutrition,
    meal_breakdown: Vec<MealNutritionBreakdown>,
    goal_adherence: GoalAdherence,
    recommendations: Vec<String>,
    next_meal_suggestions: Vec<MealSuggestion>,
    hydration_reminder: bool,
    sleep_nutrition_impact: SleepNutritionImpact,
}

#[derive(Serialize, Debug, Clone)]
pub struct MealNutritionBreakdown {
    meal_type: String,
    nutrition: BasicNutrition,
    timing_score: f32,
    satiety_index: f32,
}

#[derive(Serialize, Debug, Clone)]
pub struct GoalAdherence {
    calories: f32,
    protein: f32,
    carbs: f32,
    fat: f32,
    overall_score: f32,
}

#[derive(Serialize, Debug, Clone)]
pub struct MealSuggestion {
    meal_type: String,
    recipe_name: String,
    nutrition_benefit: String,
    prep_time: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct SleepNutritionImpact {
    sleep_quality_prediction: f32,
    caffeine_cutoff_time: DateTime<Utc>,
    pre_sleep_meal_suggestions: Vec<String>,
}

async fn analyze_daily_nutrition(
    State(state): State<AppState>,
    Json(request): Json<DailyNutritionRequest>,
) -> Result<Json<DailyNutritionAnalysis>, StatusCode> {
    let daily_analysis_internal = state
        .analyzer
        .analyze_daily_nutrition(&request.meals, request.user_id, request.date)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ai_insights = state
        .ai_engine
        .generate_daily_insights(&daily_analysis_internal, request.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert internal type to public type
    let daily_analysis = DailyNutritionAnalysis {
        total_nutrition: daily_analysis_internal.total_nutrition,
        meal_breakdown: daily_analysis_internal.meal_breakdown,
        goal_adherence: daily_analysis_internal.goal_adherence,
        recommendations: vec!["Stay hydrated".to_string()], // placeholder
        next_meal_suggestions: daily_analysis_internal.next_meal_suggestions,
        hydration_reminder: daily_analysis_internal.hydration_reminder,
        sleep_nutrition_impact: daily_analysis_internal.sleep_nutrition_impact,
    };

    Ok(Json(daily_analysis))
}

#[derive(Serialize)]
pub struct NutritionTrends {
    weekly_trends: Vec<WeeklyTrend>,
    monthly_patterns: Vec<MonthlyPattern>,
    seasonal_analysis: SeasonalAnalysis,
    predictive_insights: Vec<PredictiveInsight>,
}

#[derive(Serialize)]
struct WeeklyTrend {
    week_start: DateTime<Utc>,
    avg_calories: f32,
    nutrition_variance: f32,
    adherence_score: f32,
    mood_correlation: Option<f32>,
}

#[derive(Serialize)]
struct MonthlyPattern {
    month: String,
    dominant_patterns: Vec<String>,
    improvement_areas: Vec<String>,
    success_factors: Vec<String>,
}

#[derive(Serialize)]
pub struct SeasonalAnalysis {
    seasonal_preferences: Vec<SeasonalPreference>,
    nutrient_absorption_factors: Vec<String>,
    recommendation_adjustments: Vec<String>,
}

#[derive(Serialize)]
struct SeasonalPreference {
    season: String,
    preferred_foods: Vec<String>,
    nutrition_focus: Vec<String>,
}

#[derive(Serialize)]
struct PredictiveInsight {
    prediction: String,
    confidence: f32,
    timeframe: String,
    actionable_steps: Vec<String>,
}

async fn analyze_nutrition_trends(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<NutritionTrends>, StatusCode> {
    let user_id: Uuid = params
        .get("user_id")
        .and_then(|id| id.parse().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let trends = state
        .analyzer
        .analyze_long_term_trends(user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(trends))
}

async fn recommend_meals(
    State(state): State<AppState>,
    Json(request): Json<MealRecommendationRequest>,
) -> Result<Json<Vec<MealRecommendation>>, StatusCode> {
    let recommendations = state
        .recommendation_engine
        .recommend_meals(&request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(recommendations))
}

async fn recommend_supplements(
    State(state): State<AppState>,
    Json(request): Json<models::SupplementRecommendationRequest>,
) -> Result<Json<Vec<models::SupplementRecommendation>>, StatusCode> {
    let recommendations = state
        .recommendation_engine
        .recommend_supplements(&request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(recommendations))
}

async fn calculate_nutrition_goals(
    State(state): State<AppState>,
    Json(request): Json<models::NutritionGoalsRequest>,
) -> Result<Json<models::NutritionGoals>, StatusCode> {
    let goals = state
        .recommendation_engine
        .calculate_nutrition_goals(&request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(goals))
}

async fn track_goal_progress(
    State(state): State<AppState>,
    Json(request): Json<models::GoalTrackingRequest>,
) -> Result<Json<models::GoalProgress>, StatusCode> {
    let progress = state
        .recommendation_engine
        .track_goal_progress(&request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(progress))
}

async fn generate_health_insights(
    State(_state): State<AppState>,
    Query(_params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    // Placeholder implementation
    Ok(Json(vec!["Stay hydrated".to_string(), "Increase vegetable intake".to_string()]))
}

async fn predict_deficiencies(
    State(state): State<AppState>,
    Json(request): Json<models::DeficiencyPredictionRequest>,
) -> Result<Json<Vec<models::DeficiencyPrediction>>, StatusCode> {
    let predictions = state
        .recommendation_engine
        .predict_deficiencies(&request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(predictions))
}

#[derive(Deserialize)]
pub struct MealRecommendationRequest {
    user_id: Uuid,
    meal_type: String,
    time_constraint: Option<i32>,
    dietary_preferences: Vec<String>,
    nutrition_targets: Option<BasicNutrition>,
    mood_state: Option<String>,
    energy_level: Option<String>,
    recent_meals: Vec<String>,
}

#[derive(Serialize)]
pub struct MealRecommendation {
    recipe_id: Uuid,
    recipe_name: String,
    match_score: f32,
    nutrition_alignment: f32,
    variety_score: f32,
    preparation_time: i32,
    key_benefits: Vec<String>,
    potential_concerns: Vec<String>,
}
