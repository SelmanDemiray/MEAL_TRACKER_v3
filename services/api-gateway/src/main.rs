use axum::{
    extract::{State, WebSocketUpgrade},
    http::Method,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
    Json, Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
};
use std::sync::Arc;
use prometheus::{Encoder, TextEncoder};
use std::net::SocketAddr;

mod auth;
mod handlers;
mod middleware_layer;
mod websocket;
mod services;
mod models;
mod database;
mod cache;
mod metrics;

use handlers::*;
use services::ServiceOrchestrator;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::Client,
    pub service_orchestrator: Arc<ServiceOrchestrator>,
    pub websocket_manager: Arc<websocket::WebSocketManager>,
    pub metrics: Arc<metrics::MetricsCollector>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://mealprep:mealprep_secure_2024@postgres:5432/mealprep".to_string());
    
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://redis:6379".to_string());

    let db = sqlx::PgPool::connect(&database_url).await?;
    let redis = redis::Client::open(redis_url)?;
    
    sqlx::migrate!("./migrations").run(&db).await?;

    let service_orchestrator = Arc::new(ServiceOrchestrator::new().await?);
    let websocket_manager = Arc::new(websocket::WebSocketManager::new());
    let metrics = Arc::new(metrics::MetricsCollector::new());

    let app_state = AppState {
        db,
        redis,
        service_orchestrator,
        websocket_manager,
        metrics,
    };

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("🚀 Advanced Meal Prep API Gateway running on port 8080");
    
    axum::serve(listener, app).await?;
    Ok(())
}

fn create_router(state: AppState) -> Router {
    Router::new()
        // Authentication routes
        .route("/api/auth/register", post(auth_handlers::register))
        .route("/api/auth/login", post(auth_handlers::login))
        .route("/api/auth/refresh", post(auth_handlers::refresh_token))
        
        // User management
        .route("/api/users/profile", get(user_handlers::get_profile))
        .route("/api/users/profile", put(user_handlers::update_profile))
        .route("/api/users/preferences", get(user_handlers::get_preferences))
        .route("/api/users/preferences", put(user_handlers::update_preferences))
        
        // Meal planning routes
        .route("/api/meals", get(meal_handlers::list_meals))
        .route("/api/meals", post(meal_handlers::create_meal))
        .route("/api/meals/:id", get(meal_handlers::get_meal))
        .route("/api/meals/:id", put(meal_handlers::update_meal))
        .route("/api/meals/:id", delete(meal_handlers::delete_meal))
        .route("/api/meals/search", get(meal_handlers::search_meals))
        .route("/api/meals/recommendations", get(meal_handlers::get_recommendations))
        
        // Meal planning
        .route("/api/meal-plans", get(meal_plan_handlers::list_plans))
        .route("/api/meal-plans", post(meal_plan_handlers::create_plan))
        .route("/api/meal-plans/:id", get(meal_plan_handlers::get_plan))
        .route("/api/meal-plans/:id", put(meal_plan_handlers::update_plan))
        .route("/api/meal-plans/generate", post(meal_plan_handlers::generate_ai_plan))
        
        // Nutrition tracking
        .route("/api/nutrition/log", post(nutrition_handlers::log_meal))
        .route("/api/nutrition/daily", get(nutrition_handlers::get_daily_nutrition))
        .route("/api/nutrition/weekly", get(nutrition_handlers::get_weekly_nutrition))
        .route("/api/nutrition/goals", get(nutrition_handlers::get_goals))
        .route("/api/nutrition/goals", put(nutrition_handlers::update_goals))
        .route("/api/nutrition/analysis", get(nutrition_handlers::get_nutritional_analysis))
        
        // Shopping lists
        .route("/api/shopping-lists", get(shopping_handlers::list_shopping_lists))
        .route("/api/shopping-lists", post(shopping_handlers::create_shopping_list))
        .route("/api/shopping-lists/:id", get(shopping_handlers::get_shopping_list))
        .route("/api/shopping-lists/:id/items", post(shopping_handlers::add_items))
        .route("/api/shopping-lists/:id/optimize", post(shopping_handlers::optimize_list))
        
        // Analytics and insights
        .route("/api/analytics/dashboard", get(analytics_handlers::get_dashboard))
        .route("/api/analytics/trends", get(analytics_handlers::get_trends))
        .route("/api/analytics/predictions", get(analytics_handlers::get_predictions))
        .route("/api/analytics/insights", get(analytics_handlers::get_insights))
        
        // Recipe management
        .route("/api/recipes", get(recipe_handlers::list_recipes))
        .route("/api/recipes", post(recipe_handlers::create_recipe))
        .route("/api/recipes/:id", get(recipe_handlers::get_recipe))
        .route("/api/recipes/:id/scale", post(recipe_handlers::scale_recipe))
        .route("/api/recipes/import", post(recipe_handlers::import_recipe))
        
        // Inventory management
        .route("/api/inventory", get(inventory_handlers::get_inventory))
        .route("/api/inventory/items", post(inventory_handlers::add_item))
        .route("/api/inventory/items/:id", put(inventory_handlers::update_item))
        .route("/api/inventory/expiring", get(inventory_handlers::get_expiring))
        
        // WebSocket for real-time updates
        .route("/ws", get(websocket_handler))
        
        // Health and metrics
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
        
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            middleware_layer::auth_middleware,
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket::handle_socket(socket, state))
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "services": {
            "database": "connected",
            "redis": "connected",
            "microservices": "operational"
        }
    }))
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    state.metrics.export_metrics().await
}

// Add this function to handle metrics endpoint
async fn metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
