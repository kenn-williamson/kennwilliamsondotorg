use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::refresh_token::{RefreshToken, CreateRefreshToken};

/// Repository trait for refresh token operations
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// Create a new refresh token
    async fn create_token(&self, token_data: &CreateRefreshToken) -> Result<RefreshToken>;
    
    /// Find refresh token by token string
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>>;
    
    /// Revoke a specific refresh token
    async fn revoke_token(&self, token: &str) -> Result<()>;
    
    /// Revoke all refresh tokens for a user
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<()>;
    
    /// Clean up expired tokens
    #[allow(dead_code)] // Future feature for cleanup service
    async fn cleanup_expired_tokens(&self) -> Result<u64>;
}
