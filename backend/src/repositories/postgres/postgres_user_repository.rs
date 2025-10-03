use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user::User;
use crate::repositories::traits::user_repository::{
    CreateOAuthUserData, CreateUserData, UserRepository, UserUpdates,
};

/// PostgreSQL implementation of UserRepository
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user_data: &CreateUserData) -> Result<User> {
        // Create user with email/password
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, display_name, slug)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, display_name, slug, active, real_name, google_user_id, created_at, updated_at
            "#,
            user_data.email,
            user_data.password_hash,
            user_data.display_name,
            user_data.slug
        )
        .fetch_one(&self.pool)
        .await?;

        // Add default 'user' role
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'user'
            "#,
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create_oauth_user(&self, user_data: &CreateOAuthUserData) -> Result<User> {
        // Create OAuth user (no password)
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, display_name, slug, real_name, google_user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password_hash, display_name, slug, active, real_name, google_user_id, created_at, updated_at
            "#,
            user_data.email,
            user_data.display_name,
            user_data.slug,
            user_data.real_name,
            user_data.google_user_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Add default 'user' role
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'user'
            "#,
            user.id
        )
        .execute(&self.pool)
        .await?;

        // Add 'email-verified' role (OAuth emails are pre-verified by provider)
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'email-verified'
            "#,
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, real_name, google_user_id, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_google_user_id(&self, google_user_id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, real_name, google_user_id, created_at, updated_at FROM users WHERE google_user_id = $1",
            google_user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, real_name, google_user_id, created_at, updated_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User> {
        // Update only user-controlled fields (display_name and slug)
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET display_name = $1, slug = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, email, password_hash, display_name, slug, active, real_name, google_user_id, created_at, updated_at
            "#,
            updates.display_name,
            updates.slug,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn link_google_account(
        &self,
        user_id: Uuid,
        google_user_id: &str,
        real_name: Option<String>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET google_user_id = $1, real_name = $2, updated_at = NOW()
            WHERE id = $3
            "#,
            google_user_id,
            real_name.as_deref(),
            user_id
        )
        .execute(&self.pool)
        .await?;

        // Add 'email-verified' role if not already present (OAuth emails are verified)
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'email-verified'
            ON CONFLICT DO NOTHING
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_real_name(&self, user_id: Uuid, real_name: Option<String>) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET real_name = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            real_name.as_deref(),
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn slug_exists(&self, slug: &str) -> Result<bool> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE slug = $1 AND active = true")
                .bind(slug)
                .fetch_one(&self.pool)
                .await?;

        Ok(count > 0)
    }

    async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()> {
        // Match exact query from UserManagementService::change_password
        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            password_hash,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool> {
        // Match exact query from UserManagementService::slug_exists_excluding_user
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE slug = $1 AND id != $2")
                .bind(slug)
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?;

        Ok(count > 0)
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
        let roles = sqlx::query!(
            r#"
            SELECT r.name
            FROM roles r
            JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(roles.into_iter().map(|r| r.name).collect())
    }

    async fn add_role_to_user(&self, user_id: Uuid, role_name: &str) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = $2
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
            user_id,
            role_name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn has_role(&self, user_id: Uuid, role_name: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM user_roles ur
            JOIN roles r ON r.id = ur.role_id
            WHERE ur.user_id = $1 AND r.name = $2
            "#,
        )
        .bind(user_id)
        .bind(role_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}
