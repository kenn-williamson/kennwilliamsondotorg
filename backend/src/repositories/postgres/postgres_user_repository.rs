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

    async fn find_by_slug(&self, slug: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE slug = $1 AND active = true"
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User> {
        // Match exact query from UserManagementService::update_profile
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

    async fn email_exists(&self, email: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE email = $1 AND active = true"
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
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

    async fn deactivate_user(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE users SET active = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
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

    async fn get_all_users_with_roles(
        &self,
        search: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::models::api::UserWithRoles>> {
        let limit = limit.unwrap_or(50).min(100); // Cap at 100 for safety
        let offset = offset.unwrap_or(0);

        // First, get the users
        let users = if let Some(search_term) = search {
            sqlx::query_as!(
                User,
                r#"
                SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at
                FROM users
                WHERE email ILIKE $1 OR display_name ILIKE $1 OR slug ILIKE $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                format!("%{}%", search_term),
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                User,
                r#"
                SELECT id, email, password_hash, display_name, slug, active, created_at, updated_at
                FROM users
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        };

        // Then, for each user, get their roles
        let mut users_with_roles = Vec::new();
        for user in users {
            let roles = self.get_user_roles(user.id).await?;
            
            users_with_roles.push(crate::models::api::UserWithRoles {
                id: user.id,
                email: user.email,
                display_name: user.display_name,
                slug: user.slug,
                active: user.active,
                created_at: user.created_at,
                updated_at: user.updated_at,
                roles,
            });
        }

        Ok(users_with_roles)
    }

    async fn add_user_role(&self, user_id: Uuid, role: &str) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = $2
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
            user_id,
            role
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_user_role(&self, user_id: Uuid, role: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM user_roles 
            WHERE user_id = $1 AND role_id = (SELECT id FROM roles WHERE name = $2)
            "#,
            user_id,
            role
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn count_all_users(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }

    async fn count_active_users(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE active = true"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }
}
