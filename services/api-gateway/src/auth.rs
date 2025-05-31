use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub username: String,
    pub email: String,
    pub exp: usize,   // expiration timestamp
    pub iat: usize,   // issued at timestamp
    pub jti: String,  // JWT ID for tracking
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash)
            .map_err(|e| anyhow!("Failed to verify password: {}", e))
    }

    pub fn generate_tokens(&self, user_id: Uuid, username: String, email: String) -> Result<TokenPair> {
        let now = Utc::now();
        let access_token_exp = now + Duration::hours(1);
        let refresh_token_exp = now + Duration::days(30);

        // Access token claims
        let access_claims = Claims {
            sub: user_id.to_string(),
            username: username.clone(),
            email: email.clone(),
            exp: access_token_exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };

        // Refresh token claims (longer expiry)
        let refresh_claims = Claims {
            sub: user_id.to_string(),
            username,
            email,
            exp: refresh_token_exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };

        let encoding_key = EncodingKey::from_secret(self.jwt_secret.as_ref());
        
        let access_token = encode(&Header::default(), &access_claims, &encoding_key)
            .map_err(|e| anyhow!("Failed to encode access token: {}", e))?;
        
        let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)
            .map_err(|e| anyhow!("Failed to encode refresh token: {}", e))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: 3600, // 1 hour in seconds
            token_type: "Bearer".to_string(),
        })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let validation = Validation::default();

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| anyhow!("Invalid token: {}", e))?;

        // Check if token is expired
        let now = Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(anyhow!("Token expired"));
        }

        Ok(token_data.claims)
    }

    pub fn extract_bearer_token(auth_header: &str) -> Result<&str> {
        if !auth_header.starts_with("Bearer ") {
            return Err(anyhow!("Invalid authorization header format"));
        }
        
        Ok(&auth_header[7..])
    }

    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenPair> {
        let claims = self.validate_token(refresh_token)?;
        
        // Generate new token pair
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|e| anyhow!("Invalid user ID in token: {}", e))?;
        
        self.generate_tokens(user_id, claims.username, claims.email)
    }

    pub fn generate_token(&self, user_id: Uuid, username: &str, role: &str) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as usize;
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + 24 * 60 * 60, // 24 hours
            iat: now,
            username: username.to_string(),
            role: role.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn extract_token_from_header(&self, auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub tokens: TokenPair,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub email_verified: bool,
    pub created_at: String,
}

// Password validation
pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(anyhow!("Password must be at least 8 characters long"));
    }
    
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(anyhow!("Password must contain at least one uppercase letter"));
    }
    
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(anyhow!("Password must contain at least one lowercase letter"));
    }
    
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(anyhow!("Password must contain at least one number"));
    }
    
    Ok(())
}

// Email validation
pub fn validate_email(email: &str) -> Result<()> {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    });
    if !regex.is_match(email) {
        return Err(anyhow!("Invalid email format"));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        assert!(validate_password("Password123").is_ok());
        assert!(validate_password("weak").is_err());
        assert!(validate_password("NoNumbers!").is_err());
        assert!(validate_password("nouppercase123").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_password_hashing() {
        let auth_service = AuthService::new("test-secret".to_string());
        let password = "TestPassword123";
        
        let hash = auth_service.hash_password(password).unwrap();
        assert!(auth_service.verify_password(password, &hash).unwrap());
        assert!(!auth_service.verify_password("WrongPassword", &hash).unwrap());
    }
}
