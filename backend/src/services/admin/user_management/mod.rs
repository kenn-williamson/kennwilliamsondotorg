use std::sync::Arc;
use uuid::Uuid;

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
        let password_hash = hash(&new_password, bcrypt::DEFAULT_COST)
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
    use rand::{Rng, distr::Alphanumeric};
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}

fn hash(password: &str, cost: u32) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, cost)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::repositories::mocks::{MockUserRepository, MockRefreshTokenRepository, MockAdminRepository};
    use crate::models::api::UserWithRoles;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_users_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();

        // Create test data
        let user = UserWithRoles {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            roles: vec!["user".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Configure mock expectations
        mock_admin_repo
            .expect_get_all_users_with_roles()
            .with(eq(None), eq(None), eq(None))
            .times(1)
            .returning(move |_, _, _| Ok(vec![user.clone()]));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.get_users(None, None, None).await;

        // Assert
        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, "test@example.com");
    }

    #[tokio::test]
    async fn test_deactivate_user_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_admin_repo
            .expect_update_user_status()
            .with(eq(user_id), eq(false))
            .times(1)
            .returning(|_, _| Ok(()));

        mock_refresh_repo
            .expect_revoke_all_user_tokens()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.deactivate_user(user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_activate_user_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_admin_repo
            .expect_update_user_status()
            .with(eq(user_id), eq(true))
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.activate_user(user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reset_user_password_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_user_repo
            .expect_update_password()
            .with(eq(user_id), mockall::predicate::function(|_hash: &str| true))
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.reset_user_password(user_id).await;

        // Assert
        assert!(result.is_ok());
        let password = result.unwrap();
        assert_eq!(password.len(), 12); // Random password length
    }

    #[tokio::test]
    async fn test_promote_to_admin_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_admin_repo
            .expect_add_user_role()
            .with(eq(user_id), eq("admin"))
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.promote_to_admin(user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_is_user_admin_true() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_user_repo
            .expect_get_user_roles()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["admin".to_string(), "user".to_string()]));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.is_user_admin(user_id).await;

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_is_user_admin_false() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_user_repo
            .expect_get_user_roles()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.is_user_admin(user_id).await;

        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
