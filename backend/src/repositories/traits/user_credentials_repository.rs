use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user_credentials::UserCredentials;

/// Repository trait for user credentials (local password authentication)
#[async_trait]
pub trait UserCredentialsRepository: Send + Sync {
    /// Create credentials for a user (during registration or password set)
    async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials>;

    /// Find credentials by user ID
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>>;

    /// Update password hash (during password change)
    async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()>;

    /// Delete credentials (during account deletion or password removal)
    async fn delete(&self, user_id: Uuid) -> Result<()>;

    /// Check if user has password credentials
    async fn has_password(&self, user_id: Uuid) -> Result<bool>;
}
