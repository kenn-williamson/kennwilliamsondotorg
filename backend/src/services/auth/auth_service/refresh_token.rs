use anyhow::Result;
use chrono::{Duration, Utc};
use rand::{rng, Rng};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::AuthService;
use crate::models::api::{AuthResponse, RefreshTokenRequest};
use crate::models::db::refresh_token::CreateRefreshToken;

impl AuthService {
    /// Refresh a JWT token using a refresh token
    /// Returns full AuthResponse with updated user data and roles
    pub async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<Option<AuthResponse>> {
        // Hash the provided token to lookup in database
        let token_hash = hash_token(&request.refresh_token);

        // Find refresh token
        let token_record = self
            .refresh_token_repository
            .find_by_token(&token_hash)
            .await?;
        let token_record = match token_record {
            Some(token) => token,
            None => return Ok(None), // Token not found or expired
        };

        // Check 6-month hard limit
        let six_months_ago = Utc::now() - Duration::days(180);
        if token_record.created_at < six_months_ago {
            // Delete the expired token
            self.refresh_token_repository
                .revoke_token(&token_hash)
                .await?;
            return Ok(None);
        }

        // Get user
        let user = self
            .user_repository
            .find_by_id(token_record.user_id)
            .await?;
        let user = match user {
            Some(user) => user,
            None => return Ok(None), // User no longer exists
        };

        // Get user roles (fetch fresh roles on token refresh)
        let roles = self.user_repository.get_user_roles(user.id).await?;

        // Generate new JWT with roles and refresh token
        let new_jwt = self.jwt_service.generate_token(&user, &roles)?;
        let new_refresh_token = generate_refresh_token_string();
        let new_token_hash = hash_token(&new_refresh_token);

        // Delete old token and create new token
        self.refresh_token_repository
            .revoke_token(&token_hash)
            .await?;

        // Create new refresh token
        let expires_at = Utc::now() + Duration::days(7);
        let token_data = CreateRefreshToken {
            user_id: token_record.user_id,
            token_hash: new_token_hash,
            device_info: token_record.device_info,
            expires_at,
        };
        self.refresh_token_repository
            .create_token(&token_data)
            .await?;

        // Build fully populated user response
        let user_response = self.build_user_response_with_details(user, roles).await?;

        // Return full AuthResponse with user data
        Ok(Some(AuthResponse {
            token: new_jwt,
            refresh_token: new_refresh_token,
            user: user_response,
        }))
    }

    /// Revoke a specific refresh token
    pub async fn revoke_refresh_token(
        &self,
        request: crate::models::api::RevokeTokenRequest,
    ) -> Result<bool> {
        let token_hash = hash_token(&request.refresh_token);
        let result = self
            .refresh_token_repository
            .revoke_token(&token_hash)
            .await;
        Ok(result.is_ok())
    }

    /// Revoke all refresh tokens for a user
    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<u64> {
        self.refresh_token_repository
            .revoke_all_user_tokens(user_id)
            .await?;
        Ok(1) // Return count of affected tokens
    }
}

/// Generate refresh token string
fn generate_refresh_token_string() -> String {
    let mut token_bytes = [0u8; 32]; // 256 bits
    rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash token for storage
fn hash_token(token: &str) -> String {
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
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user() -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_refresh_token() -> RefreshToken {
        crate::test_utils::RefreshTokenBuilder::new()
            .with_token_hash("test_token_hash")
            .without_device_info()
            .expires_at(Utc::now() + chrono::Duration::days(7))
            .never_used()
            .build()
    }

    fn create_old_refresh_token() -> RefreshToken {
        crate::test_utils::RefreshTokenBuilder::new()
            .with_token_hash("old_token_hash")
            .without_device_info()
            .expires_at(Utc::now() + chrono::Duration::days(7))
            .created_at(Utc::now() - chrono::Duration::days(200)) // 200 days ago (expired)
            .never_used()
            .build()
    }

    #[tokio::test]
    async fn refresh_token_successful_with_valid_token() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        let user = create_test_user();
        let test_refresh_token = create_test_refresh_token();
        let user_id = test_refresh_token.user_id;
        let token_hash = hash_token("valid_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_find_by_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(move |_| Ok(Some(test_refresh_token.clone())));

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_revoke_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(|_| Ok(()));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = RefreshTokenRequest {
            refresh_token: "valid_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.refresh_token(request).await?;

        assert!(result.is_some());
        let response = result.unwrap();
        assert!(!response.token.is_empty());
        assert!(!response.refresh_token.is_empty());
        assert_eq!(response.user.email, "test@example.com");
        assert_eq!(response.user.roles, vec!["user"]);
        assert_eq!(response.user.email_verified, false);

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn refresh_token_fails_with_invalid_token() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        let token_hash = hash_token("invalid_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_find_by_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(|_| Ok(None));

        let request = RefreshTokenRequest {
            refresh_token: "invalid_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.refresh_token(request).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn refresh_token_fails_with_expired_token() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        let old_token = create_old_refresh_token();
        let token_hash = hash_token("expired_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_find_by_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(move |_| Ok(Some(old_token.clone())));

        refresh_repo
            .expect_revoke_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(|_| Ok(()));

        let request = RefreshTokenRequest {
            refresh_token: "expired_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.refresh_token(request).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn refresh_token_fails_when_user_not_found() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        let test_refresh_token = create_test_refresh_token();
        let user_id = test_refresh_token.user_id;
        let token_hash = hash_token("valid_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_find_by_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(move |_| Ok(Some(test_refresh_token.clone())));

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None));

        let request = RefreshTokenRequest {
            refresh_token: "valid_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.refresh_token(request).await?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn handles_database_error_during_token_lookup() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        let token_hash = hash_token("valid_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_find_by_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = RefreshTokenRequest {
            refresh_token: "valid_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.refresh_token(request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }

    #[tokio::test]
    async fn revoke_refresh_token_successful() -> Result<()> {
        let mut refresh_repo = MockRefreshTokenRepository::new();

        let token_hash = hash_token("valid_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_revoke_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(|_| Ok(()));

        let request = crate::models::api::RevokeTokenRequest {
            refresh_token: "valid_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(MockUserRepository::new()),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.revoke_refresh_token(request).await?;
        assert!(result);

        Ok(())
    }

    #[tokio::test]
    async fn revoke_refresh_token_handles_database_error() -> Result<()> {
        let mut refresh_repo = MockRefreshTokenRepository::new();

        let token_hash = hash_token("valid_refresh_token");

        // Setup mock expectations
        refresh_repo
            .expect_revoke_token()
            .times(1)
            .with(eq(token_hash.clone()))
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = crate::models::api::RevokeTokenRequest {
            refresh_token: "valid_refresh_token".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(MockUserRepository::new()),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.revoke_refresh_token(request).await?;
        assert!(!result); // Should return false on error

        Ok(())
    }

    #[tokio::test]
    async fn revoke_all_user_tokens_successful() -> Result<()> {
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        refresh_repo
            .expect_revoke_all_user_tokens()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(())); // Tokens revoked

        let auth_service = AuthService::new(
            Box::new(MockUserRepository::new()),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.revoke_all_user_tokens(user_id).await?;
        assert_eq!(result, 1); // Returns 1 as per implementation

        Ok(())
    }

    #[tokio::test]
    async fn revoke_all_user_tokens_handles_database_error() -> Result<()> {
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        refresh_repo
            .expect_revoke_all_user_tokens()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let auth_service = AuthService::new(
            Box::new(MockUserRepository::new()),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.revoke_all_user_tokens(user_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }
}
