use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::AppState;
use std::time::Instant;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MetricsMiddleware;

impl MetricsMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for MetricsMiddleware {
    type Service = MetricsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MetricsService { inner }
    }
}

#[derive(Clone)]
pub struct MetricsService<S> {
    inner: S,
}

impl<S> Service<Request> for MetricsService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let start = Instant::now();
            let response = inner.call(req).await?;
            let duration = start.elapsed();
            
            // Record metrics here
            tracing::info!("Request processed in {:?}", duration);
            
            Ok(response)
        })
    }
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement authentication middleware
    Ok(next.run(request).await)
}

pub async fn metrics_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    tracing::info!("Request took {:?}", duration);
    
    Ok(response)
}
