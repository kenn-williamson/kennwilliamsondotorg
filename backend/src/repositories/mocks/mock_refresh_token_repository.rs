use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::refresh_token::{CreateRefreshToken, RefreshToken};
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;

// Generate mock for RefreshTokenRepository trait
mock! {
    pub RefreshTokenRepository {}

    #[async_trait]
    impl RefreshTokenRepository for RefreshTokenRepository {
        async fn create_token(&self, token_data: &CreateRefreshToken) -> Result<RefreshToken>;
        async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>>;
        async fn revoke_token(&self, token: &str) -> Result<()>;
        async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<()>;
        async fn cleanup_expired_tokens(&self) -> Result<u64>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    // Helper function to create a test refresh token
    fn create_test_refresh_token() -> RefreshToken {
        RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "test_token_hash".to_string(),
            device_info: None,
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_used_at: None,
        }
    }

    // Helper function to create test data
    fn create_test_token_data() -> CreateRefreshToken {
        CreateRefreshToken {
            user_id: Uuid::new_v4(),
            token_hash: "test_token_hash".to_string(),
            device_info: None,
            expires_at: Utc::now() + chrono::Duration::days(7),
        }
    }

    #[tokio::test]
    async fn test_mock_create_token() {
        let mut mock_repo = MockRefreshTokenRepository::new();
        let token_data = create_test_token_data();

        // Setup mock expectation
        mock_repo
            .expect_create_token()
            .times(1)
            .returning(|_| Ok(create_test_refresh_token()));

        // Test the mock
        let result = mock_repo.create_token(&token_data).await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.token_hash, "test_token_hash");
    }

    #[tokio::test]
    async fn test_mock_find_by_token() {
        let mut mock_repo = MockRefreshTokenRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_find_by_token()
            .times(1)
            .with(eq("test_token_hash"))
            .returning(|_| Ok(Some(create_test_refresh_token())));

        // Test the mock
        let result = mock_repo.find_by_token("test_token_hash").await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(token.is_some());
        assert_eq!(token.unwrap().token_hash, "test_token_hash");
    }

    #[tokio::test]
    async fn test_mock_find_by_token_second() {
        let mut mock_repo = MockRefreshTokenRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_find_by_token()
            .times(1)
            .with(eq("valid_token"))
            .returning(|_| Ok(Some(create_test_refresh_token())));

        // Test the mock
        let result = mock_repo.find_by_token("valid_token").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_mock_revoke_token() {
        let mut mock_repo = MockRefreshTokenRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_revoke_token()
            .times(1)
            .with(eq("token_to_revoke"))
            .returning(|_| Ok(()));

        // Test the mock
        let result = mock_repo.revoke_token("token_to_revoke").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_error_handling() {
        let mut mock_repo = MockRefreshTokenRepository::new();

        // Setup mock to return an error
        mock_repo
            .expect_find_by_token()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database connection failed")));

        // Test error handling
        let result = mock_repo.find_by_token("error_token").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Database connection failed"));
    }
}
