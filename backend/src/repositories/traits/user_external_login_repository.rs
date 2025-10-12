use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user_external_login::UserExternalLogin;

/// Data for creating external login
pub struct CreateExternalLogin {
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
}

/// Repository trait for user external logins (OAuth)
#[async_trait]
pub trait UserExternalLoginRepository: Send + Sync {
    /// Create external login (link OAuth provider to user)
    async fn create(&self, data: CreateExternalLogin) -> Result<UserExternalLogin>;

    /// Find external login by provider and provider user ID
    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserExternalLogin>>;

    /// Find all external logins for a user
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserExternalLogin>>;

    /// Unlink a provider from a user
    async fn unlink_provider(&self, user_id: Uuid, provider: &str) -> Result<()>;

    /// Delete external login by ID
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// Check if provider is linked to user
    async fn is_provider_linked(&self, user_id: Uuid, provider: &str) -> Result<bool>;
}
