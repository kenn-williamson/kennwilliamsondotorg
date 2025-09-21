use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

use crate::models::db::user::User;
use crate::repositories::traits::user_repository::{UserRepository, CreateUserData, UserUpdates};

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
        // Match exact query from UserManagementService::create_user
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, display_name, slug)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, display_name, slug, active, created_at, updated_at
            "#,
            user_data.email,
            user_data.password_hash,
            user_data.display_name,
            user_data.slug
        )
        .fetch_one(&self.pool)
        .await?;

        // Add default 'user' role - match exact query from service
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

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        // Match exact query from UserManagementService::authenticate_user
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        // Match exact query from UserManagementService::get_user_by_id
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at FROM users WHERE id = $1",
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
            RETURNING id, email, password_hash, display_name, slug, active, created_at, updated_at
            "#,
            updates.display_name,
            updates.slug,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn slug_exists(&self, slug: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE slug = $1 AND active = true"
        )
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
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE slug = $1 AND id != $2"
        )
        .bind(slug)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count > 0)
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
        // Match exact query from UserManagementService::get_user_roles
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

}
