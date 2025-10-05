use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::db::user::VerificationToken;

/// Data structure for creating a new verification token
#[derive(Debug, Clone)]
pub struct CreateVerificationTokenData {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

/// Repository trait for verification token operations
#[async_trait]
pub trait VerificationTokenRepository: Send + Sync {
    /// Create a new verification token
    async fn create_token(
        &self,
        token_data: &CreateVerificationTokenData,
    ) -> Result<VerificationToken>;

    /// Find token by token hash
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<VerificationToken>>;

    /// Delete token by ID
    #[allow(dead_code)]
    async fn delete_token(&self, id: Uuid) -> Result<bool>;

    /// Delete all tokens for a user (used when email is verified)
    async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<u64>;

    /// Delete expired tokens (cleanup task)
    async fn delete_expired_tokens(&self) -> Result<u64>;
}
