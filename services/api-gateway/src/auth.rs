use anyhow::{anyhow, Result};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use uuid::Uuid;
use validator::Validate;

use crate::{AppState, models::{User, CreateUserRequest, LoginRequest}};

#[derive(Clone)]
pub struct AuthState {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: i64,
}

impl AuthState {
    pub fn new(secret: &str, expiration_seconds: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            expiration_seconds: expiration_seconds as i64,
        }
    }

    pub fn generate_token(&self, user_id: Uuid, email: &str, role: &str) -> Result<String> {
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: (Utc::now() + Duration::seconds(self.expiration_seconds)).timestamp(),
            iat: Utc::now().timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to generate token: {}", e))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub email: String,
    pub role: String,
    pub exp: i64,     // expiration time
    pub iat: i64,     // issued at
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
    pub expires_in: i64,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    info!("User registration attempt for email: {}", request.email);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        warn!("Registration validation failed: {:?}", validation_errors);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    match state.db.get_user_by_email(&request.email).await {
        Ok(_) => {
            warn!("Registration failed: user already exists for email {}", request.email);
            return Err(StatusCode::CONFLICT);
        }
        Err(_) => {} // User doesn't exist, continue
    }

    // Hash password
    let password_hash = hash(&request.password, DEFAULT_COST)
        .map_err(|e| {
            error!("Password hashing failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create user
    let user = User {
        id: Uuid::new_v4(),
        username: request.username.clone(),
        email: request.email.clone(),
        password_hash,
        email_verified: false,
        is_active: true,
        role: "user".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Save to database
    state.db.create_user(&user).await.map_err(|e| {
        error!("Failed to create user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate JWT token
    let token = state.auth.generate_token(user.id, &user.email, &user.role)
        .map_err(|e| {
            error!("Token generation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("User registered successfully: {}", user.email);

    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: user.role,
        },
        expires_in: state.auth.expiration_seconds,
    };

    Ok(Json(response))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    info!("Login attempt for email: {}", request.email);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        warn!("Login validation failed: {:?}", validation_errors);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Get user from database
    let user = state.db.get_user_by_email(&request.email).await
        .map_err(|_| {
            warn!("Login failed: user not found for email {}", request.email);
            StatusCode::UNAUTHORIZED
        })?;

    // Verify password
    let password_valid = verify(&request.password, &user.password_hash)
        .map_err(|e| {
            error!("Password verification error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !password_valid {
        warn!("Login failed: invalid password for email {}", request.email);
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !user.is_active {
        warn!("Login failed: user account disabled for email {}", request.email);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT token
    let token = state.auth.generate_token(user.id, &user.email, &user.role)
        .map_err(|e| {
            error!("Token generation failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("User logged in successfully: {}", user.email);

    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: user.role,
        },
        expires_in: state.auth.expiration_seconds,
    };

    Ok(Json(response))
}

pub async fn refresh_handler(
    State(_state): State<AppState>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // TODO: Implement token refresh logic
    Err(StatusCode::NOT_IMPLEMENTED)
}

fn generate_jwt(user: &UserInfo) -> Result<String, StatusCode> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-key".to_string());
    
    let claims = Claims {
        sub: user.id.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
        iat: Utc::now().timestamp(),
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
