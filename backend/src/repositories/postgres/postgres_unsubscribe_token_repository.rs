use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::unsubscribe_token::UnsubscribeTokenInfo;
use crate::repositories::traits::unsubscribe_token_repository::UnsubscribeTokenRepository;

pub struct PostgresUnsubscribeTokenRepository {
    pool: PgPool,
}

impl PostgresUnsubscribeTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnsubscribeTokenRepository for PostgresUnsubscribeTokenRepository {
    async fn create_or_replace(
        &self,
        user_id: Uuid,
        email_type: &str,
        token_hash: &str,
    ) -> Result<bool> {
        // Use INSERT ... ON CONFLICT to upsert
        // xmax = 0 indicates an insert (new row), xmax != 0 indicates an update
        let result = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO unsubscribe_tokens (user_id, email_type, token_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, email_type)
            DO UPDATE SET token_hash = $3, created_at = NOW()
            RETURNING CASE WHEN xmax = 0 THEN 1 ELSE 0 END
            "#,
        )
        .bind(user_id)
        .bind(email_type)
        .bind(token_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(result == 1)
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<UnsubscribeTokenInfo>> {
        let result = sqlx::query_as::<_, (Uuid, String)>(
            r#"
            SELECT user_id, email_type
            FROM unsubscribe_tokens
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(user_id, email_type)| UnsubscribeTokenInfo {
            user_id,
            email_type,
        }))
    }

    async fn delete_by_user_and_type(&self, user_id: Uuid, email_type: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM unsubscribe_tokens
            WHERE user_id = $1 AND email_type = $2
            "#,
        )
        .bind(user_id)
        .bind(email_type)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_all_for_user(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM unsubscribe_tokens
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
