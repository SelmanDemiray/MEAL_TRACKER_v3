use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Request structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionAnalysisRequest {
    pub user_id: Option<Uuid>,
    pub ingredients: Vec<Ingredient>,
    pub meal_type: Option<String>,
    pub serving_size: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
    pub preparation: Option<String>,
}

// Core nutrition data structures
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
    pub bioavailability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietaryCompliance {
    pub vegetarian_friendly: bool,
    pub vegan_friendly: bool,
    pub gluten_free: bool,
    pub dairy_free: bool,
    pub keto_friendly: bool,
    pub paleo_friendly: bool,
    pub anti_inflammatory_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub carbon_footprint_kg: f32,
    pub water_usage_liters: f32,
    pub sustainability_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub suggestion_type: String,
    pub description: String,
    pub nutrition_impact: BasicNutrition,
    pub implementation_difficulty: String,
}

// Main analysis result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionAnalysis {
    pub basic_nutrition: BasicNutrition,
    pub micronutrients: HashMap<String, f32>,
    pub health_score: f32,
    pub dietary_compliance: DietaryCompliance,
    pub environmental_impact: EnvironmentalImpact,
    pub cost_estimate: Option<f32>,
    pub allergen_warnings: Vec<String>,
    pub preparation_tips: Vec<String>,
}

// AI recommendation structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealRecommendation {
    pub meal_id: Uuid,
    pub name: String,
    pub description: String,
    pub nutrition: BasicNutrition,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: i32,
    pub difficulty_level: String,
    pub cost_estimate: f32,
    pub recommendation_score: f32,
    pub reasons: Vec<String>,
}

// Response structures
#[derive(Debug, Serialize)]
pub struct MealRecommendations {
    pub meals: Vec<MealRecommendation>,
    pub total_nutrition: BasicNutrition,
    pub adherence_to_goals: f32,
    pub variety_score: f32,
}

// Request types for different services
#[derive(Debug, Deserialize)]
pub struct RecommendationParams {
    pub meal_type: Option<String>,
    pub dietary_restrictions: Option<String>,
    pub max_prep_time: Option<i32>,
    pub budget_limit: Option<f32>,
    pub cuisine_preference: Option<String>,
    pub equipment_available: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthInsights {
    pub overall_score: f32,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyInsights {
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealOptimization {
    pub suggestions: Vec<OptimizationSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeficiencyPrediction {
    pub nutrient: String,
    pub risk_level: String,
    pub confidence: f32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionGoals {
    pub target_calories: Option<f32>,
    pub target_protein_g: Option<f32>,
    pub target_carbs_g: Option<f32>,
    pub target_fat_g: Option<f32>,
    pub target_fiber_g: Option<f32>,
    pub target_sodium_mg: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutritionRequest {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub nutrition: BasicNutrition,
    pub meals: Vec<MealEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealEntry {
    pub meal_type: String,
    pub food_items: Vec<Ingredient>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutritionSummary {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub total_nutrition: BasicNutrition,
    pub goal_adherence: f32,
    pub recommendations: Vec<String>,
    pub meal_breakdown: Vec<MealNutritionBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealNutritionBreakdown {
    pub meal_type: String,
    pub nutrition: BasicNutrition,
    pub percentage_of_daily: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatietyPrediction {
    pub satiety_score: f32,
    pub duration_hours: f32,
    pub factors: Vec<String>,
}

// Internal models for AI processing
#[derive(Debug, Clone)]
pub struct NutritionAnalysisInternal {
    pub basic_nutrition: BasicNutrition,
    pub micronutrients: HashMap<String, f32>,
    pub dietary_compliance: DietaryCompliance,
    pub environmental_impact: EnvironmentalImpact,
}
