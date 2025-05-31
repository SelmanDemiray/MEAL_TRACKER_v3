use axum::{
    extract::{Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;
use hyper::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};
use std::sync::Arc;
use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Module declarations
mod auth;
mod cache;
mod database;
mod metrics;
mod services;
mod websocket;
mod middleware_layer;
mod models;
mod handlers;

// Use statements for internal modules
use auth::AuthService;
use cache::CacheService;
use database::DatabaseManager;
use metrics::MetricsService;
use services::ServiceClient;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub auth_service: Arc<AuthService>,
    pub cache_service: Arc<CacheService>,
    pub metrics_service: Arc<MetricsService>,
    pub service_client: Arc<ServiceClient>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://mealprep:mealprep_secure_2024@postgres:5432/mealprep".to_string());
    
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://redis:6379".to_string());

    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-here".to_string());

    // Initialize services
    let db = Arc::new(DatabaseManager::new(&database_url).await?);
    let auth_service = Arc::new(AuthService::new(jwt_secret));
    let cache_service = Arc::new(CacheService::new(&redis_url).await?);
    let metrics_service = Arc::new(MetricsService::new());
    let service_client = Arc::new(ServiceClient::new());

    // Run database migrations
    db.migrate().await?;

    let app_state = AppState {
        db,
        auth_service,
        cache_service,
        metrics_service,
        service_client,
    };

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Build the application router
    let app = Router::new()
        // Health check endpoints
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // Authentication routes
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout))
        .route("/api/auth/me", get(handlers::auth::get_current_user))
        
        // User management routes
        .route("/api/users/profile", get(handlers::users::get_profile))
        .route("/api/users/profile", put(handlers::users::update_profile))
        .route("/api/users/preferences", get(handlers::users::get_preferences))
        .route("/api/users/preferences", put(handlers::users::update_preferences))
        
        // Recipe and meal routes
        .route("/api/recipes", get(handlers::recipes::list_recipes))
        .route("/api/recipes", post(handlers::recipes::create_recipe))
        .route("/api/recipes/:id", get(handlers::recipes::get_recipe))
        .route("/api/recipes/:id", put(handlers::recipes::update_recipe))
        .route("/api/recipes/:id", axum::routing::delete(handlers::recipes::delete_recipe))
        
        // Meal planning routes
        .route("/api/meal-plans", get(handlers::meal_plans::list_meal_plans))
        .route("/api/meal-plans", post(handlers::meal_plans::create_meal_plan))
        
        // Nutrition routes
        .route("/api/nutrition/analyze", post(handlers::nutrition::analyze_nutrition))
        .route("/api/nutrition/goals", get(handlers::nutrition::get_goals))
        .route("/api/nutrition/goals", put(handlers::nutrition::update_goals))
        
        // WebSocket endpoint
        .route("/ws", get(websocket::websocket_handler))
        
        // Metrics endpoint
        .route("/metrics", get(metrics::metrics_handler))
        
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(middleware_layer::MetricsMiddleware::new())
                .into_inner()
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("API Gateway server starting on 0.0.0.0:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn readiness_check(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // TODO: Add actual readiness checks for database, redis, etc.
    Json(serde_json::json!({
        "status": "ready",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
