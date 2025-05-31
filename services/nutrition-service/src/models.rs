use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicNutrition {
    pub calories: f64,
    pub protein_g: f64,
    pub carbohydrates_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
    pub sodium_mg: f64,
    pub sugar_g: f64,
    pub cholesterol_mg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub carbon_footprint_kg: f64,
    pub water_usage_liters: f64,
    pub sustainability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub suggestion_type: String,
    pub description: String,
    pub potential_improvement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealIngredient {
    pub name: String,
    pub amount: f64,
    pub unit: String,
    pub amount_g: f64,
    pub nutrition_per_100g: BasicNutrition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealAnalysisRequest {
    pub ingredients: Vec<MealIngredient>,
    pub portion_size: f64,
    pub cooking_method: Option<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealAnalysisResponse {
    pub nutrition: BasicNutrition,
    pub environmental_impact: EnvironmentalImpact,
    pub health_score: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutritionRequest {
    pub user_id: String,
    pub date: String,
    pub meals: Vec<MealAnalysisRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutritionResponse {
    pub date: String,
    pub total_nutrition: BasicNutrition,
    pub goal_progress: serde_json::Value,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealRecommendationRequest {
    pub user_id: String,
    pub dietary_preferences: Vec<String>,
    pub allergies: Vec<String>,
    pub target_nutrition: Option<BasicNutrition>,
    pub meal_type: String,
    pub prep_time_max: Option<u32>,
    pub cost_max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealRecommendation {
    pub meal_id: String,
    pub name: String,
    pub description: String,
    pub prep_time: i32,
    pub cost_estimate: f64,
    pub match_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionGoals {
    pub daily_calories: f64,
    pub daily_protein_g: f64,
    pub daily_carbs_g: f64,
    pub daily_fat_g: f64,
    pub daily_fiber_g: f64,
    pub daily_sodium_mg: f64,
    pub protein_percentage: f64,
    pub carb_percentage: f64,
    pub fat_percentage: f64,
    pub fiber_g: f64,
    pub sodium_mg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendData {
    pub date: DateTime<Utc>,
    pub value: f64,
    pub trend_direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionTrends {
    pub date: DateTime<Utc>,
    pub calories: f64,
    pub protein_g: f64,
    pub carbohydrates_g: f64,
    pub fat_g: f64,
    pub trend_direction: String,
    pub confidence_score: f64,
}

// Alias for backward compatibility
pub type NutritionTrend = NutritionTrends;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalProgress {
    pub current_value: f64,
    pub target_value: f64,
    pub percentage_complete: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct NutritionAI {
    pub model_version: String,
    pub confidence_threshold: f64,
}

impl NutritionAI {
    pub fn new() -> Self {
        Self {
            model_version: "1.0.0".to_string(),
            confidence_threshold: 0.7,
        }
    }
}

impl Default for BasicNutrition {
    fn default() -> Self {
        Self {
            calories: 0.0,
            protein_g: 0.0,
            carbohydrates_g: 0.0,
            fat_g: 0.0,
            fiber_g: 0.0,
            sugar_g: 0.0,
            sodium_mg: 0.0,
            cholesterol_mg: 0.0,
        }
    }
}

impl BasicNutrition {
    pub fn calculate_from_ingredients(ingredients: &[MealIngredient]) -> anyhow::Result<Self> {
        let mut total = BasicNutrition::default();

        for ingredient in ingredients {
            let nutrition = &ingredient.nutrition_per_100g;
            let factor = ingredient.amount_g / 100.0;
            total.calories += nutrition.calories * factor;
            total.protein_g += nutrition.protein_g * factor;
            total.carbohydrates_g += nutrition.carbohydrates_g * factor;
            total.fat_g += nutrition.fat_g * factor;
            total.fiber_g += nutrition.fiber_g * factor;
            total.sugar_g += nutrition.sugar_g * factor;
            total.sodium_mg += nutrition.sodium_mg * factor;
            total.cholesterol_mg += nutrition.cholesterol_mg * factor;
        }

        Ok(total)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionProfile {
    pub user_id: String,
    pub daily_calories: f64,
    pub protein_target: f64,
    pub carbs_target: f64,
    pub fat_target: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealAnalysis {
    pub calories: f64,
    pub macronutrients: Macronutrients,
    pub micronutrients: Micronutrients,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Macronutrients {
    pub protein: f64,
    pub carbohydrates: f64,
    pub fat: f64,
    pub fiber: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Micronutrients {
    pub vitamins: std::collections::HashMap<String, f64>,
    pub minerals: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: u32,
    pub cook_time_minutes: u32,
    pub servings: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealPlan {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub meals: Vec<PlannedMeal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedMeal {
    pub id: String,
    pub recipe_id: String,
    pub meal_type: String,
    pub scheduled_date: String,
    pub portion_size: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionLog {
    pub id: String,
    pub user_id: String,
    pub meal_id: Option<String>,
    pub food_name: String,
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub logged_at: DateTime<Utc>,
}
