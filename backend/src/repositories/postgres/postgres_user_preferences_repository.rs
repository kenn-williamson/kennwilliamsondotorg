use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user_preferences::UserPreferences;
use crate::repositories::traits::user_preferences_repository::UserPreferencesRepository;

pub struct PostgresUserPreferencesRepository {
    pool: PgPool,
}

impl PostgresUserPreferencesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserPreferencesRepository for PostgresUserPreferencesRepository {
    async fn create(&self, user_id: Uuid) -> Result<UserPreferences> {
        let preferences = sqlx::query_as::<_, UserPreferences>(
            r#"
            INSERT INTO user_preferences (user_id)
            VALUES ($1)
            RETURNING user_id, timer_is_public, timer_show_in_list, notify_blog_posts, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(preferences)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserPreferences>> {
        let preferences = sqlx::query_as::<_, UserPreferences>(
            r#"
            SELECT user_id, timer_is_public, timer_show_in_list, notify_blog_posts, created_at, updated_at
            FROM user_preferences
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(preferences)
    }

    async fn update_timer_settings(
        &self,
        user_id: Uuid,
        is_public: bool,
        show_in_list: bool,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_preferences
            SET timer_is_public = $1,
                timer_show_in_list = $2,
                updated_at = NOW()
            WHERE user_id = $3
            "#,
        )
        .bind(is_public)
        .bind(show_in_list)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_blog_notifications(&self, user_id: Uuid, enabled: bool) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_preferences
            SET notify_blog_posts = $1,
                updated_at = NOW()
            WHERE user_id = $2
            "#,
        )
        .bind(enabled)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_users_with_blog_notifications(&self) -> Result<Vec<Uuid>> {
        let user_ids = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT up.user_id
            FROM user_preferences up
            INNER JOIN users u ON u.id = up.user_id
            WHERE up.notify_blog_posts = true
              AND u.active = true
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_ids)
    }
}
