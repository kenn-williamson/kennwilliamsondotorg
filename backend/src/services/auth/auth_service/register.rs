use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};

use super::slug::generate_slug;
use super::AuthService;
use crate::models::api::{AuthResponse, CreateUserRequest};
use crate::models::db::refresh_token::CreateRefreshToken;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::traits::user_repository::CreateUserData;

impl AuthService {
    /// Register a new user
    ///
    /// If email verification is configured (verification_token_repository and email_service),
    /// sends a verification email to the user.
    pub async fn register(
        &self,
        data: CreateUserRequest,
        device_info: Option<serde_json::Value>,
        frontend_url: Option<&str>,
    ) -> Result<AuthResponse> {
        // Generate slug from display_name
        let slug = generate_slug(&data.display_name, &*self.user_repository).await?;

        // Hash password
        let password_hash = hash(&data.password, DEFAULT_COST)?;

        // Create user data
        let user_data = CreateUserData {
            email: data.email.clone(),
            password_hash: password_hash.clone(),
            display_name: data.display_name.clone(),
            slug,
        };

        // Create user with credentials and preferences in a single transaction
        // This ensures atomicity - all tables created or none at all
        let user = self
            .user_repository
            .create_user_with_auth_data(&user_data, password_hash)
            .await?;

        // Send verification email if configured
        if let (Some(verification_repo), Some(email_service), Some(url)) = (
            &self.verification_token_repository,
            &self.email_service,
            frontend_url,
        ) {
            // Generate secure token
            let token = generate_verification_token();
            let token_hash = hash_verification_token(&token);

            // Create token data with 24-hour expiration
            use chrono::{Duration, Utc};
            use crate::repositories::traits::verification_token_repository::CreateVerificationTokenData;

            let expires_at = Utc::now() + Duration::hours(24);
            let token_data = CreateVerificationTokenData {
                user_id: user.id,
                token_hash,
                expires_at,
            };

            // Store hashed token in database
            verification_repo.create_token(&token_data).await?;

            // Send verification email with plain token
            email_service
                .send_verification_email(&user.email, Some(&user.display_name), &token, url)
                .await?;
        }

        // Get user roles
        let roles = self.user_repository.get_user_roles(user.id).await?;

        // Generate JWT token with roles and refresh token
        let token = self.jwt_service.generate_token(&user, &roles)?;
        let refresh_token =
            create_refresh_token(user.id, device_info, &*self.refresh_token_repository).await?;

        // Build fully populated user response
        let user_response = self.build_user_response_with_details(user, roles).await?;

        Ok(AuthResponse {
            token,
            refresh_token,
            user: user_response,
        })
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

/// Generate a secure random verification token (32 bytes = 256 bits)
fn generate_verification_token() -> String {
    use rand::{rng, Rng};
    let mut token_bytes = [0u8; 32];
    rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash verification token using SHA-256 for storage
fn hash_verification_token(token: &str) -> String {
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
    use chrono::Utc;
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
            .with_token_hash("test_token_hash".to_string())
            .without_device_info()
            .expires_in_days(7)
            .build()
    }

    #[tokio::test]
    async fn registers_user_successfully() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false)); // Slug doesn't exist

        user_repo
            .expect_create_user_with_auth_data()
            .times(1)
            .returning(|_, _| Ok(create_test_user()));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.register(request, None, None).await?;

        assert!(!result.token.is_empty());
        assert!(!result.refresh_token.is_empty());
        assert_eq!(result.user.email, "test@example.com");
        assert_eq!(result.user.display_name, "Test User");
        assert_eq!(result.user.roles, vec!["user"]);

        Ok(())
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn handles_database_error_during_user_creation() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let _jwt_service = JwtService::new("test-secret".to_string());

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false)); // Slug doesn't exist

        user_repo
            .expect_create_user_with_auth_data()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.register(request, None, None).await;
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
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false)); // Slug doesn't exist

        user_repo
            .expect_create_user_with_auth_data()
            .times(1)
            .returning(|_, _| Ok(create_test_user()));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );
        let result = auth_service.register(request, None, None).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn sends_verification_email_when_email_service_configured() -> Result<()> {
        use crate::models::db::VerificationToken;
        use crate::repositories::mocks::mock_verification_token_repository::MockVerificationTokenRepository;
        use crate::services::email::MockEmailService;

        let user_id = Uuid::new_v4();
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let mut verification_repo = MockVerificationTokenRepository::new();
        let email_service = MockEmailService::new();

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false));

        user_repo
            .expect_create_user_with_auth_data()
            .times(1)
            .returning(move |_, _| {
                let mut user = create_test_user();
                user.id = user_id;
                Ok(user)
            });

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        // Expect verification token to be created
        verification_repo
            .expect_create_token()
            .times(1)
            .returning(|token_data| {
                Ok(VerificationToken {
                    id: Uuid::new_v4(),
                    user_id: token_data.user_id,
                    token_hash: token_data.token_hash.clone(),
                    expires_at: token_data.expires_at,
                    created_at: Utc::now(),
                })
            });

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .verification_token_repository(Box::new(verification_repo))
            .email_service(Box::new(email_service.clone()))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .register(request, None, Some("https://kennwilliamson.org"))
            .await?;

        // Assert registration succeeded
        assert!(!result.token.is_empty());
        assert!(!result.refresh_token.is_empty());
        assert_eq!(result.user.email, "test@example.com");

        // Assert email was sent
        assert_eq!(email_service.count(), 1);
        let sent_emails = email_service.get_sent_emails();
        assert_eq!(sent_emails[0].to_email, "test@example.com");
        assert_eq!(sent_emails[0].to_name, Some("Test User".to_string()));
        assert_eq!(sent_emails[0].frontend_url, "https://kennwilliamson.org");
        assert!(!sent_emails[0].verification_token.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn does_not_send_verification_email_when_email_service_not_configured() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();

        // Setup mock expectations (same as original test)
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false));

        user_repo
            .expect_create_user_with_auth_data()
            .times(1)
            .returning(|_, _| Ok(create_test_user()));

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        // Use legacy constructor (no email service)
        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(refresh_repo),
            "test-secret".to_string(),
        );

        // Should succeed without sending email
        let result = auth_service.register(request, None, None).await?;

        assert!(!result.token.is_empty());
        assert!(!result.refresh_token.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_register_creates_user_with_auth_data_transactionally() -> Result<()> {
        let user_id = Uuid::new_v4();
        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();

        // Setup mock expectations
        user_repo
            .expect_slug_exists()
            .times(1)
            .returning(|_| Ok(false));

        // Expect the atomic create_user_with_auth_data call
        user_repo
            .expect_create_user_with_auth_data()
            .times(1)
            .returning(move |_, _| {
                let mut user = create_test_user();
                user.id = user_id;
                Ok(user)
            });

        user_repo
            .expect_get_user_roles()
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        refresh_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.register(request, None, None).await?;

        assert!(!result.token.is_empty());
        assert!(!result.refresh_token.is_empty());
        assert_eq!(result.user.email, "test@example.com");

        Ok(())
    }
}
