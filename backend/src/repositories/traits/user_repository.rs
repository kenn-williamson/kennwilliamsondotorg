use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user::User;

/// Data structure for creating a new user (email/password registration)
#[derive(Debug, Clone)]
pub struct CreateUserData {
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub slug: String,
}

/// Data structure for creating an OAuth user (no password required)
#[derive(Debug, Clone)]
pub struct CreateOAuthUserData {
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub real_name: Option<String>,
    pub google_user_id: Option<String>,
}

/// Data structure for updating user profile information (user-controlled fields only)
#[derive(Debug, Clone)]
pub struct UserUpdates {
    pub display_name: String,
    pub slug: String,
}

/// Repository trait for user data operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user with email/password
    async fn create_user(&self, user_data: &CreateUserData) -> Result<User>;

    /// Create a new OAuth user (no password)
    async fn create_oauth_user(&self, user_data: &CreateOAuthUserData) -> Result<User>;

    /// Find user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;

    /// Find user by Google user ID
    async fn find_by_google_user_id(&self, google_user_id: &str) -> Result<Option<User>>;

    /// Find user by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;

    /// Update user information
    async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User>;

    /// Link Google account to existing user
    async fn link_google_account(
        &self,
        user_id: Uuid,
        google_user_id: &str,
        real_name: Option<String>,
    ) -> Result<()>;

    /// Update real_name from OAuth provider (auto-updates on OAuth login)
    async fn update_real_name(&self, user_id: Uuid, real_name: Option<String>) -> Result<()>;

    /// Check if slug exists
    async fn slug_exists(&self, slug: &str) -> Result<bool>;

    /// Update user password
    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;

    /// Check if slug exists excluding a specific user
    async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool>;

    /// Get user roles
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>>;

    /// Add role to user
    async fn add_role_to_user(&self, user_id: Uuid, role_name: &str) -> Result<()>;

    /// Check if user has specific role
    async fn has_role(&self, user_id: Uuid, role_name: &str) -> Result<bool>;
}
