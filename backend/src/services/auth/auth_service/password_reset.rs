use anyhow::{Result, anyhow};
use chrono::{Duration, Utc};
use rand::Rng;
use sha2::{Digest, Sha256};

use super::AuthService;
use crate::models::api::{ForgotPasswordResponse, ResetPasswordResponse};
use crate::repositories::traits::password_reset_token_repository::CreatePasswordResetTokenData;

impl AuthService {
    /// Send password reset email to user
    /// Generates a secure token, stores hash in DB, sends email with link
    /// Returns success even for non-existent users (prevents user enumeration)
    pub async fn send_password_reset_email(
        &self,
        email: &str,
        frontend_url: &str,
    ) -> Result<ForgotPasswordResponse> {
        // Require password reset token repository
        let password_reset_repo = self
            .password_reset_token_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Password reset token repository not configured"))?;

        // Require email service
        let email_service = self
            .email_service
            .as_ref()
            .ok_or_else(|| anyhow!("Email service not configured"))?;

        // Look up user by email
        let user = self.user_repository.find_by_email(email).await?;

        // Only send email if user exists (but always return same response)
        if let Some(user) = user {
            // Generate secure token (32 bytes = 64 hex chars)
            let token = generate_password_reset_token();
            let token_hash = hash_password_reset_token(&token);

            // Create token data with 1-hour expiration
            let expires_at = Utc::now() + Duration::hours(1);
            let token_data = CreatePasswordResetTokenData {
                user_id: user.id,
                token_hash,
                expires_at,
            };

            // Store hashed token in database
            password_reset_repo.create_token(&token_data).await?;

            // Send password reset email using template
            use crate::services::email::templates::{
                Email, EmailTemplate, PasswordResetEmailTemplate,
            };

            let template =
                PasswordResetEmailTemplate::new(&user.display_name, &token, frontend_url);

            let html_body = template.render_html()?;
            let text_body = template.render_plain_text();
            let subject = template.subject();

            let email = Email::builder()
                .to(&user.email)
                .subject(subject)
                .text_body(text_body)
                .html_body(html_body)
                .build()?;

            email_service.send_email(email).await?;
        }

        // Return same response regardless of whether user exists (security)
        Ok(ForgotPasswordResponse {
            message:
                "If an account exists with that email, you will receive a password reset link."
                    .to_string(),
        })
    }

    /// Reset password with token
    /// Validates token, updates password, marks token as used, revokes all refresh tokens
    pub async fn reset_password_with_token(
        &self,
        token: &str,
        new_password: &str,
    ) -> Result<ResetPasswordResponse> {
        // Require password reset token repository
        let password_reset_repo = self
            .password_reset_token_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Password reset token repository not configured"))?;

        // Hash token to look it up
        let token_hash = hash_password_reset_token(token);

        // Find token (repo filters expired + used automatically)
        let reset_token = password_reset_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| anyhow!("Invalid or expired password reset token"))?;

        // Hash new password with bcrypt
        let password_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;

        // Update user password
        self.user_repository
            .update_password(reset_token.user_id, &password_hash)
            .await?;

        // Mark token as used (single-use token)
        password_reset_repo.mark_token_used(&token_hash).await?;

        // Revoke all user refresh tokens (force re-login for security)
        self.refresh_token_repository
            .revoke_all_user_tokens(reset_token.user_id)
            .await?;

        Ok(ResetPasswordResponse {
            message: "Password reset successfully. You can now login with your new password."
                .to_string(),
        })
    }
}

/// Generate a secure random token (32 bytes = 256 bits = 64 hex chars)
fn generate_password_reset_token() -> String {
    let mut token_bytes = [0u8; 32];
    rand::rng().fill(&mut token_bytes);
    hex::encode(token_bytes)
}

