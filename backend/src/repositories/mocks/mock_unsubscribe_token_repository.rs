use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::unsubscribe_token::UnsubscribeTokenInfo;
use crate::repositories::traits::unsubscribe_token_repository::UnsubscribeTokenRepository;

// Generate mock for UnsubscribeTokenRepository trait
mock! {
    pub UnsubscribeTokenRepository {}

    #[async_trait]
    impl UnsubscribeTokenRepository for UnsubscribeTokenRepository {
        async fn create_or_replace(&self, user_id: Uuid, email_type: &str, token_hash: &str) -> Result<bool>;
        async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<UnsubscribeTokenInfo>>;
        async fn delete_by_user_and_type(&self, user_id: Uuid, email_type: &str) -> Result<()>;
        async fn delete_all_for_user(&self, user_id: Uuid) -> Result<()>;
    }
}
