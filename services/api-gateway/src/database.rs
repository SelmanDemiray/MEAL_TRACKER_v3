use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use tracing::info;

use crate::models::{User, Recipe, RecipeSearchParams};

#[derive(Clone)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to database...");
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(database_url)
            .await?;

        info!("Database connection established");
        Ok(Self { pool })
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, email_verified, is_active, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.email_verified)
        .bind(user.is_active)
        .bind(&user.role)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, email_verified, is_active, role, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            email_verified: row.get("email_verified"),
            is_active: row.get("is_active"),
            role: row.get("role"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, email_verified, is_active, role, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            email_verified: row.get("email_verified"),
            is_active: row.get("is_active"),
            role: row.get("role"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    // Recipe operations
    pub async fn search_recipes(&self, params: &RecipeSearchParams) -> Result<Vec<Recipe>> {
        let limit = params.limit.unwrap_or(20).min(100);
        let offset = (params.page.unwrap_or(1) - 1) * limit;

        let rows = sqlx::query(
            "SELECT id, user_id, name, description, prep_time_minutes, cook_time_minutes, total_time_minutes, servings, tags, source_repository, ingredients, directions, created_at, updated_at FROM recipes ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let recipes = rows.into_iter().map(|row| Recipe {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            description: row.get("description"),
            prep_time_minutes: row.get("prep_time_minutes"),
            cook_time_minutes: row.get("cook_time_minutes"),
            total_time_minutes: row.get("total_time_minutes"),
            servings: row.get("servings"),
            tags: row.get("tags"),
            source_repository: row.get("source_repository"),
            ingredients: row.get("ingredients"),
            directions: row.get("directions"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(recipes)
    }

    pub async fn get_recipe_by_id(&self, recipe_id: Uuid) -> Result<Recipe> {
        let row = sqlx::query(
            "SELECT id, user_id, name, description, prep_time_minutes, cook_time_minutes, total_time_minutes, servings, tags, source_repository, ingredients, directions, created_at, updated_at FROM recipes WHERE id = $1"
        )
        .bind(recipe_id)
        .fetch_one(&self.pool)
        .await?;

        let recipe = Recipe {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            description: row.get("description"),
            prep_time_minutes: row.get("prep_time_minutes"),
            cook_time_minutes: row.get("cook_time_minutes"),
            total_time_minutes: row.get("total_time_minutes"),
            servings: row.get("servings"),
            tags: row.get("tags"),
            source_repository: row.get("source_repository"),
            ingredients: row.get("ingredients"),
            directions: row.get("directions"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(recipe)
    }

    pub async fn create_recipe(&self, recipe: &Recipe) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO recipes (
                id, user_id, name, description, prep_time_minutes, cook_time_minutes,
                total_time_minutes, servings, tags, source_repository, ingredients,
                directions, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#
        )
        .bind(&recipe.id)
        .bind(&recipe.user_id)
        .bind(&recipe.name)
        .bind(&recipe.description)
        .bind(recipe.prep_time_minutes)
        .bind(recipe.cook_time_minutes)
        .bind(recipe.total_time_minutes)
        .bind(recipe.servings)
        .bind(&recipe.tags)
        .bind(&recipe.source_repository)
        .bind(&recipe.ingredients)
        .bind(&recipe.directions)
        .bind(recipe.created_at)
        .bind(recipe.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
