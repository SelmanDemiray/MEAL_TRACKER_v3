use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// User-related models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

// Recipe-related models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub total_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub source_repository: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub directions: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeSearchParams {
    pub q: Option<String>,
    pub tags: Option<String>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeRequest {
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub directions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ImportRecipeRequest {
    pub repository_url: String,
}

#[derive(Debug, Serialize)]
pub struct ImportResponse {
    pub batch_id: String,
    pub message: String,
}

// Nutrition models
#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionGoals {
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub meal_type: String,
    pub recipe_id: Option<Uuid>,
    pub serving_size: f64,
    pub logged_at: DateTime<Utc>,
}

// API Response wrappers
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            errors: None,
        }
    }
}

// WebSocket message types
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub user_id: Uuid,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

// Meal and nutrition models
#[derive(Debug, Serialize, Deserialize)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub recipes: Vec<Recipe>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub meals: Vec<Meal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub meal_id: Option<Uuid>,
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub logged_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub items: Vec<ShoppingItem>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub id: Uuid,
    pub name: String,
    pub quantity: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub quantity: String,
    pub expiry_date: Option<DateTime<Utc>>,
}
