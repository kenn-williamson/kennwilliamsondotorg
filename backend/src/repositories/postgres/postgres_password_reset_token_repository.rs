use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user::PasswordResetToken;
use crate::repositories::traits::password_reset_token_repository::{
    CreatePasswordResetTokenData, PasswordResetTokenRepository,
};

/// PostgreSQL implementation of PasswordResetTokenRepository
pub struct PostgresPasswordResetTokenRepository {
    pool: PgPool,
}

impl PostgresPasswordResetTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PasswordResetTokenRepository for PostgresPasswordResetTokenRepository {
    async fn create_token(
        &self,
        token_data: &CreatePasswordResetTokenData,
    ) -> Result<PasswordResetToken> {
        let token = sqlx::query_as!(
            PasswordResetToken,
            r#"
            INSERT INTO password_reset_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, expires_at, used_at, created_at
            "#,
            token_data.user_id,
            token_data.token_hash,
            token_data.expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<PasswordResetToken>> {
        let token = sqlx::query_as!(
            PasswordResetToken,
            r#"
            SELECT id, user_id, token_hash, expires_at, used_at, created_at
            FROM password_reset_tokens
            WHERE token_hash = $1 AND expires_at > NOW() AND used_at IS NULL
            "#,
            token_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn mark_token_used(&self, token_hash: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE password_reset_tokens
            SET used_at = NOW()
            WHERE token_hash = $1 AND used_at IS NULL
            "#,
            token_hash
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<PasswordResetToken>> {
        let tokens = sqlx::query_as!(
            PasswordResetToken,
            r#"
            SELECT id, user_id, token_hash, expires_at, used_at, created_at
            FROM password_reset_tokens
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    async fn delete_expired_tokens(&self) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens
            WHERE expires_at <= NOW()
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
