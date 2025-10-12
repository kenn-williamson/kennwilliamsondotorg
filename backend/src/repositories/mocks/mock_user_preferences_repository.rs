use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user_preferences::UserPreferences;
use crate::repositories::traits::user_preferences_repository::UserPreferencesRepository;

// Generate mock for UserPreferencesRepository trait
mock! {
    pub UserPreferencesRepository {}

    #[async_trait]
    impl UserPreferencesRepository for UserPreferencesRepository {
        async fn create(&self, user_id: Uuid) -> Result<UserPreferences>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserPreferences>>;
        async fn update_timer_settings(&self, user_id: Uuid, is_public: bool, show_in_list: bool) -> Result<()>;
        async fn delete(&self, user_id: Uuid) -> Result<()>;
    }
}
