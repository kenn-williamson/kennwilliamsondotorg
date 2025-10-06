use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::refresh_token::{CreateRefreshToken, RefreshToken};
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;

/// PostgreSQL implementation of RefreshTokenRepository
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn create_token(&self, token_data: &CreateRefreshToken) -> Result<RefreshToken> {
        // Match exact query from RefreshTokenService::create_refresh_token_from_dto
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, token_hash, device_info, expires_at, created_at, updated_at, last_used_at
            "#,
            token_data.user_id,
            token_data.token_hash,
            token_data.device_info,
            token_data.expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        // Match exact query from RefreshTokenService::refresh_token
        let token_record = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT id, user_id, token_hash, device_info, expires_at, created_at, updated_at, last_used_at
            FROM refresh_tokens
            WHERE token_hash = $1 AND expires_at > NOW()
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token_record)
    }

    async fn revoke_token(&self, token: &str) -> Result<()> {
        // Match exact query from RefreshTokenService::delete_refresh_token_by_hash
        sqlx::query!("DELETE FROM refresh_tokens WHERE token_hash = $1", token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<()> {
        // Match exact query from RefreshTokenService::revoke_all_user_tokens
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<RefreshToken>> {
        let tokens = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT id, user_id, token_hash, device_info, expires_at, created_at, updated_at, last_used_at
            FROM refresh_tokens
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    async fn cleanup_expired_tokens(&self) -> Result<u64> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < NOW()")
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
