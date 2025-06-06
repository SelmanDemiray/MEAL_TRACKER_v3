mod models;
mod recipe_importer;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use models::{ImportRequest, ImportResponse, ImportBatch, ImportStatus, Recipe};
use recipe_importer::RecipeImporter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Option<sqlx::PgPool>, // TODO: Add database connection
    pub importer: Arc<RecipeImporter>,
    pub import_jobs: Arc<RwLock<HashMap<Uuid, ImportBatch>>>,
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
    tags: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Recipe Import Service");

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://mealprep:mealprep_secure_2024@postgres:5432/mealprep".to_string());

    // Wait for database to be ready
    let mut retry_count = 0;
    let pool = loop {
        match sqlx::PgPool::connect(&database_url).await {
            Ok(pool) => break pool,
            Err(e) => {
                retry_count += 1;
                if retry_count > 30 {
                    panic!("Failed to connect to database after 30 retries: {}", e);
                }
                error!("Failed to connect to database (attempt {}): {}", retry_count, e);
                sleep(Duration::from_secs(2)).await;
            }
        }
    };

    info!("Connected to database");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations completed");

    // Create recipe importer
    let importer = Arc::new(RecipeImporter::new(pool));

    // Auto-import recipes on startup (run in background)
    let importer_clone = importer.clone();
    tokio::spawn(async move {
        info!("Starting auto-import of default recipes");
        match importer_clone.auto_import_default_recipes().await {
            Ok(batch_ids) => {
                info!("Auto-import completed. Batch IDs: {:?}", batch_ids);
            }
            Err(e) => {
                error!("Auto-import failed: {}", e);
            }
        }
    });

    // Build application routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/recipes/import", post(import_recipes))
        .route("/api/recipes/import/:batch_id/status", get(get_import_status))
        .route("/api/recipes/search", get(search_recipes))
        .route("/api/recipes/:id", get(get_recipe))
        .layer(CorsLayer::permissive())
        .with_state(importer);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8083").await?;
    info!("Recipe Import Service listening on port 8083");

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

async fn health_check() -> Result<Json<HealthResponse>, StatusCode> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "recipe-import-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

async fn import_recipes(
    State(state): State<AppState>,
    Json(request): Json<ImportRequest>,
) -> Result<Json<ImportResponse>, StatusCode> {
    info!("Received import request for repository: {}", request.repository_url);

    // Start import process in background
    let importer = state.importer.clone();
    let import_jobs = state.import_jobs.clone();
    
    let batch_id = Uuid::new_v4();
    
    // Create initial batch entry
    let mut initial_batch = ImportBatch {
        id: batch_id,
        repository_url: request.repository_url.clone(),
        import_status: ImportStatus::Pending,
        total_recipes: 0,
        successful_imports: 0,
        failed_imports: 0,
        error_log: Vec::new(),
        started_at: chrono::Utc::now(),
        completed_at: None,
        created_by: None,
    };
    
    // Store the initial batch
    {
        let mut jobs = import_jobs.write().await;
        jobs.insert(batch_id, initial_batch.clone());
    }
    
    // Start import in background task
    tokio::spawn(async move {
        match importer.import_from_repository(request).await {
            Ok(completed_batch) => {
                let mut jobs = import_jobs.write().await;
                jobs.insert(batch_id, completed_batch);
                info!("Import batch {} completed successfully", batch_id);
            }
            Err(e) => {
                error!("Import batch {} failed: {}", batch_id, e);
                initial_batch.import_status = ImportStatus::Failed;
                initial_batch.error_log.push(format!("Import failed: {}", e));
                initial_batch.completed_at = Some(chrono::Utc::now());
                
                let mut jobs = import_jobs.write().await;
                jobs.insert(batch_id, initial_batch);
            }
        }
    });

    let response = ImportResponse {
        batch_id,
        status: ImportStatus::Pending,
        message: "Import started successfully".to_string(),
        estimated_recipes: None,
    };

    Ok(Json(response))
}

#[derive(Deserialize)]
struct BatchIdPath {
    batch_id: Uuid,
}

async fn get_import_status(
    State(state): State<AppState>,
    axum::extract::Path(BatchIdPath { batch_id }): axum::extract::Path<BatchIdPath>,
) -> Result<Json<ImportBatch>, StatusCode> {
    let jobs = state.import_jobs.read().await;
    
    match jobs.get(&batch_id) {
        Some(batch) => Ok(Json(batch.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn search_recipes(
    State(importer): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Recipe>>, StatusCode> {
    let limit = params.limit.unwrap_or(50);
    let offset = params.page.unwrap_or(0) * limit;

    let query = if let Some(search_term) = params.q {
        if let Some(tags) = params.tags {
            sqlx::query_as!(
                Recipe,
                r#"
                SELECT * FROM recipes 
                WHERE (name ILIKE $1 OR description ILIKE $1) 
                AND tags && $2::text[]
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
                format!("%{}%", search_term),
                &tags.split(',').collect::<Vec<_>>(),
                limit,
                offset
            )
        } else {
            sqlx::query_as!(
                Recipe,
                r#"
                SELECT * FROM recipes 
                WHERE name ILIKE $1 OR description ILIKE $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                format!("%{}%", search_term),
                limit,
                offset
            )
        }
    } else if let Some(tags) = params.tags {
        sqlx::query_as!(
            Recipe,
            r#"
            SELECT * FROM recipes 
            WHERE tags && $1::text[]
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            &tags.split(',').collect::<Vec<_>>(),
            limit,
            offset
        )
    } else {
        sqlx::query_as!(
            Recipe,
            "SELECT * FROM recipes ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
    };

    match query.fetch_all(&importer.db_pool).await {
        Ok(recipes) => Ok(Json(recipes)),
        Err(e) => {
            error!("Failed to search recipes: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_recipe(
    State(importer): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> Result<Json<Recipe>, StatusCode> {
    match sqlx::query_as!(Recipe, "SELECT * FROM recipes WHERE id = $1", recipe_id)
        .fetch_optional(&importer.db_pool)
        .await
    {
        Ok(Some(recipe)) => Ok(Json(recipe)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get recipe: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_import_batches(
    State(state): State<AppState>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ImportBatch>>, StatusCode> {
    let jobs = state.import_jobs.read().await;
    let batches: Vec<ImportBatch> = jobs.values().cloned().collect();
    
    Ok(Json(batches))
}
