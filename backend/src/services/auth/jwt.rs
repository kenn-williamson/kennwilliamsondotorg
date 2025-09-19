use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{Duration, Utc};

use crate::models::db::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,
    pub iat: i64,
}

#[derive(Clone)]
pub struct JwtService {
    jwt_secret: String,
}

impl JwtService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<Claims>> {
        let validation = Validation::default();
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )?;

        Ok(Some(token_data.claims))
    }

    pub fn generate_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(1); // 1 hour expiration with refresh token system

        let claims = Claims {
            sub: user.id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }
}