/// Hash token using SHA-256 for storage
fn hash_password_reset_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::{PasswordResetToken, User};
    use crate::repositories::mocks::{
        MockPasswordResetTokenRepository, MockRefreshTokenRepository, MockUserRepository,
    };
    use chrono::Utc;
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

    // Test 1: Token generated is 64 hex chars (32 bytes)
    #[test]
    fn test_generate_password_reset_token_is_64_chars_hex() {
        let token = generate_password_reset_token();
        assert_eq!(token.len(), 64); // 32 bytes = 64 hex chars
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // Test 2: Token generation is random
    #[test]
    fn test_generate_password_reset_token_is_random() {
        let token1 = generate_password_reset_token();
        let token2 = generate_password_reset_token();
        assert_ne!(token1, token2);
    }

    // Test 3: Token hash is deterministic
    #[test]
    fn test_hash_password_reset_token_is_deterministic() {
        let token = "test_token_123";
        let hash1 = hash_password_reset_token(token);
        let hash2 = hash_password_reset_token(token);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 = 64 hex chars
    }

    // Test 4: Token hash different for different inputs
    #[test]
    fn test_hash_password_reset_token_different_inputs() {
        let hash1 = hash_password_reset_token("token1");
        let hash2 = hash_password_reset_token("token2");
        assert_ne!(hash1, hash2);
    }

    // Test 5: send_password_reset_email generates and stores token
    #[tokio::test]
    async fn test_send_password_reset_email_generates_and_stores_token() -> Result<()> {
        let user_id = Uuid::new_v4();
        let mut user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();
        let email_service = crate::services::email::MockEmailService::new();

        // Expect user lookup
        user_repo
            .expect_find_by_email()
            .times(1)
            .with(mockall::predicate::eq("test@example.com"))
            .returning(move |_| Ok(Some(create_test_user(user_id))));

        // Expect token creation
        password_reset_repo
            .expect_create_token()
            .times(1)
            .returning(|token_data| {
                Ok(PasswordResetToken {
                    id: Uuid::new_v4(),
                    user_id: token_data.user_id,
                    token_hash: token_data.token_hash.clone(),
                    expires_at: token_data.expires_at,
                    used_at: None,
                    created_at: Utc::now(),
                })
            });

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .email_service(Box::new(email_service))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .send_password_reset_email("test@example.com", "https://example.com")
            .await;

        assert!(result.is_ok());

        Ok(())
    }

    // Test 6: send_password_reset_email fails if email service not configured
    #[tokio::test]
    async fn test_send_password_reset_email_fails_without_email_service() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let password_reset_repo = MockPasswordResetTokenRepository::new();

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .send_password_reset_email("test@example.com", "https://example.com")
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Email service not configured")
        );

        Ok(())
    }

    // Test 7: reset_password_with_token with valid token succeeds
    #[tokio::test]
    async fn test_reset_password_with_token_valid_token_succeeds() -> Result<()> {
        let user_id = Uuid::new_v4();
        let token = generate_password_reset_token();
        let token_hash = hash_password_reset_token(&token);

        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        // Expect token lookup
        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .with(mockall::predicate::eq(token_hash.clone()))
            .returning(move |_| {
                Ok(Some(PasswordResetToken {
                    id: Uuid::new_v4(),
                    user_id,
                    token_hash: token_hash.clone(),
                    expires_at: Utc::now() + Duration::hours(1),
                    used_at: None,
                    created_at: Utc::now(),
                }))
            });

        // Expect password update
        user_repo
            .expect_update_password()
            .times(1)
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::always(),
            )
            .returning(|_, _| Ok(()));

        // Expect token marked as used
        password_reset_repo
            .expect_mark_token_used()
            .times(1)
            .returning(|_| Ok(true));

        // Expect all refresh tokens revoked
        refresh_repo
            .expect_revoke_all_user_tokens()
            .times(1)
            .with(mockall::predicate::eq(user_id))
            .returning(|_| Ok(()));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token(&token, "newpassword123")
            .await;

        assert!(result.is_ok());

        Ok(())
    }

    // Test 8: reset_password_with_token with invalid token fails
    #[tokio::test]
    async fn test_reset_password_with_token_invalid_token_fails() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        // Token not found
        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(|_| Ok(None));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token("invalid_token", "newpassword123")
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid or expired")
        );

        Ok(())
    }

    // Test 9: reset_password_with_token with expired token fails
    #[tokio::test]
    async fn test_reset_password_with_token_expired_token_fails() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        // Repository filters expired tokens automatically - returns None
        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(|_| Ok(None));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token("expired_token", "newpassword123")
            .await;

        assert!(result.is_err());

        Ok(())
    }

    // Test 10: reset_password_with_token with already-used token fails
    #[tokio::test]
    async fn test_reset_password_with_token_used_token_fails() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        // Repository filters used tokens automatically (used_at IS NULL check)
        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(|_| Ok(None));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token("used_token", "newpassword123")
            .await;

        assert!(result.is_err());

        Ok(())
    }

    // Test 11: reset_password_with_token hashes password with bcrypt
    #[tokio::test]
    async fn test_reset_password_with_token_hashes_password() -> Result<()> {
        let user_id = Uuid::new_v4();
        let token = generate_password_reset_token();
        let token_hash = hash_password_reset_token(&token);

        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(move |_| {
                Ok(Some(PasswordResetToken {
                    id: Uuid::new_v4(),
                    user_id,
                    token_hash: token_hash.clone(),
                    expires_at: Utc::now() + Duration::hours(1),
                    used_at: None,
                    created_at: Utc::now(),
                }))
            });

        // Verify password is hashed (not plaintext)
        user_repo
            .expect_update_password()
            .times(1)
            .withf(|_, password_hash| {
                // BCrypt hashes start with $2b$ or $2a$ or $2y$
                password_hash.starts_with("$2")
            })
            .returning(|_, _| Ok(()));

        password_reset_repo
            .expect_mark_token_used()
            .times(1)
            .returning(|_| Ok(true));

        refresh_repo
            .expect_revoke_all_user_tokens()
            .times(1)
            .returning(|_| Ok(()));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token(&token, "newpassword123")
            .await;

        assert!(result.is_ok());

        Ok(())
    }

    // Test 12: reset_password_with_token revokes all user refresh tokens
    #[tokio::test]
    async fn test_reset_password_with_token_revokes_refresh_tokens() -> Result<()> {
        let user_id = Uuid::new_v4();
        let token = generate_password_reset_token();
        let token_hash = hash_password_reset_token(&token);

        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(move |_| {
                Ok(Some(PasswordResetToken {
                    id: Uuid::new_v4(),
                    user_id,
                    token_hash: token_hash.clone(),
                    expires_at: Utc::now() + Duration::hours(1),
                    used_at: None,
                    created_at: Utc::now(),
                }))
            });

        user_repo
            .expect_update_password()
            .times(1)
            .returning(|_, _| Ok(()));

        password_reset_repo
            .expect_mark_token_used()
            .times(1)
            .returning(|_| Ok(true));

        // Verify all refresh tokens are revoked
        refresh_repo
            .expect_revoke_all_user_tokens()
            .times(1)
            .with(mockall::predicate::eq(user_id))
            .returning(|_| Ok(()));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token(&token, "newpassword123")
            .await;

        assert!(result.is_ok());

        Ok(())
    }

    // Test 13: reset_password_with_token marks token as used
    #[tokio::test]
    async fn test_reset_password_with_token_marks_token_used() -> Result<()> {
        let user_id = Uuid::new_v4();
        let token = generate_password_reset_token();
        let token_hash = hash_password_reset_token(&token);
        let token_hash_for_expect = token_hash.clone();

        let mut user_repo = MockUserRepository::new();
        let mut refresh_repo = MockRefreshTokenRepository::new();
        let mut password_reset_repo = MockPasswordResetTokenRepository::new();

        password_reset_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(move |_| {
                Ok(Some(PasswordResetToken {
                    id: Uuid::new_v4(),
                    user_id,
                    token_hash: token_hash.clone(),
                    expires_at: Utc::now() + Duration::hours(1),
                    used_at: None,
                    created_at: Utc::now(),
                }))
            });

        user_repo
            .expect_update_password()
            .times(1)
            .returning(|_, _| Ok(()));

        // Verify token is marked as used
        password_reset_repo
            .expect_mark_token_used()
            .times(1)
            .with(mockall::predicate::eq(token_hash_for_expect))
            .returning(|_| Ok(true));

        refresh_repo
            .expect_revoke_all_user_tokens()
            .times(1)
            .returning(|_| Ok(()));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .password_reset_token_repository(Box::new(password_reset_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token(&token, "newpassword123")
            .await;

        assert!(result.is_ok());

        Ok(())
    }

    // Test 14: reset_password_with_token fails without repository configured
    #[tokio::test]
    async fn test_reset_password_with_token_fails_without_repo() -> Result<()> {
        let user_repo = MockUserRepository::new();
        let refresh_repo = MockRefreshTokenRepository::new();

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service
            .reset_password_with_token("token", "newpassword123")
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Password reset token repository not configured")
        );

        Ok(())
    }
}
