use std::sync::Arc;
use anyhow::Result;

use crate::models::api::SystemStatsResponse;
use crate::repositories::traits::{PhraseRepository, AdminRepository};

/// Statistics service for admin operations
pub struct StatsService {
    phrase_repository: Arc<dyn PhraseRepository>,
    admin_repository: Arc<dyn AdminRepository>,
}

impl StatsService {
    pub fn new(
        phrase_repository: Box<dyn PhraseRepository>,
        admin_repository: Box<dyn AdminRepository>,
    ) -> Self {
        Self {
            phrase_repository: Arc::from(phrase_repository),
            admin_repository: Arc::from(admin_repository),
        }
    }

    /// Get system statistics
    pub async fn get_system_stats(&self) -> Result<SystemStatsResponse> {
        // Get total users count
        let total_users = self.admin_repository.count_all_users().await?;
        
        // Get active users count
        let active_users = self.admin_repository.count_active_users().await?;
        
        // Get pending suggestions count
        let pending_suggestions = self.phrase_repository.count_pending_suggestions().await?;
        
        // Get total phrases count
        let total_phrases = self.phrase_repository.count_all_phrases().await?;

        Ok(SystemStatsResponse {
            total_users,
            active_users,
            pending_suggestions,
            total_phrases,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockPhraseRepository, MockAdminRepository};

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_get_system_stats_success() {
        // Setup mocks
        let mut mock_phrase_repo = MockPhraseRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();

        // Configure mock expectations
        mock_admin_repo
            .expect_count_all_users()
            .times(1)
            .returning(|| Ok(100));

        mock_admin_repo
            .expect_count_active_users()
            .times(1)
            .returning(|| Ok(85));

        mock_phrase_repo
            .expect_count_pending_suggestions()
            .times(1)
            .returning(|| Ok(5));

        mock_phrase_repo
            .expect_count_all_phrases()
            .times(1)
            .returning(|| Ok(50));

        // Create service
        let service = StatsService::new(
            Box::new(mock_phrase_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.get_system_stats().await;

        // Assert
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_users, 100);
        assert_eq!(stats.active_users, 85);
        assert_eq!(stats.pending_suggestions, 5);
        assert_eq!(stats.total_phrases, 50);
    }

    #[tokio::test]
    #[allow(unused_mut)]
    async fn test_get_system_stats_admin_repo_error() {
        // Setup mocks
        let mut mock_phrase_repo = MockPhraseRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();

        // Configure mock expectations - admin repo fails
        mock_admin_repo
            .expect_count_all_users()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("Database error")));

        // Create service
        let service = StatsService::new(
            Box::new(mock_phrase_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.get_system_stats().await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_get_system_stats_phrase_repo_error() {
        // Setup mocks
        let mut mock_phrase_repo = MockPhraseRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();

        // Configure mock expectations - admin repo succeeds, phrase repo fails
        mock_admin_repo
            .expect_count_all_users()
            .times(1)
            .returning(|| Ok(100));

        mock_admin_repo
            .expect_count_active_users()
            .times(1)
            .returning(|| Ok(85));

        mock_phrase_repo
            .expect_count_pending_suggestions()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("Phrase repo error")));

        // Create service
        let service = StatsService::new(
            Box::new(mock_phrase_repo),
            Box::new(mock_admin_repo),
        );

        // Test
        let result = service.get_system_stats().await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Phrase repo error"));
    }
}
