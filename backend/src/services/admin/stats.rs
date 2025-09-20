use std::sync::Arc;
use anyhow::Result;

use crate::models::api::SystemStatsResponse;
use crate::repositories::traits::{UserRepository, PhraseRepository};

/// Statistics service for admin operations
pub struct StatsService {
    user_repository: Arc<dyn UserRepository>,
    phrase_repository: Arc<dyn PhraseRepository>,
}

impl StatsService {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        phrase_repository: Box<dyn PhraseRepository>,
    ) -> Self {
        Self {
            user_repository: Arc::from(user_repository),
            phrase_repository: Arc::from(phrase_repository),
        }
    }

    /// Get system statistics
    pub async fn get_system_stats(&self) -> Result<SystemStatsResponse> {
        // Get total users count
        let total_users = self.user_repository.count_all_users().await?;
        
        // Get active users count
        let active_users = self.user_repository.count_active_users().await?;
        
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
