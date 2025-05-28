use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub profile: JsonValue, // Store as JSON in database
    pub preferences: JsonValue, // Store as JSON in database
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub full_name: String,
    pub age: Option<i32>,
    pub height_cm: Option<f32>,
    pub weight_kg: Option<f32>,
    pub activity_level: ActivityLevel,
    pub dietary_restrictions: Vec<String>,
    pub allergies: Vec<String>,
    pub health_goals: Vec<HealthGoal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "activity_level", rename_all = "snake_case")]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthGoal {
    WeightLoss,
    WeightGain,
    MuscleGain,
    Maintenance,
    ImprovedEnergy,
    BetterDigestion,
    ReducedInflammation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_cuisines: Vec<String>,
    pub cooking_time_preference: CookingTimePreference,
    pub meal_prep_frequency: MealPrepFrequency,
    pub budget_range: BudgetRange,
    pub kitchen_equipment: Vec<String>,
    pub notification_settings: NotificationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cooking_time_preference", rename_all = "snake_case")]
pub enum CookingTimePreference {
    Quick,      // < 30 min
    Medium,     // 30-60 min
    Extended,   // > 60 min
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "meal_prep_frequency", rename_all = "snake_case")]
pub enum MealPrepFrequency {
    Daily,
    EveryOtherDay,
    TwiceWeekly,
    Weekly,
    BiWeekly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetRange {
    pub min_per_meal: f32,
    pub max_per_meal: f32,
    pub weekly_limit: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub meal_reminders: bool,
    pub prep_reminders: bool,
    pub shopping_reminders: bool,
    pub goal_updates: bool,
    pub weekly_summary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cuisine_type: String,
    pub difficulty_level: DifficultyLevel,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub servings: i32,
    pub ingredients: JsonValue, // Store as JSON
    pub instructions: JsonValue, // Store as JSON
    pub nutrition_info: JsonValue, // Store as JSON
    pub tags: JsonValue, // Store as JSON
    pub rating: Option<f32>,
    pub cost_estimate: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "difficulty_level", rename_all = "snake_case")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub ingredient_id: Uuid,
    pub name: String,
    pub amount: f32,
    pub unit: String,
    pub preparation_note: Option<String>,
    pub substitutions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub step_number: i32,
    pub instruction: String,
    pub time_estimate_minutes: Option<i32>,
    pub temperature: Option<String>,
    pub equipment_needed: Vec<String>,
    pub tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionInfo {
    pub calories_per_serving: f32,
    pub protein_g: f32,
    pub carbohydrates_g: f32,
    pub fat_g: f32,
    pub fiber_g: f32,
    pub sugar_g: f32,
    pub sodium_mg: f32,
    pub vitamins: JsonValue, // Store as JSON
    pub minerals: JsonValue, // Store as JSON
    pub nutrition_score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MealPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub plan_type: PlanType,
    pub meals: JsonValue, // Store as JSON
    pub shopping_list_id: Option<Uuid>,
    pub total_cost_estimate: Option<f32>,
    pub nutrition_summary: JsonValue, // Store as JSON
    pub ai_generated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "plan_type", rename_all = "snake_case")]
pub enum PlanType {
    Weekly,
    BiWeekly,
    Monthly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedMeal {
    pub date: DateTime<Utc>,
    pub meal_type: MealType,
    pub recipe_id: Uuid,
    pub servings: i32,
    pub prep_status: PrepStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "meal_type", rename_all = "snake_case")]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    PreWorkout,
    PostWorkout,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "prep_status", rename_all = "snake_case")]
pub enum PrepStatus {
    NotStarted,
    InProgress,
    Completed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionSummary {
    pub daily_averages: NutritionInfo,
    pub weekly_totals: NutritionInfo,
    pub goal_adherence: JsonValue, // Store as JSON
    pub recommendations: Vec<String>,
}

// Request/Response DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub profile: UserProfile,
    pub preferences: UserPreferences,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct MealRecommendationRequest {
    pub meal_type: MealType,
    pub max_prep_time: Option<i32>,
    pub dietary_restrictions: Vec<String>,
    pub cuisine_preferences: Vec<String>,
    pub nutrition_targets: Option<NutritionInfo>,
    pub available_ingredients: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsDashboard {
    pub nutrition_trends: Vec<NutritionTrend>,
    pub meal_prep_stats: MealPrepStats,
    pub cost_analysis: CostAnalysis,
    pub health_insights: Vec<HealthInsight>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Serialize)]
pub struct NutritionTrend {
    pub date: DateTime<Utc>,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub goal_adherence: f32,
}

#[derive(Debug, Serialize)]
pub struct MealPrepStats {
    pub meals_prepped_this_week: i32,
    pub avg_prep_time: f32,
    pub success_rate: f32,
    pub favorite_recipes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CostAnalysis {
    pub weekly_spending: f32,
    pub cost_per_meal: f32,
    pub budget_adherence: f32,
    pub cost_trends: Vec<(DateTime<Utc>, f32)>,
}

#[derive(Debug, Serialize)]
pub struct HealthInsight {
    pub category: String,
    pub insight: String,
    pub impact_level: ImpactLevel,
    pub recommendation: String,
}

#[derive(Debug, Serialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize)]
pub struct Recommendation {
    pub title: String,
    pub description: String,
    pub action_items: Vec<String>,
    pub priority: i32,
    pub estimated_benefit: String,
}
