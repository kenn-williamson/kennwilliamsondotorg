use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user_external_login::UserExternalLogin;
use crate::repositories::traits::user_external_login_repository::{
    CreateExternalLogin, UserExternalLoginRepository,
};

pub struct PostgresUserExternalLoginRepository {
    pool: PgPool,
}

impl PostgresUserExternalLoginRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserExternalLoginRepository for PostgresUserExternalLoginRepository {
    async fn create(&self, data: CreateExternalLogin) -> Result<UserExternalLogin> {
        let login = sqlx::query_as::<_, UserExternalLogin>(
            r#"
            INSERT INTO user_external_logins (user_id, provider, provider_user_id, linked_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
            "#,
        )
        .bind(data.user_id)
        .bind(data.provider)
        .bind(data.provider_user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(login)
    }

    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserExternalLogin>> {
        let login = sqlx::query_as::<_, UserExternalLogin>(
            r#"
            SELECT id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
            FROM user_external_logins
            WHERE provider = $1 AND provider_user_id = $2
            "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(login)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserExternalLogin>> {
        let logins = sqlx::query_as::<_, UserExternalLogin>(
            r#"
            SELECT id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
            FROM user_external_logins
            WHERE user_id = $1
            ORDER BY linked_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(logins)
    }

    async fn unlink_provider(&self, user_id: Uuid, provider: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_external_logins
            WHERE user_id = $1 AND provider = $2
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_external_logins
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn is_provider_linked(&self, user_id: Uuid, provider: &str) -> Result<bool> {
        let result: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_external_logins
                WHERE user_id = $1 AND provider = $2
            )
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }
}
