use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::traits::{AdminRepository, RefreshTokenRepository, UserRepository};

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
    ) -> anyhow::Result<Vec<crate::models::api::admin::AdminUserListItem>> {
        let users_db = self
            .admin_repository
            .get_all_users_with_roles(search, limit, offset)
            .await?;

        // Convert database structs to API structs using the from_db method
        let users = users_db
            .into_iter()
            .map(crate::models::api::admin::AdminUserListItem::from_db)
            .collect();

        Ok(users)
    }

    /// Deactivate a user
    pub async fn deactivate_user(&self, user_id: Uuid) -> anyhow::Result<()> {
        // Use AdminRepository to update user status
        self.admin_repository
            .update_user_status(user_id, false)
            .await?;

        // Revoke all refresh tokens
        self.refresh_token_repository
            .revoke_all_user_tokens(user_id)
            .await?;

        Ok(())
    }

    /// Activate a user
    pub async fn activate_user(&self, user_id: Uuid) -> anyhow::Result<()> {
        // Use AdminRepository to update user status
        self.admin_repository
            .update_user_status(user_id, true)
            .await?;
        Ok(())
    }

    /// Reset user password
    pub async fn reset_user_password(&self, user_id: Uuid) -> anyhow::Result<String> {
        // Generate random password
        let new_password = generate_random_password();
        let password_hash = hash(&new_password, bcrypt::DEFAULT_COST)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;

        // Use the existing update_password method in UserRepository
        self.user_repository
            .update_password(user_id, &password_hash)
            .await?;

        Ok(new_password)
    }

    /// Promote user to admin
    pub async fn promote_to_admin(&self, user_id: Uuid) -> anyhow::Result<()> {
        // Add admin role to user using AdminRepository
        self.admin_repository
            .add_user_role(user_id, "admin")
            .await?;
        Ok(())
    }

    /// Add a role to a user with validation
    pub async fn add_role(&self, user_id: Uuid, role_name: &str) -> anyhow::Result<()> {
        // Validate role name
        Self::validate_role_name(role_name)?;

        // Cannot add 'user' role (it's immutable and auto-assigned)
        if role_name == "user" {
            return Err(anyhow::anyhow!(
                "Cannot manually add 'user' role - it is automatically assigned on registration"
            ));
        }

        // Add role using repository
        self.admin_repository
            .add_user_role(user_id, role_name)
            .await?;

        Ok(())
    }

    /// Remove a role from a user with validation
    pub async fn remove_role(&self, user_id: Uuid, role_name: &str) -> anyhow::Result<()> {
        // Validate role name
        Self::validate_role_name(role_name)?;

        // Cannot remove 'user' role (it's immutable)
        if role_name == "user" {
            return Err(anyhow::anyhow!(
                "Cannot remove 'user' role - it is a permanent base role"
            ));
        }

        // Prevent removing the last admin role
        if role_name == "admin" {
            let user_roles = self.user_repository.get_user_roles(user_id).await?;
            if user_roles.contains(&"admin".to_string()) {
                // Count total admins
                let all_users = self
                    .admin_repository
                    .get_all_users_with_roles(None, Some(1000), None)
                    .await?;
                let admin_count = all_users
                    .iter()
                    .filter(|u| u.roles.as_ref().map_or(false, |r| r.contains(&"admin".to_string())))
                    .count();

                if admin_count <= 1 {
                    return Err(anyhow::anyhow!(
                        "Cannot remove the last admin role from the system"
                    ));
                }
            }
        }

        // Remove role using repository
        self.admin_repository
            .remove_user_role(user_id, role_name)
            .await?;

        Ok(())
    }

    /// Validate role name against allowed manageable roles
    fn validate_role_name(role_name: &str) -> anyhow::Result<()> {
        const MANAGEABLE_ROLES: &[&str] = &["user", "email-verified", "trusted-contact", "admin"];

        if !MANAGEABLE_ROLES.contains(&role_name) {
            return Err(anyhow::anyhow!(
                "Invalid role name '{}'. Allowed roles: {}",
                role_name,
                MANAGEABLE_ROLES.join(", ")
            ));
        }

        Ok(())
    }

    /// Check if user is admin
    #[allow(dead_code)] // Part of admin service API for future features
    pub async fn is_user_admin(&self, user_id: Uuid) -> anyhow::Result<bool> {
        let roles = self.user_repository.get_user_roles(user_id).await?;
        Ok(roles.contains(&"admin".to_string()))
    }
}

/// Generate a random password for admin reset
fn generate_random_password() -> String {
    use rand::{distr::Alphanumeric, Rng};
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
    use crate::models::db::UserWithRoles;
    use crate::repositories::mocks::{
        MockAdminRepository, MockRefreshTokenRepository, MockUserRepository,
    };
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    #[allow(unused_mut)]
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
            real_name: None,
            google_user_id: None,
            timer_is_public: false,
            timer_show_in_list: false,
            roles: Some(vec!["user".to_string()]),
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
    #[allow(unused_mut)]
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
    #[allow(unused_mut)]
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
    #[allow(unused_mut)]
    async fn test_reset_user_password_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_user_repo
            .expect_update_password()
            .with(
                eq(user_id),
                mockall::predicate::function(|_hash: &str| true),
            )
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
    #[allow(unused_mut)]
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
    #[allow(unused_mut)]
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
    #[allow(unused_mut)]
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

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_add_role_trusted_contact_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_admin_repo
            .expect_add_user_role()
            .with(eq(user_id), eq("trusted-contact"))
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.add_role(user_id, "trusted-contact").await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_add_role_email_verified_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_admin_repo
            .expect_add_user_role()
            .with(eq(user_id), eq("email-verified"))
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.add_role(user_id, "email-verified").await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_add_role_invalid_name_fails() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.add_role(user_id, "invalid-role").await;

        // Assert
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid role name"));
        assert!(error_msg.contains("invalid-role"));
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_add_role_user_role_fails() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.add_role(user_id, "user").await;

        // Assert
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Cannot manually add"));
        assert!(error_msg.contains("user"));
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_remove_role_success() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Configure mock expectations
        mock_admin_repo
            .expect_remove_user_role()
            .with(eq(user_id), eq("trusted-contact"))
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.remove_role(user_id, "trusted-contact").await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_remove_role_user_role_fails() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.remove_role(user_id, "user").await;

        // Assert
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Cannot remove"));
        assert!(error_msg.contains("user"));
        assert!(error_msg.contains("permanent"));
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_remove_role_last_admin_fails() {
        // Setup mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_refresh_repo = MockRefreshTokenRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        // Create test data - only one admin in the system
        let admin_user = UserWithRoles {
            id: user_id,
            email: "admin@example.com".to_string(),
            display_name: "Admin User".to_string(),
            slug: "admin-user".to_string(),
            active: true,
            real_name: None,
            google_user_id: None,
            timer_is_public: false,
            timer_show_in_list: false,
            roles: Some(vec!["user".to_string(), "admin".to_string()]),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Configure mock expectations
        mock_user_repo
            .expect_get_user_roles()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string(), "admin".to_string()]));

        mock_admin_repo
            .expect_get_all_users_with_roles()
            .with(eq(None), eq(Some(1000)), eq(None))
            .times(1)
            .returning(move |_, _, _| Ok(vec![admin_user.clone()]));

        // Create service
        let service = UserManagementService::new(
            Box::new(mock_user_repo),
            Box::new(mock_refresh_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.remove_role(user_id, "admin").await;

        // Assert
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Cannot remove the last admin"));
    }
}
