use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use crate::AppState;
use crate::auth::{validate_jwt};

pub async fn auth_middleware(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    if path.starts_with("/api/auth")
        || path == "/health"
        || path == "/metrics"
        || path == "/ws"
    {
        return Ok(next.run(request).await);
    }

    let auth_header = request.headers().get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .or(auth_header);

    if let Some(token) = token {
        if token == "demo-token" {
            // Allow demo token
            return Ok(next.run(request).await);
        }
        if validate_jwt(token).is_some() {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
