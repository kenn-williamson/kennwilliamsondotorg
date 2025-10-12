use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user_profile::UserProfile;
use crate::repositories::traits::user_profile_repository::{UpdateProfile, UserProfileRepository};

pub struct PostgresUserProfileRepository {
    pool: PgPool,
}

impl PostgresUserProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserProfileRepository for PostgresUserProfileRepository {
    async fn create(&self, user_id: Uuid) -> Result<UserProfile> {
        let profile = sqlx::query_as::<_, UserProfile>(
            r#"
            INSERT INTO user_profiles (user_id)
            VALUES ($1)
            RETURNING user_id, real_name, bio, avatar_url, location, website, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>> {
        let profile = sqlx::query_as::<_, UserProfile>(
            r#"
            SELECT user_id, real_name, bio, avatar_url, location, website, created_at, updated_at
            FROM user_profiles
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn update(&self, user_id: Uuid, data: UpdateProfile) -> Result<UserProfile> {
        // Build dynamic update query based on which fields are provided
        let profile = sqlx::query_as::<_, UserProfile>(
            r#"
            UPDATE user_profiles
            SET real_name = COALESCE($1, real_name),
                bio = COALESCE($2, bio),
                avatar_url = COALESCE($3, avatar_url),
                location = COALESCE($4, location),
                website = COALESCE($5, website),
                updated_at = NOW()
            WHERE user_id = $6
            RETURNING user_id, real_name, bio, avatar_url, location, website, created_at, updated_at
            "#,
        )
        .bind(data.real_name)
        .bind(data.bio)
        .bind(data.avatar_url)
        .bind(data.location)
        .bind(data.website)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn delete(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_profiles
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
