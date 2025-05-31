use axum::{extract::State, response::IntoResponse};
use prometheus::{Encoder, TextEncoder, Counter, Histogram, Gauge, Registry, Opts, HistogramOpts};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = Counter::new("http_requests_total", "Total HTTP requests").unwrap();
    static ref HTTP_REQUEST_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
    ).unwrap();
    static ref ACTIVE_CONNECTIONS: Gauge = Gauge::new("active_connections", "Active connections").unwrap();
    static ref DATABASE_CONNECTIONS: Gauge = Gauge::new("database_connections", "Database connections").unwrap();
}

pub struct MetricsService {
    registry: Registry,
}

impl MetricsService {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        // Register metrics
        let _ = registry.register(Box::new(HTTP_REQUESTS_TOTAL.clone()));
        let _ = registry.register(Box::new(HTTP_REQUEST_DURATION.clone()));
        let _ = registry.register(Box::new(ACTIVE_CONNECTIONS.clone()));
        let _ = registry.register(Box::new(DATABASE_CONNECTIONS.clone()));

        // Initialize with default values
        HTTP_REQUESTS_TOTAL.inc();
        ACTIVE_CONNECTIONS.set(1.0); // This would be dynamic in real implementation
        DATABASE_CONNECTIONS.set(1.0);

        Self { registry }
    }

    pub fn get_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        
        match encoder.encode_to_string(&metric_families) {
            Ok(metrics) => metrics,
            Err(_) => String::from("# Error encoding metrics\n"),
        }
    }

    pub fn record_request(&self) {
        HTTP_REQUESTS_TOTAL.inc();
    }

    pub fn record_request_duration(&self, duration: f64) {
        HTTP_REQUEST_DURATION.observe(duration);
    }

    pub fn set_active_connections(&self, count: f64) {
        ACTIVE_CONNECTIONS.set(count);
    }

    pub fn set_database_connections(&self, count: f64) {
        DATABASE_CONNECTIONS.set(count);
    }
}

pub async fn metrics_handler(State(state): State<crate::AppState>) -> impl IntoResponse {
    let metrics = state.metrics_service.get_metrics();
    (
        axum::http::StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4")],
        metrics,
    )
}
