use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user_preferences::UserPreferences;

/// Repository trait for user application preferences
#[async_trait]
pub trait UserPreferencesRepository: Send + Sync {
    /// Create default preferences for a user
    async fn create(&self, user_id: Uuid) -> Result<UserPreferences>;

    /// Find preferences by user ID
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserPreferences>>;

    /// Update timer visibility settings
    /// Planned feature for timer privacy controls
    #[allow(dead_code)]
    async fn update_timer_settings(
        &self,
        user_id: Uuid,
        is_public: bool,
        show_in_list: bool,
    ) -> Result<()>;
}
