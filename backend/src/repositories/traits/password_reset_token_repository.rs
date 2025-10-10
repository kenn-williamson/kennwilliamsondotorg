use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::db::user::PasswordResetToken;

/// Data structure for creating a new password reset token
#[derive(Debug, Clone)]
pub struct CreatePasswordResetTokenData {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

/// Repository trait for password reset token operations
#[async_trait]
pub trait PasswordResetTokenRepository: Send + Sync {
    /// Create a new password reset token
    async fn create_token(
        &self,
        token_data: &CreatePasswordResetTokenData,
    ) -> Result<PasswordResetToken>;

    /// Find token by token hash (filters out expired and used tokens)
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<PasswordResetToken>>;

    /// Mark a token as used (sets used_at timestamp)
    async fn mark_token_used(&self, token_hash: &str) -> Result<bool>;

    /// Delete all tokens for a user
    async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<u64>;

    /// Find all password reset tokens for a user (for data export)
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<PasswordResetToken>>;

    /// Delete expired tokens (cleanup task)
    async fn delete_expired_tokens(&self) -> Result<u64>;
}
