use crate::repositories::traits::{RefreshTokenRepository, VerificationTokenRepository};
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct CleanupService {
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    verification_token_repository: Arc<dyn VerificationTokenRepository>,
}

impl CleanupService {
    pub fn new(
        refresh_token_repository: Box<dyn RefreshTokenRepository>,
        verification_token_repository: Box<dyn VerificationTokenRepository>,
    ) -> Self {
        Self {
            refresh_token_repository: Arc::from(refresh_token_repository),
            verification_token_repository: Arc::from(verification_token_repository),
        }
    }

    /// Clean up expired tokens from both refresh_tokens and verification_tokens tables
    /// Returns the total number of tokens deleted
    pub async fn cleanup_expired_tokens(&self) -> Result<u64> {
        let refresh_count = self.refresh_token_repository.cleanup_expired_tokens().await?;
        let verification_count = self.verification_token_repository.delete_expired_tokens().await?;

        let total = refresh_count + verification_count;

        log::info!(
            "Cleanup completed: {} refresh tokens, {} verification tokens, {} total",
            refresh_count,
            verification_count,
            total
        );

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::refresh_token::{CreateRefreshToken, RefreshToken};
    use crate::models::db::user::VerificationToken;
    use crate::repositories::traits::{RefreshTokenRepository, VerificationTokenRepository};
    use async_trait::async_trait;
    use uuid::Uuid;

    // Mock RefreshTokenRepository for testing
    struct MockRefreshTokenRepository {
        cleanup_count: u64,
        should_fail: bool,
    }

    #[async_trait]
    impl RefreshTokenRepository for MockRefreshTokenRepository {
        async fn create_token(&self, _token_data: &CreateRefreshToken) -> Result<RefreshToken> {
            unimplemented!()
        }

        async fn find_by_token(&self, _token: &str) -> Result<Option<RefreshToken>> {
            unimplemented!()
        }

        async fn revoke_token(&self, _token: &str) -> Result<()> {
            unimplemented!()
        }

        async fn revoke_all_user_tokens(&self, _user_id: Uuid) -> Result<()> {
            unimplemented!()
        }

        async fn cleanup_expired_tokens(&self) -> Result<u64> {
            if self.should_fail {
                anyhow::bail!("Mock refresh token cleanup failed");
            }
            Ok(self.cleanup_count)
        }
    }

    // Mock VerificationTokenRepository for testing
    struct MockVerificationTokenRepository {
        cleanup_count: u64,
        should_fail: bool,
    }

    #[async_trait]
    impl VerificationTokenRepository for MockVerificationTokenRepository {
        async fn create_token(
            &self,
            _token_data: &crate::repositories::traits::verification_token_repository::CreateVerificationTokenData,
        ) -> Result<VerificationToken> {
            unimplemented!()
        }

        async fn find_by_token_hash(&self, _token_hash: &str) -> Result<Option<VerificationToken>> {
            unimplemented!()
        }

        async fn delete_token(&self, _id: Uuid) -> Result<bool> {
            unimplemented!()
        }

        async fn delete_all_user_tokens(&self, _user_id: Uuid) -> Result<u64> {
            unimplemented!()
        }

        async fn delete_expired_tokens(&self) -> Result<u64> {
            if self.should_fail {
                anyhow::bail!("Mock verification token cleanup failed");
            }
            Ok(self.cleanup_count)
        }
    }

    #[tokio::test]
    async fn test_cleanup_service_creation() {
        let refresh_repo = Box::new(MockRefreshTokenRepository {
            cleanup_count: 0,
            should_fail: false,
        });
        let verification_repo = Box::new(MockVerificationTokenRepository {
            cleanup_count: 0,
            should_fail: false,
        });

        let service = CleanupService::new(refresh_repo, verification_repo);

        // Just verify service was created successfully
        assert!(Arc::strong_count(&service.refresh_token_repository) >= 1);
        assert!(Arc::strong_count(&service.verification_token_repository) >= 1);
    }

    #[tokio::test]
    async fn test_cleanup_calls_both_repositories() {
        let refresh_repo = Box::new(MockRefreshTokenRepository {
            cleanup_count: 5,
            should_fail: false,
        });
        let verification_repo = Box::new(MockVerificationTokenRepository {
            cleanup_count: 3,
            should_fail: false,
        });

        let service = CleanupService::new(refresh_repo, verification_repo);
        let result = service.cleanup_expired_tokens().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8); // 5 + 3
    }

    #[tokio::test]
    async fn test_cleanup_returns_zero_when_no_tokens() {
        let refresh_repo = Box::new(MockRefreshTokenRepository {
            cleanup_count: 0,
            should_fail: false,
        });
        let verification_repo = Box::new(MockVerificationTokenRepository {
            cleanup_count: 0,
            should_fail: false,
        });

        let service = CleanupService::new(refresh_repo, verification_repo);
        let result = service.cleanup_expired_tokens().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_cleanup_handles_refresh_token_error() {
        let refresh_repo = Box::new(MockRefreshTokenRepository {
            cleanup_count: 0,
            should_fail: true, // This will cause an error
        });
        let verification_repo = Box::new(MockVerificationTokenRepository {
            cleanup_count: 3,
            should_fail: false,
        });

        let service = CleanupService::new(refresh_repo, verification_repo);
        let result = service.cleanup_expired_tokens().await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mock refresh token cleanup failed"));
    }

    #[tokio::test]
    async fn test_cleanup_handles_verification_token_error() {
        let refresh_repo = Box::new(MockRefreshTokenRepository {
            cleanup_count: 5,
            should_fail: false,
        });
        let verification_repo = Box::new(MockVerificationTokenRepository {
            cleanup_count: 0,
            should_fail: true, // This will cause an error
        });

        let service = CleanupService::new(refresh_repo, verification_repo);
        let result = service.cleanup_expired_tokens().await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mock verification token cleanup failed"));
    }
}
