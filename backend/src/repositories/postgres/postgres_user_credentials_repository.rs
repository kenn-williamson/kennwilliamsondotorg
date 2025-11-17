use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user_credentials::UserCredentials;
use crate::repositories::traits::user_credentials_repository::UserCredentialsRepository;

pub struct PostgresUserCredentialsRepository {
    pool: PgPool,
}

impl PostgresUserCredentialsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserCredentialsRepository for PostgresUserCredentialsRepository {
    async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials> {
        let credentials = sqlx::query_as::<_, UserCredentials>(
            r#"
            INSERT INTO user_credentials (user_id, password_hash)
            VALUES ($1, $2)
            RETURNING user_id, password_hash, password_updated_at, created_at
            "#,
        )
        .bind(user_id)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(credentials)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>> {
        let credentials = sqlx::query_as::<_, UserCredentials>(
            r#"
            SELECT user_id, password_hash, password_updated_at, created_at
            FROM user_credentials
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(credentials)
    }

    async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_credentials
            SET password_hash = $1, password_updated_at = NOW()
            WHERE user_id = $2
            "#,
        )
        .bind(new_password_hash)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn has_password(&self, user_id: Uuid) -> Result<bool> {
        let result: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(SELECT 1 FROM user_credentials WHERE user_id = $1)
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }
}
