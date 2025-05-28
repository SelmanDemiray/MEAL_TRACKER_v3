use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::AppState;

pub async fn auth_middleware(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement authentication middleware
    Ok(next.run(request).await)
}
