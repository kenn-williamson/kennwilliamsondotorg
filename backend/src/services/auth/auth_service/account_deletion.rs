use anyhow::{anyhow, Result};
use uuid::Uuid;

use super::AuthService;

impl AuthService {
    /// Delete a user's account and all associated data
    /// 
    /// This method performs a hard delete of the user account with the following behavior:
    /// 1. Validates the user exists and is not the system user
    /// 2. Delegates to repository layer which handles phrase reassignment and cascade deletion
    /// 
    /// # Arguments
    /// * `user_id` - The ID of the user to delete
    /// 
    /// # Returns
    /// * `Result<()>` - Success if deletion completed, error if failed
    /// 
    /// # Errors
    /// * Returns error if user is the system user (protection)
    /// * Returns error if user not found
    /// * Returns error if repository deletion fails
    pub async fn delete_account(&self, user_id: Uuid) -> Result<()> {
        // Validate user exists and is not the system user
        let user = self.user_repository.find_by_id(user_id).await?;
        let user = match user {
            Some(user) => user,
            None => return Err(anyhow!("User not found")),
        };

        // Check if this is the system user (protection)
        if user.email == "system@kennwilliamson.org" {
            return Err(anyhow!("Cannot delete system user"));
        }

        log::info!("Starting account deletion for user {}", user_id);

        // Delegate to repository layer which handles phrase reassignment and cascade deletion
        self.user_repository.delete_user(user_id).await?;

        log::info!("Successfully deleted account for user {}", user_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use anyhow::Result;
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user(email: &str) -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: email.to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_system_user() -> crate::models::db::User {
        create_test_user("system@kennwilliamson.org")
    }

    #[tokio::test]
    async fn delete_account_successful() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(create_test_user("test@example.com"))));

        user_repo
            .expect_delete_user()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(()));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );

        let result = auth_service.delete_account(user_id).await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn delete_account_fails_when_user_not_found() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );

        let result = auth_service.delete_account(user_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("User not found"));

        Ok(())
    }

    #[tokio::test]
    async fn delete_account_fails_when_trying_to_delete_system_user() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let system_user = create_system_user();
        let system_user_id = system_user.id;

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(system_user_id))
            .returning(move |_| Ok(Some(system_user.clone())));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );

        let result = auth_service.delete_account(system_user_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot delete system user"));

        Ok(())
    }

    #[tokio::test]
    async fn delete_account_fails_when_repository_delete_fails() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(create_test_user("test@example.com"))));

        user_repo
            .expect_delete_user()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Err(anyhow!("Database error")));

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );

        let result = auth_service.delete_account(user_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));

        Ok(())
    }
}
