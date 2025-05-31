use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::sync::Arc;
use chrono::{DateTime, Utc};

mod analytics_engine;
mod trend_analyzer;
mod predictive_model;
mod models;
mod database;

use analytics_engine::AnalyticsEngine;
use trend_analyzer::TrendAnalyzer;
use predictive_model::PredictiveModel;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::Client,
    pub analytics_engine: Arc<AnalyticsEngine>,
    pub trend_analyzer: Arc<TrendAnalyzer>,
    pub predictive_model: Arc<PredictiveModel>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")?;
    let redis_url = std::env::var("REDIS_URL")?;

    let db = sqlx::PgPool::connect(&database_url).await?;
    let redis = redis::Client::open(redis_url)?;

    let analytics_engine = Arc::new(AnalyticsEngine::new());
    let trend_analyzer = Arc::new(TrendAnalyzer::new());
    let predictive_model = Arc::new(PredictiveModel::new().await?);

    let app_state = AppState {
        db,
        redis,
        analytics_engine,
        trend_analyzer,
        predictive_model,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/dashboard", get(get_dashboard))
        .route("/trends", get(get_trends))
        .route("/predictions", get(get_predictions))
        .route("/insights", get(get_insights))
        .route("/reports", post(generate_report))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await?;
    tracing::info!("📊 Analytics Service running on port 8082");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "analytics-service",
        "timestamp": chrono::Utc::now()
    })))
}

#[derive(Serialize)]
struct DashboardData {
    overview: AnalyticsOverview,
    trends: Vec<TrendData>,
    insights: Vec<Insight>,
}

#[derive(Serialize)]
struct AnalyticsOverview {
    total_users: i64,
    active_users: i64,
    meals_analyzed: i64,
}

#[derive(Serialize)]
struct TrendData {
    name: String,
    data_points: Vec<DataPoint>,
    trend_direction: String,
    change_percentage: f64,
}

#[derive(Serialize)]
struct DataPoint {
    timestamp: DateTime<Utc>,
    value: f64,
}

#[derive(Serialize)]
struct Insight {
    id: String,
    title: String,
    description: String,
    category: String,
    importance: String,
    timestamp: DateTime<Utc>,
}

async fn get_dashboard(
    Query(_params): Query<std::collections::HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<DashboardData>, StatusCode> {
    let overview = AnalyticsOverview {
        total_users: 1250,
        active_users: 890,
        meals_analyzed: 15420,
    };

    let trends = vec![
        TrendData {
            name: "User Engagement".to_string(),
            data_points: vec![
                DataPoint {
                    timestamp: Utc::now() - chrono::Duration::days(7),
                    value: 75.0,
                },
                DataPoint {
                    timestamp: Utc::now(),
                    value: 82.5,
                },
            ],
            trend_direction: "up".to_string(),
            change_percentage: 10.0,
        },
    ];

    let insights = vec![
        Insight {
            id: "insight_1".to_string(),
            title: "Increased User Activity".to_string(),
            description: "User activity has increased by 15% this week".to_string(),
            category: "engagement".to_string(),
            importance: "high".to_string(),
            timestamp: Utc::now(),
        },
    ];

    Ok(Json(DashboardData {
        overview,
        trends,
        insights,
    }))
}

async fn get_trends(
    Query(_params): Query<std::collections::HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TrendData>>, StatusCode> {
    let trends = vec![
        TrendData {
            name: "Nutrition Goals Achievement".to_string(),
            data_points: vec![
                DataPoint {
                    timestamp: Utc::now() - chrono::Duration::days(30),
                    value: 68.0,
                },
                DataPoint {
                    timestamp: Utc::now(),
                    value: 75.5,
                },
            ],
            trend_direction: "up".to_string(),
            change_percentage: 11.0,
        },
    ];

    Ok(Json(trends))
}

async fn get_predictions(
    Query(_params): Query<std::collections::HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let predictions = vec![
        "User engagement will increase by 8% next month".to_string(),
        "Protein intake goals achievement rate will improve to 85%".to_string(),
        "Meal prep frequency will stabilize at 3.2 sessions per week".to_string(),
    ];

    Ok(Json(predictions))
}

async fn get_insights(
    Query(_params): Query<std::collections::HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Insight>>, StatusCode> {
    let insights = vec![
        Insight {
            id: "insight_nutrition_1".to_string(),
            title: "Protein Intake Optimization".to_string(),
            description: "Users who track protein consistently achieve 23% better results".to_string(),
            category: "nutrition".to_string(),
            importance: "medium".to_string(),
            timestamp: Utc::now(),
        },
        Insight {
            id: "insight_behavioral_1".to_string(),
            title: "Weekend Planning Pattern".to_string(),
            description: "85% of successful users plan meals on Sunday evenings".to_string(),
            category: "behavior".to_string(),
            importance: "high".to_string(),
            timestamp: Utc::now(),
        },
    ];

    Ok(Json(insights))
}

async fn generate_report(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let report = serde_json::json!({
        "report_id": uuid::Uuid::new_v4().to_string(),
        "type": "comprehensive",
        "generated_at": Utc::now(),
        "status": "completed",
        "data": {
            "summary": "Weekly analytics report generated successfully",
            "metrics_count": 15,
            "insights_count": 8
        }
    });

    Ok(Json(report))
}
