use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Internal types for analysis
#[derive(Debug, Clone, Serialize)]
pub struct NutritionAnalysisInternal {
    pub basic_nutrition: crate::BasicNutrition,
    pub micronutrients: Vec<crate::Micronutrient>,
    pub dietary_compliance: crate::DietaryCompliance,
    pub environmental_impact: crate::EnvironmentalImpact,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyNutritionAnalysisInternal {
    pub total_nutrition: crate::BasicNutrition,
    pub meal_breakdown: Vec<crate::MealNutritionBreakdown>,
    pub goal_adherence: crate::GoalAdherence,
    pub next_meal_suggestions: Vec<crate::MealSuggestion>,
    pub hydration_reminder: bool,
    pub sleep_nutrition_impact: crate::SleepNutritionImpact,
}

// AI insights types
#[derive(Debug, Clone, Serialize)]
pub struct HealthInsights {
    pub overall_score: f32,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyInsights {
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MealOptimization {
    pub suggestions: Vec<crate::OptimizationSuggestion>,
}

// Request types for different services
#[derive(Debug, Deserialize)]
pub struct SupplementRecommendationRequest {
    pub user_id: Uuid,
    pub current_nutrition: crate::BasicNutrition,
    pub health_goals: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SupplementRecommendation {
    pub name: String,
    pub dosage: String,
    pub reason: String,
    pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct NutritionGoalsRequest {
    pub user_id: Uuid,
    pub age: i32,
    pub weight_kg: f32,
    pub height_cm: f32,
    pub activity_level: String,
    pub goals: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionGoals {
    pub daily_calories: f32,
    pub daily_protein: f32,
    pub daily_carbs: f32,
    pub daily_fat: f32,
    pub daily_fiber: f32,
}

#[derive(Debug, Deserialize)]
pub struct GoalTrackingRequest {
    pub user_id: Uuid,
    pub current_intake: crate::BasicNutrition,
    pub goals: NutritionGoals,
}

#[derive(Debug, Serialize)]
pub struct GoalProgress {
    pub adherence_score: f32,
    pub areas_for_improvement: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeficiencyPredictionRequest {
    pub user_id: Uuid,
    pub nutrition_history: Vec<DailyNutritionHistory>,
    pub health_indicators: Vec<String>,
    pub time_period_days: i32,
}

#[derive(Debug, Serialize)]
pub struct DeficiencyPrediction {
    pub nutrient_name: String,
    pub deficiency_risk: f32,
    pub predicted_onset_days: Option<i32>,
    pub severity: DeficiencySeverity,
    pub recommendations: Vec<String>,
    pub confidence_level: f32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeficiencySeverity {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Deserialize)]
pub struct DailyNutritionHistory {
    pub date: chrono::DateTime<chrono::Utc>,
    pub nutrition: crate::BasicNutrition,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MealOptimizationRequest {
    pub user_id: Uuid,
    pub current_meal: Vec<crate::MealIngredient>,
    pub nutrition_goals: Option<crate::BasicNutrition>,
    pub dietary_restrictions: Vec<String>,
    pub optimization_priorities: Vec<OptimizationPriority>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationPriority {
    Nutrition,
    Cost,
    Taste,
    PrepTime,
    Sustainability,
}

#[derive(Debug, Serialize)]
pub struct MealOptimizationResult {
    pub original_nutrition: crate::BasicNutrition,
    pub optimized_nutrition: crate::BasicNutrition,
    pub ingredient_changes: Vec<IngredientChange>,
    pub improvement_score: f32,
    pub cost_impact: f32,
    pub prep_time_impact: i32,
}

#[derive(Debug, Serialize)]
pub struct IngredientChange {
    pub action: ChangeAction,
    pub ingredient: String,
    pub amount: Option<f32>,
    pub unit: Option<String>,
    pub reason: String,
    pub nutrition_impact: crate::BasicNutrition,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeAction {
    Add,
    Remove,
    Increase,
    Decrease,
    Substitute,
}

#[derive(Debug, Deserialize)]
pub struct TrendAnalysisRequest {
    pub user_id: Uuid,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub metrics: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TrendAnalysisResult {
    pub trends: Vec<NutritionTrend>,
    pub insights: Vec<TrendInsight>,
    pub predictions: Vec<TrendPrediction>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct NutritionTrend {
    pub metric: String,
    pub data_points: Vec<TrendDataPoint>,
    pub trend_direction: TrendDirection,
    pub correlation_strength: f32,
}

#[derive(Debug, Serialize)]
pub struct TrendDataPoint {
    pub date: chrono::DateTime<chrono::Utc>,
    pub value: f32,
    pub goal_value: Option<f32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
    Volatile,
}

#[derive(Debug, Serialize)]
pub struct TrendInsight {
    pub category: String,
    pub insight: String,
    pub confidence: f32,
    pub action_required: bool,
}

#[derive(Debug, Serialize)]
pub struct TrendPrediction {
    pub metric: String,
    pub predicted_value: f32,
    pub prediction_date: chrono::DateTime<chrono::Utc>,
    pub confidence_interval: (f32, f32),
}

// Re-export only the types that are actually used
// These re-exports are commented out to avoid unused import warnings
// pub use crate::{
//     BasicNutrition, Micronutrient, DietaryCompliance, OptimizationSuggestion,
//     EnvironmentalImpact, MealIngredient, LoggedMeal, MealNutritionBreakdown,
//     GoalAdherence, MealSuggestion, SleepNutritionImpact,
//     MealRecommendationRequest, MealRecommendation
// };
