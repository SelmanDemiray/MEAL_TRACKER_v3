use prometheus::{Counter, Histogram, HistogramOpts, Gauge, Registry, Encoder, TextEncoder};
use std::sync::Arc;
use axum::response::Response;
use axum::http::StatusCode;

#[derive(Clone)]
pub struct MetricsCollector {
    registry: Arc<Registry>,
    request_counter: Counter,
    request_duration: Histogram,
    active_connections: Gauge,
    error_counter: Counter,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());
        
        let request_counter = Counter::new(
            "http_requests_total",
            "Total number of HTTP requests"
        ).expect("Failed to create request counter");
        
        let request_duration = Histogram::with_opts(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds"
            )
        ).expect("Failed to create request duration histogram");
        
        let active_connections = Gauge::new(
            "active_connections",
            "Number of active connections"
        ).expect("Failed to create active connections gauge");
        
        let error_counter = Counter::new(
            "http_errors_total",
            "Total number of HTTP errors"
        ).expect("Failed to create error counter");

        // Register metrics
        registry.register(Box::new(request_counter.clone())).unwrap();
        registry.register(Box::new(request_duration.clone())).unwrap();
        registry.register(Box::new(active_connections.clone())).unwrap();
        registry.register(Box::new(error_counter.clone())).unwrap();

        Self {
            registry,
            request_counter,
            request_duration,
            active_connections,
            error_counter,
        }
    }

    pub fn record_request(&self) {
        self.request_counter.inc();
    }

    pub fn record_request_duration(&self, duration: f64) {
        self.request_duration.observe(duration);
    }

    pub fn record_error(&self) {
        self.error_counter.inc();
    }

    pub fn set_active_connections(&self, count: f64) {
        self.active_connections.set(count);
    }

    pub async fn export_metrics(&self) -> Response {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();

        match encoder.encode(&metric_families, &mut buffer) {
            Ok(_) => {
                let response_body = String::from_utf8(buffer).unwrap_or_default();
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "text/plain; version=0.0.4; charset=utf-8")
                    .body(response_body.into())
                    .unwrap()
            }
            Err(_) => {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error generating metrics".into())
                    .unwrap()
            }
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
