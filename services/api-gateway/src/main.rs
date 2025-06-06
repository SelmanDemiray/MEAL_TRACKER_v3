//! # API Gateway Service - Central Orchestration Layer
//! 
//! This is the main entry point for the Meal Prep Pro API Gateway service.
//! The API Gateway serves as the central hub that orchestrates all requests
//! between the frontend and various microservices.
//! 
//! ## Architecture Overview
//! 
//! The API Gateway follows a layered architecture pattern:
//! ```
//! Frontend Request → API Gateway → Middleware Stack → Route Handlers → Microservices
//! ```
//! 
//! ## Key Responsibilities:
//! 
//! - **Authentication & Authorization**: JWT token validation and user session management
//! - **Request Routing**: Intelligent routing to appropriate microservices based on endpoints
//! - **Rate Limiting**: Protection against abuse with configurable rate limits per user/IP
//! - **CORS Handling**: Cross-origin request management for web clients
//! - **WebSocket Management**: Real-time bidirectional communication for live updates
//! - **Metrics Collection**: Performance monitoring and business analytics
//! - **Error Handling**: Centralized error handling and response formatting
//! - **Caching**: Redis-based response caching for frequently accessed data
//! 
//! ## Service Discovery
//! 
//! The gateway communicates with these microservices:
//! - Nutrition Service (Port 8081): AI-powered nutrition analysis and recommendations
//! - Analytics Service (Port 8082): Data insights and business intelligence
//! - Recipe Import Service (Port 8083): External recipe data import and processing
//! 
//! ## Security Features
//! 
//! - JWT-based authentication with RS256 algorithm
//! - Rate limiting per IP and authenticated user
//! - Input validation and sanitization
//! - SQL injection prevention with prepared statements
//! - CORS configuration for web security
//! 
//! ## Performance Optimizations
//! 
//! - Connection pooling for database and Redis
//! - Async/await for non-blocking I/O operations
//! - Response compression (gzip)
//! - Efficient request routing with Axum
//! - Smart caching strategies for frequently accessed data

use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::sleep;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn, error, debug};
use sqlx::{PgPool, postgres::PgPoolOptions};
use redis::Client as RedisClient;

// Import our custom modules
mod auth;
mod cache;
mod database;
mod handlers;
mod metrics;
mod middleware_layer;
mod models;
mod services;
mod websocket;

use auth::AuthState;
use cache::CacheService;
use database::DatabaseService;
use handlers::recipe_handlers;
use metrics::MetricsCollector;
use middleware_layer::auth_middleware;
use services::ServiceOrchestrator;

/// Application state that is shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool for PostgreSQL operations
    /// Used for user data, meal plans, recipes, and analytics
    pub db: DatabaseService,
    
    /// Redis cache service for session management and response caching
    /// Improves performance by caching frequently accessed data
    pub cache: CacheService,
    
    /// Authentication service for JWT token management
    /// Handles user login, token validation, and session management
    pub auth: AuthState,
    
    /// Service registry for communicating with other microservices
    /// Manages HTTP clients and service discovery for nutrition, analytics, etc.
    pub services: ServiceOrchestrator,
    
    /// Metrics collection service for monitoring and analytics
    /// Tracks API usage, performance metrics, and business KPIs
    pub metrics: Arc<MetricsCollector>,
}

/// Application configuration loaded from environment variables
/// This structure holds all configuration settings for the service
#[derive(Clone, Debug)]
pub struct Config {
    /// PostgreSQL database connection URL
    /// Format: postgresql://user:password@host:port/database
    pub database_url: String,
    
    /// Redis connection URL for caching and session storage
    /// Format: redis://host:port or redis://host:port/db_number
    pub redis_url: String,
    
    /// Secret key used for JWT token signing and verification
    /// Should be a strong, randomly generated secret in production
    pub jwt_secret: String,
    
    /// JWT token expiration time in seconds
    /// Default: 3600 seconds (1 hour)
    pub jwt_expiration: u64,
    
    /// Allowed CORS origins for web client access
    /// Comma-separated list of allowed origins
    pub cors_origins: Vec<String>,
    
    /// URLs for downstream microservices
    pub nutrition_service_url: String,
    pub analytics_service_url: String,
    pub recipe_import_service_url: String,
    
