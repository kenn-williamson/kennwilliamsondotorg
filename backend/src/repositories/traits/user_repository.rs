use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::user::User;

/// Data structure for creating a new user
#[derive(Debug, Clone)]
pub struct CreateUserData {
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub slug: String,
}

/// Data structure for updating user information
#[derive(Debug, Clone)]
pub struct UserUpdates {
    pub display_name: Option<String>,
    pub slug: Option<String>,
    pub active: Option<bool>,
}

/// Repository trait for user data operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user
    async fn create_user(&self, user_data: &CreateUserData) -> Result<User>;
    
    /// Find user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    
    /// Find user by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    
    /// Find user by slug
    async fn find_by_slug(&self, slug: &str) -> Result<Option<User>>;
    
    /// Update user information
    async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User>;
    
    /// Check if email exists
    async fn email_exists(&self, email: &str) -> Result<bool>;
    
    /// Check if slug exists
    async fn slug_exists(&self, slug: &str) -> Result<bool>;
    
    /// Deactivate user (soft delete)
    async fn deactivate_user(&self, id: Uuid) -> Result<()>;
    
    /// Update user password
    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;
    
    /// Check if slug exists excluding a specific user
    async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool>;
    
    /// Get user roles
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>>;
    
    /// Get all users with their roles (for admin)
    async fn get_all_users_with_roles(
        &self,
        search: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::models::api::UserWithRoles>>;
    
    /// Add a role to a user
    async fn add_user_role(&self, user_id: Uuid, role: &str) -> Result<()>;
    
    /// Remove a role from a user
    async fn remove_user_role(&self, user_id: Uuid, role: &str) -> Result<()>;
    
    /// Count all users
    async fn count_all_users(&self) -> Result<i64>;
    
    /// Count active users
    async fn count_active_users(&self) -> Result<i64>;
}
