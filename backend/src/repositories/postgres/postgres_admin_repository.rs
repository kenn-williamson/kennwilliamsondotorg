use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

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
    ) -> Result<Vec<crate::models::db::UserWithRoles>> {
        let limit = limit.unwrap_or(100).min(100); // Default 100, cap at 100 for safety
        let offset = offset.unwrap_or(0);

        if let Some(search_term) = search {
            sqlx::query_as!(
                crate::models::db::UserWithRoles,
                r#"
                SELECT
                    u.id, u.email, u.display_name, u.slug, u.active, u.created_at, u.updated_at,
                    array_agg(r.name) as roles
                FROM users u
                INNER JOIN user_roles ur ON u.id = ur.user_id
                INNER JOIN roles r ON ur.role_id = r.id
                WHERE u.display_name ILIKE $1 OR u.email ILIKE $1 OR u.slug ILIKE $1
                GROUP BY u.id, u.email, u.display_name, u.slug, u.active, u.created_at, u.updated_at
                ORDER BY u.created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                format!("%{}%", search_term),
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch users with roles: {}", e))
        } else {
            sqlx::query_as!(
                crate::models::db::UserWithRoles,
                r#"
                SELECT
                    u.id, u.email, u.display_name, u.slug, u.active, u.created_at, u.updated_at,
                    array_agg(r.name) as roles
                FROM users u
                INNER JOIN user_roles ur ON u.id = ur.user_id
                INNER JOIN roles r ON ur.role_id = r.id
                GROUP BY u.id, u.email, u.display_name, u.slug, u.active, u.created_at, u.updated_at
                ORDER BY u.created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch users with roles: {}", e))
        }
    }

    async fn count_all_users(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn count_active_users(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE active = true")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn get_admin_emails(&self) -> Result<Vec<String>> {
        let emails = sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT u.email
            FROM users u
            INNER JOIN user_roles ur_admin ON u.id = ur_admin.user_id
            INNER JOIN roles r_admin ON ur_admin.role_id = r_admin.id AND r_admin.name = 'admin'
            INNER JOIN user_roles ur_verified ON u.id = ur_verified.user_id
            INNER JOIN roles r_verified ON ur_verified.role_id = r_verified.id AND r_verified.name = 'email-verified'
            WHERE u.active = true
            ORDER BY u.email
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(emails)
    }
}

impl PostgresAdminRepository {
    // Helper method removed - no longer needed with array_agg() approach
}
