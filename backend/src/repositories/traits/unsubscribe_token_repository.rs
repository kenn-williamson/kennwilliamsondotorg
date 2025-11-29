use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::unsubscribe_token::UnsubscribeTokenInfo;

/// Repository trait for unsubscribe tokens
/// Manages one-click email unsubscribe tokens
#[async_trait]
pub trait UnsubscribeTokenRepository: Send + Sync {
    /// Create or replace an unsubscribe token for a user and email type
    /// Uses UPSERT to handle existing tokens
    ///
    /// # Arguments
    /// * `user_id` - The user this token is for
    /// * `email_type` - The type of email (e.g., "blog_notifications")
    /// * `token_hash` - SHA-256 hash of the raw token
    ///
    /// # Returns
    /// * `true` if a new token was created, `false` if an existing one was replaced
    async fn create_or_replace(
        &self,
        user_id: Uuid,
        email_type: &str,
        token_hash: &str,
    ) -> Result<bool>;

    /// Find token info by token hash
    /// Returns user_id and email_type if token exists
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<UnsubscribeTokenInfo>>;

    /// Delete a token by user and email type
    /// Used when user re-subscribes or token is consumed
    async fn delete_by_user_and_type(&self, user_id: Uuid, email_type: &str) -> Result<()>;

    /// Delete all tokens for a user
    /// Used during account deletion
    async fn delete_all_for_user(&self, user_id: Uuid) -> Result<()>;
}
