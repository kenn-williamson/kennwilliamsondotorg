use std::sync::Arc;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use rand::{Rng, distr::Alphanumeric};

use crate::models::api::UserWithRoles;
use crate::repositories::traits::{UserRepository, RefreshTokenRepository, AdminRepository};

/// User management service for admin operations
pub struct UserManagementService {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    admin_repository: Arc<dyn AdminRepository>,
}

impl UserManagementService {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        refresh_token_repository: Box<dyn RefreshTokenRepository>,
        admin_repository: Box<dyn AdminRepository>,
    ) -> Self {
        Self {
            user_repository: Arc::from(user_repository),
            refresh_token_repository: Arc::from(refresh_token_repository),
            admin_repository: Arc::from(admin_repository),
        }
    }

    /// Get all users with optional search
    pub async fn get_users(
        &self,
        search: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<Vec<UserWithRoles>> {
        self.admin_repository.get_all_users_with_roles(search, limit, offset).await
    }

    /// Deactivate a user
    pub async fn deactivate_user(&self, user_id: Uuid) -> anyhow::Result<()> {
        // Use AdminRepository to update user status
        self.admin_repository.update_user_status(user_id, false).await?;
        
        // Revoke all refresh tokens
        self.refresh_token_repository.revoke_all_user_tokens(user_id).await?;
        
        Ok(())
    }

    /// Activate a user
    pub async fn activate_user(&self, user_id: Uuid) -> anyhow::Result<()> {
        // Use AdminRepository to update user status
        self.admin_repository.update_user_status(user_id, true).await?;
        Ok(())
    }

    /// Reset user password
    pub async fn reset_user_password(&self, user_id: Uuid) -> anyhow::Result<String> {
        // Generate random password
        let new_password = generate_random_password();
        let password_hash = hash(&new_password, DEFAULT_COST)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;

        // Use the existing update_password method in UserRepository
        self.user_repository.update_password(user_id, &password_hash).await?;
        
        Ok(new_password)
    }

    /// Promote user to admin
    pub async fn promote_to_admin(&self, user_id: Uuid) -> anyhow::Result<()> {
        // Add admin role to user using AdminRepository
        self.admin_repository.add_user_role(user_id, "admin").await?;
        Ok(())
    }

    /// Check if user is admin
    pub async fn is_user_admin(&self, user_id: Uuid) -> anyhow::Result<bool> {
        let roles = self.user_repository.get_user_roles(user_id).await?;
        Ok(roles.contains(&"admin".to_string()))
    }
}

/// Generate a random password for admin reset
fn generate_random_password() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}