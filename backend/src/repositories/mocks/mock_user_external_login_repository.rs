use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user_external_login::UserExternalLogin;
use crate::repositories::traits::user_external_login_repository::{
    CreateExternalLogin, UserExternalLoginRepository,
};

// Generate mock for UserExternalLoginRepository trait
mock! {
    pub UserExternalLoginRepository {}

    #[async_trait]
    impl UserExternalLoginRepository for UserExternalLoginRepository {
        async fn create(&self, data: CreateExternalLogin) -> Result<UserExternalLogin>;
        async fn find_by_provider(&self, provider: &str, provider_user_id: &str) -> Result<Option<UserExternalLogin>>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserExternalLogin>>;
        async fn unlink_provider(&self, user_id: Uuid, provider: &str) -> Result<()>;
        async fn is_provider_linked(&self, user_id: Uuid, provider: &str) -> Result<bool>;
    }
}
