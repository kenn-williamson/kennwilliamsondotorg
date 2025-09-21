use anyhow::Result;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::api::PasswordChangeRequest;
use super::AuthService;

impl AuthService {
    /// Change user password
    pub async fn change_password(&self, user_id: Uuid, request: PasswordChangeRequest) -> Result<()> {
        // Get current user
        let user = self.user_repository.find_by_id(user_id).await?;
        let user = match user {
            Some(user) => user,
            None => return Err(anyhow::anyhow!("User not found")),
        };

        // Verify current password
        if !verify(&request.current_password, &user.password_hash)? {
            return Err(anyhow::anyhow!("Current password is incorrect"));
        }

        // Hash new password
        let new_password_hash = hash(&request.new_password, DEFAULT_COST)?;

        // Update password
        self.user_repository.update_password(user_id, &new_password_hash).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use mockall::predicate::eq;
    use uuid::Uuid;
    use chrono::Utc;
    use bcrypt::{hash, DEFAULT_COST};

    fn create_test_user() -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: hash("current_password", DEFAULT_COST).unwrap(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn change_password_successful() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user = create_test_user();
        let user_id = user.id;

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_update_password()
            .times(1)
            .with(eq(user_id), mockall::predicate::function(|hash: &str| {
                // Verify that the new password hash is different from the old one
                let current_hash = bcrypt::hash("current_password", DEFAULT_COST).unwrap();
                hash != &current_hash
            }))
            .returning(|_, _| Ok(()));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn change_password_fails_when_user_not_found() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Ok(None));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("User not found"));
        
        Ok(())
    }

    #[tokio::test]
    async fn change_password_fails_with_incorrect_current_password() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user = create_test_user();
        let user_id = user.id;

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        let request = PasswordChangeRequest {
            current_password: "wrong_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Current password is incorrect"));
        
        Ok(())
    }

    #[tokio::test]
    async fn change_password_handles_database_error_during_user_lookup() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user_id = Uuid::new_v4();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(|_| Err(anyhow::anyhow!("Database error")));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
        
        Ok(())
    }

    #[tokio::test]
    async fn change_password_handles_database_error_during_password_update() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user = create_test_user();
        let user_id = user.id;

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_update_password()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
        
        Ok(())
    }

    #[tokio::test]
    async fn change_password_with_various_password_combinations() -> Result<()> {
        let test_cases = vec![
            ("current_password", "new_password123"),
            ("current_password", "AnotherNewPass456"),
            ("current_password", "Complex!@#Pass789"),
            ("current_password", "VeryLongPasswordWithNumbers123"),
        ];

        for (current_pass, new_pass) in test_cases {
            let mut user_repo = MockUserRepository::new();
            let user = create_test_user();
            let user_id = user.id;

            // Setup mock expectations
            user_repo
                .expect_find_by_id()
                .times(1)
                .with(eq(user_id))
                .returning(move |_| Ok(Some(user.clone())));

            user_repo
                .expect_update_password()
                .times(1)
                .returning(|_, _| Ok(()));

            let request = PasswordChangeRequest {
                current_password: current_pass.to_string(),
                new_password: new_pass.to_string(),
            };

            let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
            assert!(result.is_ok(), "Failed for current_pass: '{}', new_pass: '{}'", current_pass, new_pass);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn change_password_verifies_password_hash_difference() -> Result<()> {
        let mut user_repo = MockUserRepository::new();
        let user = create_test_user();
        let user_id = user.id;
        let old_hash = user.password_hash.clone();

        // Setup mock expectations
        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        user_repo
            .expect_update_password()
            .times(1)
            .with(eq(user_id), mockall::predicate::function(move |hash: &str| {
                // Verify that the new password hash is different from the old one
                hash != &old_hash
            }))
            .returning(|_, _| Ok(()));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "completely_different_password".to_string(),
        };

        let auth_service = AuthService::new(
            Box::new(user_repo),
            Box::new(MockRefreshTokenRepository::new()),
            "test-secret".to_string(),
        );
        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_ok());
        
        Ok(())
    }
}
