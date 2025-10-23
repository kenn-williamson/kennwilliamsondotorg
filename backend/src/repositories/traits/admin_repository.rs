use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

/// Repository trait for admin-specific user operations
#[async_trait]
pub trait AdminRepository: Send + Sync {
    /// Update user active status (admin only)
    async fn update_user_status(&self, user_id: Uuid, active: bool) -> Result<()>;

    /// Add a role to a user (admin only)
    async fn add_user_role(&self, user_id: Uuid, role: &str) -> Result<()>;

    /// Remove a role from a user (admin only)
    #[allow(dead_code)] // Future feature for admin role management
    async fn remove_user_role(&self, user_id: Uuid, role: &str) -> Result<()>;

    /// Get all users with their roles (admin only)
    async fn get_all_users_with_roles(
        &self,
        search: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::models::db::UserWithRoles>>;

    /// Count all users (admin only)
    async fn count_all_users(&self) -> Result<i64>;

    /// Count active users (admin only)
    async fn count_active_users(&self) -> Result<i64>;

    /// Get email addresses of all active, verified admin users for notifications
    /// Returns empty vec if no admins found (not an error)
    async fn get_admin_emails(&self) -> Result<Vec<String>>;
}
