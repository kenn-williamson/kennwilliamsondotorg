use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;
use serde_json::Value;

use crate::models::db::refresh_token::RefreshToken;

/// Data structure for creating a refresh token
#[derive(Debug, Clone)]
pub struct CreateRefreshTokenData {
    pub user_id: Uuid,
    pub token: String,
    pub device_info: Option<Value>,
    pub expires_at: DateTime<Utc>,
}

/// Repository trait for refresh token operations
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// Create a new refresh token
    async fn create_token(&self, token_data: &CreateRefreshTokenData) -> Result<RefreshToken>;
    
    /// Find refresh token by token string
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>>;
    
    /// Revoke a specific refresh token
    async fn revoke_token(&self, token: &str) -> Result<()>;
    
    /// Revoke all refresh tokens for a user
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<()>;
    
    /// Clean up expired tokens
    async fn cleanup_expired_tokens(&self) -> Result<u64>;
    
    /// Check if token is valid (exists and not expired)
    async fn is_token_valid(&self, token: &str) -> Result<bool>;
}
