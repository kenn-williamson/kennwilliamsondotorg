use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user_profile::UserProfile;
use crate::repositories::traits::user_profile_repository::{
    UpdateProfile, UserProfileRepository,
};

// Generate mock for UserProfileRepository trait
mock! {
    pub UserProfileRepository {}

    #[async_trait]
    impl UserProfileRepository for UserProfileRepository {
        async fn create(&self, user_id: Uuid) -> Result<UserProfile>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>>;
        async fn update(&self, user_id: Uuid, data: UpdateProfile) -> Result<UserProfile>;
        async fn delete(&self, user_id: Uuid) -> Result<()>;
    }
}
