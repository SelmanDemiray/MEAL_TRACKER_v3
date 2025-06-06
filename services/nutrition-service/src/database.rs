// Database operations for nutrition service
use anyhow::Result;
use sqlx::{PgPool, Row};
use tracing::info;

#[derive(Clone, Debug)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to nutrition database...");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(database_url)
            .await?;

        info!("Nutrition database connection established");
        Ok(Self { pool })
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_nutrition_data(&self, ingredient_name: &str) -> Result<Option<NutritionData>> {
        let row = sqlx::query(
            "SELECT name, calories_per_100g, protein_g, carbs_g, fat_g, fiber_g FROM nutrition_data WHERE LOWER(name) = LOWER($1)",
        )
        .bind(ingredient_name)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(NutritionData {
                name: row.get("name"),
                calories_per_100g: row.get("calories_per_100g"),
                protein_g: row.get("protein_g"),
                carbs_g: row.get("carbs_g"),
                fat_g: row.get("fat_g"),
                fiber_g: row.get("fiber_g"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_preferences(&self, user_id: &str) -> Result<Option<UserPreferences>> {
        let row = sqlx::query(
            "SELECT dietary_restrictions, allergies, health_goals FROM user_profiles WHERE user_id = $1",
        )
        .bind(uuid::Uuid::parse_str(user_id)?)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(UserPreferences {
                dietary_restrictions: row.get("dietary_restrictions"),
                allergies: row.get("allergies"),
                health_goals: row.get("health_goals"),
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct NutritionData {
    pub name: String,
    pub calories_per_100g: f32,
    pub protein_g: f32,
    pub carbs_g: f32,
    pub fat_g: f32,
    pub fiber_g: f32,
}

#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub dietary_restrictions: Option<Vec<String>>,
    pub allergies: Option<Vec<String>>,
    pub health_goals: Option<Vec<String>>,
}
