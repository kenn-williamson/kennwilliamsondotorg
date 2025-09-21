use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

use crate::repositories::traits::admin_repository::AdminRepository;

/// PostgreSQL implementation of AdminRepository
pub struct PostgresAdminRepository {
    pool: PgPool,
}

impl PostgresAdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AdminRepository for PostgresAdminRepository {
    async fn update_user_status(&self, user_id: Uuid, active: bool) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET active = $1, updated_at = NOW() WHERE id = $2",
            active,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
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
                crate::models::db::user::User,
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
                crate::models::db::user::User,
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

impl PostgresAdminRepository {
    /// Helper method to get user roles (used internally)
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
}
