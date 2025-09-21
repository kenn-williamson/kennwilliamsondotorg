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
