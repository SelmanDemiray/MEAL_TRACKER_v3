use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Internal types for analysis
#[derive(Debug, Clone)]
pub struct NutritionAnalysisInternal {
    pub basic_nutrition: crate::BasicNutrition,
    pub micronutrients: Vec<crate::Micronutrient>,
    pub dietary_compliance: crate::DietaryCompliance,
    pub environmental_impact: crate::EnvironmentalImpact,
}

#[derive(Debug, Clone)]
pub struct DailyNutritionAnalysisInternal {
    pub total_nutrition: crate::BasicNutrition,
    pub meal_breakdown: Vec<crate::MealNutritionBreakdown>,
    pub goal_adherence: crate::GoalAdherence,
    pub next_meal_suggestions: Vec<crate::MealSuggestion>,
    pub hydration_reminder: bool,
    pub sleep_nutrition_impact: crate::SleepNutritionImpact,
}

// AI insights types
#[derive(Debug, Clone)]
pub struct HealthInsights {
    pub overall_score: f32,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone)]
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
    pub nutrition_history: Vec<crate::BasicNutrition>,
    pub health_indicators: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeficiencyPrediction {
    pub nutrient: String,
    pub risk_level: String,
    pub confidence: f32,
    pub recommendations: Vec<String>,
}

// Re-export types from main.rs with proper derives
pub use crate::{
    BasicNutrition, Micronutrient, DietaryCompliance, OptimizationSuggestion,
    EnvironmentalImpact, MealIngredient, LoggedMeal, MealNutritionBreakdown,
    GoalAdherence, MealSuggestion, SleepNutritionImpact, NutritionTrends,
    SeasonalAnalysis, MealRecommendationRequest, MealRecommendation
};
