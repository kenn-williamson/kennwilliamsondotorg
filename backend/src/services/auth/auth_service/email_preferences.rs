use anyhow::{Result, anyhow};
use sha2::{Digest, Sha256};

use super::AuthService;
use crate::models::db::unsubscribe_token::email_types;

/// Hash a raw token to compare with stored hash
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Result of an unsubscribe operation
#[derive(Debug)]
pub struct UnsubscribeResult {
    pub email_type: String,
}

impl AuthService {
    /// Unsubscribe a user from email notifications using a token
    ///
    /// This method:
    /// 1. Validates the token format
    /// 2. Hashes the token and looks it up in the database
    /// 3. Disables the corresponding email preference
    /// 4. Deletes the used token
    ///
    /// # Arguments
    /// * `raw_token` - The raw unsubscribe token from the email link
    ///
    /// # Returns
    /// * `Ok(UnsubscribeResult)` - The email type that was unsubscribed
    /// * `Err` - If token is invalid, not found, or operation fails
    pub async fn unsubscribe_by_token(&self, raw_token: &str) -> Result<UnsubscribeResult> {
        // Validate dependencies
        let unsubscribe_repo = self
            .unsubscribe_token_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Unsubscribe token repository not configured"))?;

        let prefs_repo = self
            .preferences_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Preferences repository not configured"))?;

        // Validate token format (hex string, 64 chars for 32-byte token)
        if raw_token.len() != 64 || !raw_token.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow!("Invalid unsubscribe token format"));
        }

        // Hash the token to compare with stored hash
        let token_hash = hash_token(raw_token);

        // Look up token in database
        let token_info = unsubscribe_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| anyhow!("Unsubscribe token not found or expired"))?;

        // Disable the corresponding preference based on email_type
        match token_info.email_type.as_str() {
            email_types::BLOG_NOTIFICATIONS => {
                prefs_repo
                    .update_blog_notifications(token_info.user_id, false)
                    .await?;
            }
            _ => {
                return Err(anyhow!("Unknown email type: {}", token_info.email_type));
            }
        }

        log::info!(
            "User {} unsubscribed from {} notifications",
            token_info.user_id,
            token_info.email_type
        );

        // Delete the token after use (one-time use)
        if let Err(e) = unsubscribe_repo
            .delete_by_user_and_type(token_info.user_id, &token_info.email_type)
            .await
        {
            log::warn!("Failed to delete used unsubscribe token: {}", e);
            // Non-critical, continue
        }

        Ok(UnsubscribeResult {
            email_type: token_info.email_type,
        })
    }

    /// Validate an unsubscribe token without performing the unsubscribe
    ///
    /// Used by the frontend to display the confirmation page
    ///
    /// # Arguments
    /// * `raw_token` - The raw unsubscribe token from the email link
    ///
    /// # Returns
    /// * `Ok(Some(email_type))` - Token is valid, returns the email type
    /// * `Ok(None)` - Token not found
    /// * `Err` - If validation fails
    pub async fn validate_unsubscribe_token(&self, raw_token: &str) -> Result<Option<String>> {
        let unsubscribe_repo = self
            .unsubscribe_token_repository
            .as_ref()
            .ok_or_else(|| anyhow!("Unsubscribe token repository not configured"))?;

        // Validate token format
        if raw_token.len() != 64 || !raw_token.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow!("Invalid token format"));
        }

        let token_hash = hash_token(raw_token);

        match unsubscribe_repo.find_by_token_hash(&token_hash).await? {
            Some(info) => Ok(Some(info.email_type)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::unsubscribe_token::UnsubscribeTokenInfo;
    use crate::repositories::mocks::{
        MockRefreshTokenRepository, MockUnsubscribeTokenRepository, MockUserPreferencesRepository,
        MockUserRepository,
    };
    use crate::services::auth::AuthService;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_unsubscribe_by_token_success() {
        let user_id = Uuid::new_v4();

        // Create a valid 64-char hex token
        let raw_token = "a".repeat(64);
        let token_hash = hash_token(&raw_token);

        let mut mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();
        let mut mock_prefs_repo = MockUserPreferencesRepository::new();

        // Setup expectations
        mock_unsubscribe_repo
            .expect_find_by_token_hash()
            .withf(move |hash| hash == token_hash)
            .times(1)
            .returning(move |_| {
                Ok(Some(UnsubscribeTokenInfo {
                    user_id,
                    email_type: email_types::BLOG_NOTIFICATIONS.to_string(),
                }))
            });

        mock_prefs_repo
            .expect_update_blog_notifications()
            .withf(move |id, enabled| *id == user_id && !*enabled)
            .times(1)
            .returning(|_, _| Ok(()));

        mock_unsubscribe_repo
            .expect_delete_by_user_and_type()
            .times(1)
            .returning(|_, _| Ok(()));

        let service = AuthService::builder()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(mock_prefs_repo))
            .unsubscribe_token_repository(Box::new(mock_unsubscribe_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service.unsubscribe_by_token(&raw_token).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email_type, email_types::BLOG_NOTIFICATIONS);
    }

    #[tokio::test]
    async fn test_unsubscribe_by_token_invalid_format() {
        let service = AuthService::builder()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(MockUserPreferencesRepository::new()))
            .unsubscribe_token_repository(Box::new(MockUnsubscribeTokenRepository::new()))
            .jwt_secret("test-secret".to_string())
            .build();

        // Too short
        let result = service.unsubscribe_by_token("abc123").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid"));

        // Non-hex characters
        let result = service.unsubscribe_by_token(&"g".repeat(64)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unsubscribe_by_token_not_found() {
        let mut mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();

        mock_unsubscribe_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(|_| Ok(None));

        let service = AuthService::builder()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .preferences_repository(Box::new(MockUserPreferencesRepository::new()))
            .unsubscribe_token_repository(Box::new(mock_unsubscribe_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service.unsubscribe_by_token(&"a".repeat(64)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_validate_unsubscribe_token_valid() {
        let raw_token = "b".repeat(64);
        let token_hash = hash_token(&raw_token);

        let mut mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();

        mock_unsubscribe_repo
            .expect_find_by_token_hash()
            .withf(move |hash| hash == token_hash)
            .times(1)
            .returning(|_| {
                Ok(Some(UnsubscribeTokenInfo {
                    user_id: Uuid::new_v4(),
                    email_type: email_types::BLOG_NOTIFICATIONS.to_string(),
                }))
            });

        let service = AuthService::builder()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .unsubscribe_token_repository(Box::new(mock_unsubscribe_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service.validate_unsubscribe_token(&raw_token).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(email_types::BLOG_NOTIFICATIONS.to_string())
        );
    }

    #[tokio::test]
    async fn test_validate_unsubscribe_token_not_found() {
        let mut mock_unsubscribe_repo = MockUnsubscribeTokenRepository::new();

        mock_unsubscribe_repo
            .expect_find_by_token_hash()
            .times(1)
            .returning(|_| Ok(None));

        let service = AuthService::builder()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .unsubscribe_token_repository(Box::new(mock_unsubscribe_repo))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = service.validate_unsubscribe_token(&"c".repeat(64)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}
