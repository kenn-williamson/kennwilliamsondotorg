use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;

use super::AuthService;
use crate::events::types::PasswordChangedEvent;
use crate::models::api::{PasswordChangeRequest, SetPasswordRequest};

impl AuthService {
    /// Change user password
    pub async fn change_password(
        &self,
        user_id: Uuid,
        request: PasswordChangeRequest,
    ) -> Result<()> {
        // Get current user to verify they exist
        let user = self.user_repository.find_by_id(user_id).await?;
        if user.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }

        // Get current password from credentials (OAuth-only users cannot change password)
        let credentials_repo = self
            .credentials_repository
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Credentials repository not configured"))?;

        let credential = credentials_repo.find_by_user_id(user_id).await?;
        let password_hash = match credential {
            Some(cred) => cred.password_hash,
            None => {
                return Err(anyhow::anyhow!(
                    "Cannot change password for OAuth-only accounts"
                ));
            }
        };

        if !verify(&request.current_password, &password_hash)? {
            return Err(anyhow::anyhow!("Current password is incorrect"));
        }

        // Hash new password
        let new_password_hash = hash(&request.new_password, DEFAULT_COST)?;

        // Update password in credentials table
        credentials_repo
            .update_password(user_id, new_password_hash)
            .await?;

        // Publish PasswordChangedEvent if event publisher is configured
        if let Some(event_publisher) = &self.event_publisher {
            let event = PasswordChangedEvent::new(user_id);
            if let Err(e) = event_publisher.publish(Box::new(event)).await {
                log::error!("Failed to publish PasswordChangedEvent: {}", e);
                // Don't fail the operation if event publishing fails
            }
        }

        Ok(())
    }

    /// Set password for users who don't have credentials (e.g., OAuth-only users)
    /// This allows them to add password authentication to their account
    pub async fn set_password(&self, user_id: Uuid, request: SetPasswordRequest) -> Result<()> {
        // Get current user to verify they exist
        let user = self.user_repository.find_by_id(user_id).await?;
        if user.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }

        // Get credentials repository
        let credentials_repo = self
            .credentials_repository
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Credentials repository not configured"))?;

        // Check if user already has credentials
        let existing_credential = credentials_repo.find_by_user_id(user_id).await?;
        if existing_credential.is_some() {
            return Err(anyhow::anyhow!(
                "User already has password credentials. Use change-password endpoint instead."
            ));
        }

        // Hash new password
        let password_hash = hash(&request.new_password, DEFAULT_COST)?;

        // Create credentials for the user
        credentials_repo.create(user_id, password_hash).await?;

        // Publish PasswordChangedEvent if event publisher is configured
        if let Some(event_publisher) = &self.event_publisher {
            let event = PasswordChangedEvent::new(user_id);
            if let Err(e) = event_publisher.publish(Box::new(event)).await {
                log::error!("Failed to publish PasswordChangedEvent: {}", e);
                // Don't fail the operation if event publishing fails
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::mock_refresh_token_repository::MockRefreshTokenRepository;
    use crate::repositories::mocks::mock_user_credentials_repository::MockUserCredentialsRepository;
    use crate::repositories::mocks::mock_user_repository::MockUserRepository;
    use anyhow::Result;
    use bcrypt::{DEFAULT_COST, hash};
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    fn create_test_user() -> crate::models::db::User {
        crate::models::db::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
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
        let mut creds_repo = MockUserCredentialsRepository::new();
        let user = create_test_user();
        let user_id = user.id;

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| {
                Ok(Some(crate::models::db::UserCredentials {
                    user_id,
                    password_hash: hash("current_password", DEFAULT_COST).unwrap(),
                    password_updated_at: Utc::now(),
                    created_at: Utc::now(),
                }))
            });

        creds_repo
            .expect_update_password()
            .times(1)
            .with(
                eq(user_id),
                mockall::predicate::function(|hash: &String| {
                    let current_hash = bcrypt::hash("current_password", DEFAULT_COST).unwrap();
                    hash != &current_hash
                }),
            )
            .returning(|_, _| Ok(()));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .jwt_secret("test-secret".to_string())
            .build();

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
        let mut creds_repo = MockUserCredentialsRepository::new();
        let user = create_test_user();
        let user_id = user.id;

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| {
                Ok(Some(crate::models::db::UserCredentials {
                    user_id,
                    password_hash: hash("current_password", DEFAULT_COST).unwrap(),
                    password_updated_at: Utc::now(),
                    created_at: Utc::now(),
                }))
            });

        let request = PasswordChangeRequest {
            current_password: "wrong_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .jwt_secret("test-secret".to_string())
            .build();

        let result = auth_service.change_password(user_id, request).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Current password is incorrect")
        );

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
        let mut creds_repo = MockUserCredentialsRepository::new();
        let user = create_test_user();
        let user_id = user.id;

        user_repo
            .expect_find_by_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));

        creds_repo
            .expect_find_by_user_id()
            .times(1)
            .with(eq(user_id))
            .returning(move |_| {
                Ok(Some(crate::models::db::UserCredentials {
                    user_id,
                    password_hash: hash("current_password", DEFAULT_COST).unwrap(),
                    password_updated_at: Utc::now(),
                    created_at: Utc::now(),
                }))
            });

        creds_repo
            .expect_update_password()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let request = PasswordChangeRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .jwt_secret("test-secret".to_string())
            .build();

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
            let mut creds_repo = MockUserCredentialsRepository::new();
            let user = create_test_user();
            let user_id = user.id;

            user_repo
                .expect_find_by_id()
                .times(1)
                .with(eq(user_id))
                .returning(move |_| Ok(Some(user.clone())));

            creds_repo
                .expect_find_by_user_id()
                .times(1)
                .with(eq(user_id))
                .returning(move |_| {
                    Ok(Some(crate::models::db::UserCredentials {
                        user_id,
                        password_hash: hash(current_pass, DEFAULT_COST).unwrap(),
                        password_updated_at: Utc::now(),
                        created_at: Utc::now(),
                    }))
                });

            creds_repo
                .expect_update_password()
                .times(1)
                .returning(|_, _| Ok(()));

            let request = PasswordChangeRequest {
                current_password: current_pass.to_string(),
                new_password: new_pass.to_string(),
            };

            let auth_service = AuthService::builder()
                .user_repository(Box::new(user_repo))
                .credentials_repository(Box::new(creds_repo))
                .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
                .jwt_secret("test-secret".to_string())
                .build();

            let result = auth_service.change_password(user_id, request).await;
            assert!(
                result.is_ok(),
                "Failed for current_pass: '{}', new_pass: '{}'",
                current_pass,
                new_pass
            );
        }

        Ok(())
    }
}
