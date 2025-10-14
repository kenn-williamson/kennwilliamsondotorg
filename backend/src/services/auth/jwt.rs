use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::models::db::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub roles: Vec<String>, // User roles for RBAC
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

    pub fn generate_token(&self, user: &User, roles: &[String]) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(1); // 1 hour expiration with refresh token system

        let claims = Claims {
            sub: user.id.to_string(),
            roles: roles.to_vec(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn generates_and_verifies_token_with_roles() -> Result<()> {
        let jwt_service = JwtService::new("test-secret".to_string());
        let user = create_test_user();
        let roles = vec!["user".to_string(), "email-verified".to_string()];

        let token = jwt_service.generate_token(&user, &roles)?;
        let claims = jwt_service.verify_token(&token).await?;

        assert!(claims.is_some());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.roles, roles);

        Ok(())
    }

    #[tokio::test]
    async fn generates_token_with_empty_roles() -> Result<()> {
        let jwt_service = JwtService::new("test-secret".to_string());
        let user = create_test_user();
        let roles: Vec<String> = vec![];

        let token = jwt_service.generate_token(&user, &roles)?;
        let claims = jwt_service.verify_token(&token).await?;

        assert!(claims.is_some());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert!(claims.roles.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn generates_token_with_multiple_roles() -> Result<()> {
        let jwt_service = JwtService::new("test-secret".to_string());
        let user = create_test_user();
        let roles = vec![
            "user".to_string(),
            "email-verified".to_string(),
            "admin".to_string(),
        ];

        let token = jwt_service.generate_token(&user, &roles)?;
        let claims = jwt_service.verify_token(&token).await?;

        assert!(claims.is_some());
        let claims = claims.unwrap();
        assert_eq!(claims.roles.len(), 3);
        assert!(claims.roles.contains(&"user".to_string()));
        assert!(claims.roles.contains(&"email-verified".to_string()));
        assert!(claims.roles.contains(&"admin".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn token_verification_fails_with_wrong_secret() -> Result<()> {
        let jwt_service = JwtService::new("test-secret".to_string());
        let user = create_test_user();
        let roles = vec!["user".to_string()];

        let token = jwt_service.generate_token(&user, &roles)?;

        // Try to verify with different secret
        let wrong_jwt_service = JwtService::new("wrong-secret".to_string());
        let result = wrong_jwt_service.verify_token(&token).await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn preserves_role_order_in_token() -> Result<()> {
        let jwt_service = JwtService::new("test-secret".to_string());
        let user = create_test_user();
        let roles = vec![
            "zebra".to_string(),
            "admin".to_string(),
            "user".to_string(),
        ];

        let token = jwt_service.generate_token(&user, &roles)?;
        let claims = jwt_service.verify_token(&token).await?;

        assert!(claims.is_some());
        let claims = claims.unwrap();
        // Roles should maintain their order
        assert_eq!(claims.roles[0], "zebra");
        assert_eq!(claims.roles[1], "admin");
        assert_eq!(claims.roles[2], "user");

        Ok(())
    }
}
