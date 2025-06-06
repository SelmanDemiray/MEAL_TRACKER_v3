use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct JsonRecipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub directions: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub total_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub source_repository: Option<String>,
    pub original_filename: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub directions: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequest {
    pub repository_url: String,
    pub import_format: Option<String>,
    pub filter_criteria: Option<FilterCriteria>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCriteria {
    pub max_prep_time: Option<i32>,
    pub max_cook_time: Option<i32>,
    pub required_tags: Option<Vec<String>>,
    pub excluded_tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportBatch {
    pub id: Uuid,
    pub repository_url: String,
    pub import_status: ImportStatus,
    pub total_recipes: i32,
    pub successful_imports: i32,
    pub failed_imports: i32,
    pub error_log: Vec<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResponse {
    pub batch_id: Uuid,
    pub status: ImportStatus,
    pub message: String,
    pub estimated_recipes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeFile {
    pub file_path: String,
    pub content: String,
    pub format: FileFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileFormat {
    Json,
    Yaml,
    Markdown,
    PlainText,
}

// Structure for parsing recipe files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedRecipe {
    pub name: String,
    pub description: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub servings: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub tags: Vec<String>,
    pub source_file: String,
}

#[derive(Debug, Serialize)]
pub struct RecipeSearchResult {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub total_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub source_repository: Option<String>,
    pub similarity_score: f32,
}

#[derive(Debug, Serialize)]
pub struct RecipeDetail {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub total_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub source_repository: Option<String>,
    pub original_filename: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub directions: Option<Vec<String>>,
}