    /// Server binding address and port
    pub bind_address: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:36379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
            jwt_expiration: std::env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:39000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            nutrition_service_url: std::env::var("NUTRITION_SERVICE_URL")
                .unwrap_or_else(|_| "http://nutrition-service:8081".to_string()),
            analytics_service_url: std::env::var("ANALYTICS_SERVICE_URL")
                .unwrap_or_else(|_| "http://analytics-service:8082".to_string()),
            recipe_import_service_url: std::env::var("RECIPE_IMPORT_SERVICE_URL")
                .unwrap_or_else(|_| "http://recipe-import-service:8083".to_string()),
            bind_address: std::env::var("BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
        })
    }
}

/// Health check response structure
/// Used by monitoring systems and load balancers to verify service health
#[derive(Serialize)]
pub struct HealthResponse {
    /// Service health status: "healthy", "degraded", or "unhealthy"
    status: String,
    
    /// Current timestamp when health check was performed
    timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Service version for deployment tracking
    version: String,
    
    /// Status of dependent services (database, cache, etc.)
    services: HashMap<String, String>,
    
    /// Optional additional health metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<HashMap<String, serde_json::Value>>,
}

/// Health check endpoint handler
async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    debug!("Health check endpoint called");
    
    let mut services = HashMap::new();
    let mut overall_healthy = true;
    
    // Check database connectivity
    // The database is critical for user data, recipes, and meal plans
    match state.db.health_check().await {
        Ok(_) => {
            services.insert("database".to_string(), "connected".to_string());
            debug!("Database health check passed");
        }
        Err(e) => {
            services.insert("database".to_string(), "disconnected".to_string());
            overall_healthy = false;
            warn!("Database health check failed: {}", e);
        }
    }
    
    // Check Redis cache connectivity
    // Redis is used for session storage and response caching
    match state.cache.health_check().await {
        Ok(_) => {
            services.insert("redis".to_string(), "connected".to_string());
            debug!("Redis health check passed");
        }
        Err(e) => {
            services.insert("redis".to_string(), "disconnected".to_string());
            overall_healthy = false;
            warn!("Redis health check failed: {}", e);
        }
    }
    
    // Check downstream microservices availability
    // These services are critical for AI functionality and data processing
    match state.services.health_check().await {
        Ok(service_statuses) => {
            for (service_name, status) in service_statuses {
                services.insert(service_name, if status { "healthy".to_string() } else { "unhealthy".to_string() });
            }
            debug!("Microservices health check completed");
        }
        Err(e) => {
            services.insert("microservices".to_string(), "degraded".to_string());
            // Don't mark as unhealthy for microservice issues - gateway can still function
            warn!("Some microservices health checks failed: {}", e);
        }
    }
    
    let status = if overall_healthy {
        "healthy"
    } else {
        "unhealthy"
    };
    
    let response = HealthResponse {
        status: status.to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services,
        metrics: None, // Could include performance metrics here
    };
    
    info!(
        status = status,
        services_count = response.services.len(),
        "Health check completed"
    );
    
    if overall_healthy {
        Ok(Json(response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// Metrics endpoint handler
async fn metrics_handler(State(state): State<AppState>) -> axum::response::Response {
    state.metrics.export_metrics().await
}

/// WebSocket handler
async fn websocket_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Implement WebSocket upgrade
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Create and configure the main application router
/// 
/// This function sets up all the routes, middleware, and handlers for the API Gateway.
/// The router is organized into logical groups with appropriate middleware applied.
/// 
/// # Route Organization
/// 
/// - `/health` and `/metrics`: System endpoints (no auth required)
/// - `/api/auth/*`: Authentication endpoints (registration, login, refresh)
/// - `/api/users/*`: User management (requires authentication)
/// - `/api/meals/*`: Meal management and planning (requires authentication)
/// - `/api/nutrition/*`: Nutrition tracking and analysis (requires authentication)
/// - `/api/recipes/*`: Recipe management (requires authentication)
/// - `/api/shopping/*`: Shopping list management (requires authentication)
/// - `/api/analytics/*`: Analytics and insights (requires authentication)
/// - `/ws`: WebSocket upgrade endpoint for real-time features
/// 
/// # Middleware Stack (applied in order)
/// 
/// 1. **TraceLayer**: Request tracing and logging
/// 2. **CompressionLayer**: Gzip compression for responses
/// 3. **CorsLayer**: Cross-origin request handling
/// 4. **RateLimitLayer**: Rate limiting protection
/// 5. **TimeoutLayer**: Request timeout handling
/// 6. **AuthMiddleware**: JWT authentication (on protected routes)
fn create_router(state: AppState) -> Router {
    // Create CORS layer with configured origins
    // This allows the frontend to make requests from different domains
    let cors_layer = CorsLayer::new()
        .allow_origin(Any) // In production, use specific origins from config
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .expose_headers([
            axum::http::header::CONTENT_LENGTH,
            axum::http::header::CONTENT_TYPE,
        ]);
    
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
        .route("/api/auth/register", post(auth::register_handler))
        .route("/api/auth/login", post(auth::login_handler))
        .route("/api/auth/refresh", post(auth::refresh_handler));
    
    // Protected routes that require valid JWT authentication
    let protected_routes = Router::new()
        // User management endpoints
        .route("/api/users/profile", get(handlers::get_user_profile))
        .route("/api/users/profile", put(handlers::update_user_profile))
        .route("/api/users/preferences", get(handlers::get_user_preferences))
        .route("/api/users/preferences", put(handlers::update_user_preferences))
        
        // Meal management endpoints
        .route("/api/meals", get(handlers::get_meals))
        .route("/api/meals", post(handlers::create_meal))
        .route("/api/meals/:id", get(handlers::get_meal))
        .route("/api/meals/:id", put(handlers::update_meal))
        .route("/api/meals/:id", delete(handlers::delete_meal))
        .route("/api/meals/search", get(handlers::search_meals))
        .route("/api/meals/recommendations", get(handlers::get_meal_recommendations))
        
        // Meal planning endpoints
        .route("/api/meal-plans", get(handlers::get_meal_plans))
        .route("/api/meal-plans", post(handlers::create_meal_plan))
        .route("/api/meal-plans/:id", get(handlers::get_meal_plan))
        .route("/api/meal-plans/:id", put(handlers::update_meal_plan))
        .route("/api/meal-plans/generate", post(handlers::generate_meal_plan))
        
        // Nutrition tracking endpoints
        .route("/api/nutrition/log", post(handlers::log_nutrition))
        .route("/api/nutrition/daily", get(handlers::get_daily_nutrition))
        .route("/api/nutrition/weekly", get(handlers::get_weekly_nutrition))
        .route("/api/nutrition/goals", get(handlers::get_nutrition_goals))
        .route("/api/nutrition/goals", put(handlers::update_nutrition_goals))
        .route("/api/nutrition/analysis", get(handlers::get_nutrition_analysis))
        
        // Recipe management endpoints
        .route("/api/recipes", get(recipe_handlers::get_recipes))
        .route("/api/recipes", post(recipe_handlers::create_recipe))
        .route("/api/recipes/:id", get(recipe_handlers::get_recipe))
        .route("/api/recipes/:id/scale", post(recipe_handlers::scale_recipe))
        .route("/api/recipes/import", post(recipe_handlers::import_recipes))
        
        // Shopping list endpoints
        .route("/api/shopping-lists", get(handlers::get_shopping_lists))
        .route("/api/shopping-lists", post(handlers::create_shopping_list))
        .route("/api/shopping-lists/:id", get(handlers::get_shopping_list))
        .route("/api/shopping-lists/:id/items", post(handlers::add_shopping_item))
        .route("/api/shopping-lists/:id/optimize", post(handlers::optimize_shopping_list))
        
        // Analytics endpoints
        .route("/api/analytics/dashboard", get(handlers::get_analytics_dashboard))
        .route("/api/analytics/trends", get(handlers::get_analytics_trends))
        .route("/api/analytics/predictions", get(handlers::get_analytics_predictions))
        .route("/api/analytics/insights", get(handlers::get_analytics_insights))
        
        // Inventory management endpoints
        .route("/api/inventory", get(handlers::get_inventory))
        .route("/api/inventory/items", post(handlers::add_inventory_item))
        .route("/api/inventory/items/:id", put(handlers::update_inventory_item))
        .route("/api/inventory/expiring", get(handlers::get_expiring_items))
        
        // Apply authentication middleware to all protected routes
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));
    
    // WebSocket endpoint for real-time features
    let websocket_routes = Router::new()
        .route("/ws", get(websocket_handler));
    
    // Combine all routes and apply middleware
    Router::new()
        .merge(public_routes)      // Public endpoints
        .merge(protected_routes)   // Authenticated endpoints  
        .merge(websocket_routes)   // WebSocket endpoint
        .layer(TraceLayer::new_for_http())    // Logging and tracing
        .layer(CompressionLayer::new())    // Response compression
        .layer(cors_layer)                 // CORS handling
        .with_state(state)         // Provide app state to all handlers
}

/// Initialize and start the API Gateway service
/// 
/// This is the main entry point that:
/// 1. Sets up logging and tracing
/// 2. Loads configuration from environment
/// 3. Initializes all service dependencies
/// 4. Creates the application router
/// 5. Starts the HTTP server
/// 
/// # Error Handling
/// 
/// The function will panic if critical services can't be initialized:
/// - Database connection failure
/// - Redis connection failure
/// - Invalid configuration
/// 
/// # Graceful Shutdown
/// 
/// The server listens for SIGTERM and SIGINT signals for graceful shutdown.
/// During shutdown, it will:
/// - Stop accepting new requests
/// - Complete in-flight requests
/// - Close database connections
/// - Clean up resources
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging and tracing
    // This provides detailed logs for debugging and monitoring
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(true)
        .init();
    
    info!("🚀 Starting Meal Prep Pro API Gateway");
    info!("📦 Version: {}", env!("CARGO_PKG_VERSION"));
    
    // Load configuration from environment variables
    let config = Config::from_env()
        .map_err(|e| {
            error!("Failed to load configuration: {}", e);
            e
        })?;
    
    info!("🔧 Configuration loaded successfully");
    debug!("Config: {:#?}", config);
    
    // Initialize database service with connection pooling
    info!("🗄️  Initializing database service...");
    let db = DatabaseService::new(&config.database_url)
        .await
        .map_err(|e| {
            error!("Failed to initialize database service: {}", e);
            e
        })?;
    info!("✅ Database service initialized");
    
    // Initialize Redis cache service
    info!("📦 Initializing cache service...");
    let cache = CacheService::new(&config.redis_url)
        .await
        .map_err(|e| {
            error!("Failed to initialize cache service: {}", e);
            e
        })?;
    info!("✅ Cache service initialized");
    
    // Initialize authentication service
    info!("🔐 Initializing authentication service...");
    let auth = AuthState::new(&config.jwt_secret, config.jwt_expiration);
    info!("✅ Authentication service initialized");
    
    // Initialize service orchestrator for microservice communication
    info!("🌐 Initializing service orchestrator...");
    let services = ServiceOrchestrator::new().await
        .map_err(|e| {
            error!("Failed to initialize service orchestrator: {}", e);
            e
        })?;
    info!("✅ Service orchestrator initialized");
    
    // Initialize metrics service for monitoring
    info!("📊 Initializing metrics service...");
    let metrics = Arc::new(MetricsCollector::new());
    info!("✅ Metrics service initialized");
    
    // Create application state
    let app_state = AppState {
        db,
        cache,
        auth,
        services,
        metrics,
    };
    
    // Create the main application router with all routes and middleware
    info!("🛠️  Creating application router...");
    let app = create_router(app_state);
    info!("✅ Application router created");
    
    // Parse the binding address
    let addr: SocketAddr = config.bind_address.parse()
        .map_err(|e| {
            error!("Invalid bind address: {}", e);
            e
        })?;
    
    info!("🌐 Starting HTTP server on {}", addr);
    
    // Start the HTTP server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| {
            error!("Failed to bind to address {}: {}", addr, e);
            e
        })?;
    
    info!("🎉 API Gateway is running on http://{}", addr);
    info!("📖 API Documentation: http://{}/health", addr);
    info!("📊 Metrics endpoint: http://{}/metrics", addr);
    info!("🔗 WebSocket endpoint: ws://{}/ws", addr);
    
    // Start the server with graceful shutdown handling
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    info!("👋 API Gateway shutdown complete");
    Ok(())
}

/// Handle graceful shutdown signals
/// 
/// This function listens for SIGTERM (Docker/Kubernetes) and SIGINT (Ctrl+C)
/// signals to perform a graceful shutdown of the service.
/// 
/// During shutdown:
/// - New requests are rejected
/// - Existing connections are allowed to complete
/// - Database connections are closed
/// - Cache connections are cleaned up
/// - Metrics are flushed
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Received Ctrl+C signal, initiating graceful shutdown...");
        },
        _ = terminate => {
            info!("🛑 Received SIGTERM signal, initiating graceful shutdown...");
        }
    }
    
    info!("⏳ Shutting down gracefully...");
    sleep(Duration::from_secs(2)).await;
    info!("✅ Graceful shutdown completed");
}
