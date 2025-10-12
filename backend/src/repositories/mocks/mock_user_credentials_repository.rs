use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user_credentials::UserCredentials;
use crate::repositories::traits::user_credentials_repository::UserCredentialsRepository;

// Generate mock for UserCredentialsRepository trait
mock! {
    pub UserCredentialsRepository {}

    #[async_trait]
    impl UserCredentialsRepository for UserCredentialsRepository {
        async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>>;
        async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()>;
        async fn delete(&self, user_id: Uuid) -> Result<()>;
        async fn has_password(&self, user_id: Uuid) -> Result<bool>;
    }
}
