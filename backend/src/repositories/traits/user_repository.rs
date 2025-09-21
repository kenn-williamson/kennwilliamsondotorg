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

/// Data structure for updating user profile information (user-controlled fields only)
#[derive(Debug, Clone)]
pub struct UserUpdates {
    pub display_name: String,
    pub slug: String,
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
    
    /// Update user information
    async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User>;
    
    /// Check if slug exists
    async fn slug_exists(&self, slug: &str) -> Result<bool>;
    
    /// Update user password
    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;
    
    /// Check if slug exists excluding a specific user
    async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool>;
    
    /// Get user roles
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>>;
}
