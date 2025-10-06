use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user::VerificationToken;
use crate::repositories::traits::verification_token_repository::{
    CreateVerificationTokenData, VerificationTokenRepository,
};

// Generate mock for VerificationTokenRepository trait
mock! {
    pub VerificationTokenRepository {}

    #[async_trait]
    impl VerificationTokenRepository for VerificationTokenRepository {
        async fn create_token(&self, token_data: &CreateVerificationTokenData) -> Result<VerificationToken>;
        async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<VerificationToken>>;
        async fn delete_token(&self, id: Uuid) -> Result<bool>;
        async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<u64>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<VerificationToken>>;
        async fn delete_expired_tokens(&self) -> Result<u64>;
    }
}
