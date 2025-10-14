use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::AuthService;
use crate::models::api::{SendVerificationEmailResponse, VerifyEmailResponse};
use crate::repositories::traits::verification_token_repository::CreateVerificationTokenData;

impl AuthService {
    /// Send verification email to user
    /// Generates a secure token, stores hash in DB, sends email with link
    pub async fn send_verification_email(
        &self,
        user_id: Uuid,
        frontend_url: &str,
    ) -> Result<SendVerificationEmailResponse> {
        // Require verification token repository
        let verification_repo = self
            .verification_token_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Verification token repository not configured"))?;

        // Require email service
        let email_service = self
            .email_service
            .as_ref()
            .ok_or_else(|| anyhow!("Email service not configured"))?;

        // Get user to get email and display name
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        // Generate secure token
        let token = generate_verification_token();
        let token_hash = hash_verification_token(&token);

        // Create token data with 24-hour expiration
        let expires_at = Utc::now() + Duration::hours(24);
        let token_data = CreateVerificationTokenData {
            user_id,
            token_hash,
            expires_at,
        };

        // Store hashed token in database
        verification_repo.create_token(&token_data).await?;

        // Send verification email with plain token
        email_service
            .send_verification_email(&user.email, Some(&user.display_name), &token, frontend_url)
            .await?;

        Ok(SendVerificationEmailResponse {
            message: "Verification email sent. Please check your inbox.".to_string(),
        })
    }

    /// Verify email with token
    /// Validates token, assigns email-verified role, deletes all user tokens
    pub async fn verify_email(&self, token: &str) -> Result<VerifyEmailResponse> {
        // Require verification token repository
        let verification_repo = self
            .verification_token_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Verification token repository not configured"))?;

        // Hash the token to look it up
        let token_hash = hash_verification_token(token);

        // Find token (repository already filters out expired tokens)
        let verification_token = verification_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| anyhow!("Invalid or expired verification token"))?;

        // Assign email-verified role to user
        self.user_repository
            .add_role_to_user(verification_token.user_id, "email-verified")
            .await?;

        // Delete all verification tokens for this user (cleanup)
        verification_repo
            .delete_all_user_tokens(verification_token.user_id)
            .await?;

        Ok(VerifyEmailResponse {
            message: "Email verified successfully!".to_string(),
            email_verified: true,
        })
    }
}

/// Generate a secure random token (32 bytes = 256 bits)
fn generate_verification_token() -> String {
    let mut token_bytes = [0u8; 32];
    rand::rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash token using SHA-256 for storage
fn hash_verification_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::{User, VerificationToken};
    use crate::repositories::mocks::{
        MockRefreshTokenRepository, MockUserRepository, MockVerificationTokenRepository,
    };
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    fn create_test_user(id: Uuid) -> User {
        User {
            id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_generate_verification_token_is_64_chars_hex() {
        let token = generate_verification_token();
        assert_eq!(token.len(), 64); // 32 bytes = 64 hex chars
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_verification_token_is_random() {
        let token1 = generate_verification_token();
        let token2 = generate_verification_token();
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_hash_verification_token_is_deterministic() {
        let token = "test_token_123";
        let hash1 = hash_verification_token(token);
        let hash2 = hash_verification_token(token);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 = 64 hex chars
    }

    #[test]
    fn test_hash_verification_token_different_inputs() {
        let hash1 = hash_verification_token("token1");
        let hash2 = hash_verification_token("token2");
        assert_ne!(hash1, hash2);
    }

    #[tokio::test]
    async fn test_send_verification_email_generates_and_stores_token() -> Result<()> {
        let user_id = Uuid::new_v4();
        let mut user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut verification_repo = MockVerificationTokenRepository::new();
        let email_service = crate::services::email::MockEmailService::new();

        // Expect user lookup
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(mockall::predicate::eq(user_id))
            .returning(move |_| Ok(Some(create_test_user(user_id))));

        // Expect token creation
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

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .verification_token_repository(Box::new(verification_repo))
            .email_service(Box::new(email_service))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .send_verification_email(user_id, "https://example.com")
            .await;

        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_email_success_assigns_role_and_deletes_tokens() -> Result<()> {
        let user_id = Uuid::new_v4();
        let token = generate_verification_token();
        let token_hash = hash_verification_token(&token);

        let mut user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut verification_repo = MockVerificationTokenRepository::new();
        let email_service = crate::services::email::MockEmailService::new();

        // Expect token lookup
        verification_repo
            .expect_find_by_token_hash()
            .times(1)
            .with(mockall::predicate::eq(token_hash.clone()))
            .returning(move |_| {
                Ok(Some(VerificationToken {
                    id: Uuid::new_v4(),
                    user_id,
                    token_hash: token_hash.clone(),
                    expires_at: Utc::now() + Duration::hours(24),
                    created_at: Utc::now(),
                }))
            });

        // Expect role assignment
        user_repo
            .expect_add_role_to_user()
            .times(1)
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq("email-verified"),
            )
            .returning(|_, _| Ok(()));

        // Expect token deletion
        verification_repo
            .expect_delete_all_user_tokens()
            .times(1)
            .with(mockall::predicate::eq(user_id))
            .returning(|_| Ok(1));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .verification_token_repository(Box::new(verification_repo))
            .email_service(Box::new(email_service))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.verify_email(&token).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.email_verified, true);

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_email_expired_token_fails() -> Result<()> {
        let _user_id = Uuid::new_v4();
        let token = generate_verification_token();
        let _token_hash = hash_verification_token(&token);

        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut verification_repo = MockVerificationTokenRepository::new();
        let email_service = crate::services::email::MockEmailService::new();

        // Return expired token
        verification_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(move |_| Ok(None)); // find_by_token_hash already filters expired

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .verification_token_repository(Box::new(verification_repo))
            .email_service(Box::new(email_service))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.verify_email(&token).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid"));

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_email_invalid_token_fails() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut verification_repo = MockVerificationTokenRepository::new();
        let email_service = crate::services::email::MockEmailService::new();

        // Token not found
        verification_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(|_| Ok(None));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .verification_token_repository(Box::new(verification_repo))
            .email_service(Box::new(email_service))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.verify_email("invalid_token").await;
        assert!(result.is_err());

        Ok(())
    }
}
