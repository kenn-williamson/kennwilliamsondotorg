use anyhow::Result;
use bcrypt::verify;

use super::AuthService;
use crate::models::api::{AuthResponse, LoginRequest, UserResponse};
use crate::models::db::refresh_token::CreateRefreshToken;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;

impl AuthService {
    /// Login a user with email and password
    pub async fn login(
        &self,
        data: LoginRequest,
        device_info: Option<serde_json::Value>,
    ) -> Result<Option<AuthResponse>> {
        // Get user by email
        let user = self.user_repository.find_by_email(&data.email).await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User not found
        };

        // Verify password (return None if user has no password - OAuth-only user)
        let password_hash = match &user.password_hash {
            Some(hash) => hash,
            None => return Ok(None), // OAuth-only user, cannot login with password
        };

        if !verify(&data.password, password_hash)? {
            return Ok(None); // Invalid password
        }

        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;

        // Generate JWT token with roles and refresh token
        let token = self.jwt_service.generate_token(&user, &roles)?;
        let refresh_token =
            create_refresh_token(user.id, device_info, &*self.refresh_token_repository).await?;

        Ok(Some(AuthResponse {
            token,
            refresh_token,
            user: UserResponse::from_user_with_roles(user, roles),
        }))
    }
}

/// Create refresh token
async fn create_refresh_token(
    user_id: uuid::Uuid,
    device_info: Option<serde_json::Value>,
    refresh_token_repository: &dyn RefreshTokenRepository,
) -> Result<String> {
    // Generate random token
    let token = generate_refresh_token_string();
    let token_hash = hash_token(&token);

    // Set expiration (7 days)
    let expires_at = chrono::Utc::now() + chrono::Duration::days(7);

    // Create token data
    let token_data = CreateRefreshToken {
        user_id,
        token_hash,
        device_info,
        expires_at,
    };

    // Store in database
    refresh_token_repository.create_token(&token_data).await?;

    // Return plain token (not hash)
    Ok(token)
}

/// Generate refresh token string
fn generate_refresh_token_string() -> String {
    use rand::{rng, Rng};
    let mut token_bytes = [0u8; 32]; // 256 bits
    rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash token for storage
fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::refresh_token::RefreshToken;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::services::auth::jwt::JwtService;
    use anyhow::Result;
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user() -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: Some(hash("password123", DEFAULT_COST).unwrap()),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            real_name: None,
            google_user_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_refresh_token() -> RefreshToken {
        RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "test_token_hash".to_string(),
            device_info: None,
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_used_at: None,
        }
    }

    #[tokio::test]
    async fn login_successful_with_valid_credentials() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(|_| Ok(Some(create_test_user())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.login(request, None).await?;

        assert!(result.is_some());
        let auth_response = result.unwrap();
        assert!(!auth_response.token.is_empty());
        assert!(!auth_response.refresh_token.is_empty());
        assert_eq!(auth_response.user.email, "test@example.com");

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn login_fails_with_invalid_password() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(|_| Ok(Some(create_test_user())));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.login(request, None).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn login_fails_when_user_not_found() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("nonexistent@example.com"))
            .returning(|_| Ok(None));

        let request = LoginRequest {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.login(request, None).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn handles_database_error_during_user_lookup() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_find_by_email()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.login(request, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }

    #[tokio::test]
    async fn handles_database_error_during_refresh_token_creation() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(|_| Ok(Some(create_test_user())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.login(request, None).await;
        assert!(result.is_err());

        Ok(())
    }
}
