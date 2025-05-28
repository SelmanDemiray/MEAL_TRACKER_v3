use axum::response::IntoResponse;

pub struct MetricsCollector {
    // TODO: Implement metrics collection
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn export_metrics(&self) -> impl IntoResponse {
        "# HELP metrics placeholder\n# TYPE metrics placeholder\n"
    }
}
