use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user::VerificationToken;
use crate::repositories::traits::verification_token_repository::{
    CreateVerificationTokenData, VerificationTokenRepository,
};

/// PostgreSQL implementation of VerificationTokenRepository
pub struct PostgresVerificationTokenRepository {
    pool: PgPool,
}

impl PostgresVerificationTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VerificationTokenRepository for PostgresVerificationTokenRepository {
    async fn create_token(
        &self,
        token_data: &CreateVerificationTokenData,
    ) -> Result<VerificationToken> {
        let token = sqlx::query_as!(
            VerificationToken,
            r#"
            INSERT INTO verification_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, expires_at, created_at
            "#,
            token_data.user_id,
            token_data.token_hash,
            token_data.expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<VerificationToken>> {
        let token = sqlx::query_as!(
            VerificationToken,
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM verification_tokens
            WHERE token_hash = $1 AND expires_at > NOW()
            "#,
            token_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn delete_token(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM verification_tokens
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM verification_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<VerificationToken>> {
        let tokens = sqlx::query_as!(
            VerificationToken,
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM verification_tokens
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
            DELETE FROM verification_tokens
            WHERE expires_at <= NOW()
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
