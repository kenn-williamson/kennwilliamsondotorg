use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use rand::{distributions::Alphanumeric, Rng};

use crate::models::api::UserWithRoles;

/// User management service for admin operations
pub struct UserManagementService;

impl UserManagementService {
    /// Get all users with optional search
    pub async fn get_users(
        pool: &PgPool,
        search: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<UserWithRoles>, sqlx::Error> {
        let search_term = search.unwrap_or_default();
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let users = sqlx::query!(
            r#"
            SELECT 
                u.id,
                u.email,
                u.display_name,
                u.slug,
                u.active,
                u.created_at,
                u.updated_at,
                COALESCE(
                    ARRAY_AGG(r.name) FILTER (WHERE r.name IS NOT NULL),
                    ARRAY[]::TEXT[]
                ) as roles
            FROM users u
            LEFT JOIN user_roles ur ON u.id = ur.user_id
            LEFT JOIN roles r ON ur.role_id = r.id
            WHERE ($1 = '' OR u.display_name ILIKE $2 OR u.email ILIKE $2)
            GROUP BY u.id, u.email, u.display_name, u.slug, u.active, u.created_at, u.updated_at
            ORDER BY u.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            search_term,
            format!("%{}%", search_term),
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let users = users.into_iter().map(|row| UserWithRoles {
            id: row.id,
            email: row.email,
            display_name: row.display_name,
            slug: row.slug,
            active: row.active,
            created_at: row.created_at,
            updated_at: row.updated_at,
            roles: row.roles.unwrap_or_default(),
        }).collect();

        Ok(users)
    }


    /// Deactivate a user
    pub async fn deactivate_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        // Start a transaction
        let mut tx = pool.begin().await?;

        // Deactivate user
        sqlx::query!(
            "UPDATE users SET active = false WHERE id = $1",
            user_id
        )
        .execute(&mut *tx)
        .await?;

        // Delete all refresh tokens for the user
        sqlx::query!(
            "DELETE FROM refresh_tokens WHERE user_id = $1",
            user_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(())
    }

    /// Activate a user
    pub async fn activate_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET active = true WHERE id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Reset user password
    pub async fn reset_user_password(pool: &PgPool, user_id: Uuid) -> Result<String, sqlx::Error> {
        // Generate random password
        let new_password = generate_random_password();
        let password_hash = hash(&new_password, DEFAULT_COST)
            .map_err(|e| sqlx::Error::Protocol(format!("Password hashing failed: {}", e)))?;

        // Update password
        sqlx::query!(
            "UPDATE users SET password_hash = $1 WHERE id = $2",
            password_hash,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(new_password)
    }

    /// Promote user to admin
    pub async fn promote_to_admin(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        // Get admin role ID
        let admin_role_id = sqlx::query_scalar!(
            "SELECT id FROM roles WHERE name = 'admin'"
        )
        .fetch_one(pool)
        .await?;

        // Check if user already has admin role
        let existing_role = sqlx::query_scalar!(
            "SELECT id FROM user_roles WHERE user_id = $1 AND role_id = $2",
            user_id,
            admin_role_id
        )
        .fetch_optional(pool)
        .await?;

        if existing_role.is_none() {
            // Add admin role
            sqlx::query!(
                "INSERT INTO user_roles (id, user_id, role_id, assigned_at) VALUES (uuid_generate_v7(), $1, $2, NOW())",
                user_id,
                admin_role_id
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// Check if user is admin
    pub async fn is_user_admin(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let admin_role = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) > 0
            FROM user_roles ur
            JOIN roles r ON ur.role_id = r.id
            WHERE ur.user_id = $1 AND r.name = 'admin'
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(admin_role.unwrap_or(false))
    }
}


/// Generate a random password for admin reset
fn generate_random_password() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
