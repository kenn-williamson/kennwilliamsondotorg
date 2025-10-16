use anyhow::Result;
use bcrypt::verify;

use super::AuthService;
use crate::models::api::{AuthResponse, LoginRequest};
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

        // Check credentials table for password
        let creds_repo = self.credentials_repository.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Credentials repository not configured"))?;

        let credentials = creds_repo.find_by_user_id(user.id).await?;
        let password_hash = match credentials {
            Some(creds) => creds.password_hash,
            None => return Ok(None), // OAuth-only user, no password credentials
        };

        if !verify(&data.password, &password_hash)? {
            return Ok(None); // Invalid password
        }

        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;

        // Generate JWT token with roles and refresh token
        let token = self.jwt_service.generate_token(&user, &roles)?;
        let refresh_token =
            create_refresh_token(user.id, device_info, &*self.refresh_token_repository).await?;

        // Build fully populated user response
        let user_response = self.build_user_response_with_details(user, roles).await?;

        Ok(Some(AuthResponse {
            token,
            refresh_token,
            user: user_response,
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
    use crate::models::db::UserCredentials;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use crate::repositories::mocks::mock_user_credentials_repository::MockUserCredentialsRepository;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::services::auth::jwt::JwtService;
    use anyhow::Result;
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user_without_password(user_id: Uuid) -> crate::models::db::User {
        crate::models::db::User {
            id: user_id,
            email: "oauth@example.com".to_string(),
            display_name: "OAuth User".to_string(),
            slug: "oauth-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_credentials(user_id: Uuid, password: &str) -> UserCredentials {
        UserCredentials {
            user_id,
            password_hash: hash(password, DEFAULT_COST).unwrap(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        }
    }

    fn create_test_refresh_token() -> RefreshToken {
        crate::test_utils::RefreshTokenBuilder::new()
            .with_token_hash("test_token_hash".to_string())
            .without_device_info()
            .expires_in_days(7)
            .build()
    }

    #[tokio::test]
    async fn login_successful_with_valid_credentials() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();
        let user = crate::models::db::User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(2) // Called once for password check, once for has_credentials in response
            .returning(move |_| Ok(Some(create_test_credentials(user_id, "password123"))));

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

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.login(request, None).await?;

        assert!(result.is_some());
        let auth_response = result.unwrap();
        assert!(!auth_response.token.is_empty());
        assert!(!auth_response.refresh_token.is_empty());
        assert_eq!(auth_response.user.email, "test@example.com");

        Ok(())
    }

    #[tokio::test]
    async fn login_fails_with_invalid_password() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();
        let user = crate::models::db::User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .returning(move |_| Ok(Some(create_test_credentials(user_id, "correct_password"))));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

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
        let mut creds_repo = MockUserCredentialsRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();
        let user = crate::models::db::User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .returning(move |_| Ok(Some(create_test_credentials(user_id, "password123"))));

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

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.login(request, None).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_login_with_credentials_table_success() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();
        let user = crate::models::db::User {
            id: user_id,
            email: "creds@example.com".to_string(),
            display_name: "Creds User".to_string(),
            slug: "creds-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // User exists
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("creds@example.com"))
            .returning(move |_| Ok(Some(user.clone())));

        // User has credentials in credentials table
        creds_repo
            .expect_find_by_user_id()
            .times(2) // Called once for password check, once for has_credentials in response
            .returning(move |_| Ok(Some(create_test_credentials(user_id, "password123"))));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = LoginRequest {
            email: "creds@example.com".to_string(),
            password: "password123".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.login(request, None).await?;
        assert!(result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_login_oauth_only_user_rejects_password() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();

        // OAuth-only user exists
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("oauth@example.com"))
            .returning(move |_| Ok(Some(create_test_user_without_password(user_id))));

        // User has NO credentials (OAuth-only)
        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .returning(|_| Ok(None));

        let request = LoginRequest {
            email: "oauth@example.com".to_string(),
            password: "anypassword".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.login(request, None).await?;
        // Should return None (or error in updated implementation) for OAuth-only users
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_login_user_with_both_password_and_oauth() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();
        let user = crate::models::db::User {
            id: user_id,
            email: "both@example.com".to_string(),
            display_name: "Both User".to_string(),
            slug: "both-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // User has both password and OAuth
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("both@example.com"))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(2) // Called once for password check, once for has_credentials in response
            .returning(move |_| Ok(Some(create_test_credentials(user_id, "password123"))));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = LoginRequest {
            email: "both@example.com".to_string(),
            password: "password123".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.login(request, None).await?;
        assert!(result.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_login_wrong_password_with_credentials_table() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();

        let user_id = Uuid::new_v4();
        let user = crate::models::db::User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        user_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .returning(move |_| Ok(Some(create_test_credentials(user_id, "correct_password"))));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.login(request, None).await?;
        assert!(result.is_none());

        Ok(())
    }
}
