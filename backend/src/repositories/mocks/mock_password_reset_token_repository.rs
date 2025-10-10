use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user::PasswordResetToken;
use crate::repositories::traits::password_reset_token_repository::{
    CreatePasswordResetTokenData, PasswordResetTokenRepository,
};

// Generate mock for PasswordResetTokenRepository trait
mock! {
    pub PasswordResetTokenRepository {}

    #[async_trait]
    impl PasswordResetTokenRepository for PasswordResetTokenRepository {
        async fn create_token(&self, token_data: &CreatePasswordResetTokenData) -> Result<PasswordResetToken>;
        async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<PasswordResetToken>>;
        async fn mark_token_used(&self, token_hash: &str) -> Result<bool>;
        async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<u64>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<PasswordResetToken>>;
        async fn delete_expired_tokens(&self) -> Result<u64>;
    }
}
