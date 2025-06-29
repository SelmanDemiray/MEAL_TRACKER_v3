// Authentication module placeholder

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key_for_development".to_string())
}

pub fn validate_jwt(token: &str) -> Option<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).ok()
}
