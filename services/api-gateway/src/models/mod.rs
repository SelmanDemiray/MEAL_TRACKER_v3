use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub role: String,
    pub profile: Option<UserProfile>,
    pub preferences: Option<UserPreferences>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub dietary_restrictions: Vec<String>,
    pub allergies: Vec<String>,
    pub preferred_cuisines: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub servings: i32,
    pub difficulty: String,
    pub cuisine_type: Option<String>,
    pub tags: Vec<String>,
    pub nutritional_info: Option<NutritionalInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionalInfo {
    pub calories_per_serving: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
    pub sugar_g: f64,
    pub sodium_mg: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub meals: Vec<PlannedMeal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedMeal {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub meal_type: String, // breakfast, lunch, dinner, snack
    pub date: DateTime<Utc>,
    pub servings: i32,
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
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeRequest {
    pub name: String,
    pub description: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub servings: i32,
    pub difficulty: String,
    pub cuisine_type: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}
