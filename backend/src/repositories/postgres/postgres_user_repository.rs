use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user::{User, UserWithTimer};
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
        // Create user with basic info only (auth data in separate tables)
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, display_name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, email, display_name, slug, active, created_at, updated_at
            "#,
            user_data.email,
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

    async fn create_user_with_auth_data(
        &self,
        user_data: &CreateUserData,
        password_hash: String,
    ) -> Result<User> {
        // Begin transaction
        let mut tx = self.pool.begin().await?;

        // 1. Create user in users table (core identity only)
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, display_name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, email, display_name, slug, active, created_at, updated_at
            "#,
            user_data.email,
            user_data.display_name,
            user_data.slug
        )
        .fetch_one(&mut *tx)
        .await?;

        // 2. Add default 'user' role
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'user'
            "#,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        // 3. Create credentials in user_credentials table
        sqlx::query!(
            r#"
            INSERT INTO user_credentials (user_id, password_hash)
            VALUES ($1, $2)
            "#,
            user.id,
            password_hash
        )
        .execute(&mut *tx)
        .await?;

        // 4. Create preferences in user_preferences table with defaults
        // Default to public (true, true) to maintain backward compatibility
        sqlx::query!(
            r#"
            INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
            VALUES ($1, true, true)
            "#,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        // 5. Create profile in user_profiles table (empty but row exists)
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (user_id)
            VALUES ($1)
            "#,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction - all or nothing
        tx.commit().await?;

        Ok(user)
    }

    async fn create_oauth_user(&self, user_data: &CreateOAuthUserData) -> Result<User> {
        // Begin transaction for atomic multi-table creation
        let mut tx = self.pool.begin().await?;

        // 1. Create OAuth user in users table (core identity only)
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, display_name, slug)
            VALUES ($1, $2, $3)
            RETURNING id, email, display_name, slug, active, created_at, updated_at
            "#,
            user_data.email,
            user_data.display_name,
            user_data.slug
        )
        .fetch_one(&mut *tx)
        .await?;

        // 2. Add default 'user' role
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'user'
            "#,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        // 3. Add 'email-verified' role (OAuth emails are pre-verified by provider)
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'email-verified'
            "#,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        // 4. Create external login record if Google ID provided
        if let Some(ref google_id) = user_data.google_user_id {
            sqlx::query!(
                r#"
                INSERT INTO user_external_logins (user_id, provider, provider_user_id)
                VALUES ($1, 'google', $2)
                "#,
                user.id,
                google_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // 5. Create preferences in user_preferences table with defaults
        // Default to public (true, true) to maintain backward compatibility
        sqlx::query!(
            r#"
            INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
            VALUES ($1, true, true)
            "#,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        // 6. Create profile in user_profiles table (with real_name if provided)
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (user_id, real_name)
            VALUES ($1, $2)
            "#,
            user.id,
            user_data.real_name
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction - all or nothing
        tx.commit().await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, display_name, slug, active, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_google_user_id(&self, google_user_id: &str) -> Result<Option<User>> {
        // Query through user_external_logins table
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT u.id, u.email, u.display_name, u.slug, u.active, u.created_at, u.updated_at
            FROM users u
            INNER JOIN user_external_logins uel ON u.id = uel.user_id
            WHERE uel.provider = 'google' AND uel.provider_user_id = $1
            "#,
            google_user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, display_name, slug, active, created_at, updated_at FROM users WHERE id = $1",
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
            RETURNING id, email, display_name, slug, active, created_at, updated_at
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
        // Begin transaction for atomic multi-table updates
        let mut tx = self.pool.begin().await?;

        // 1. Insert or update external login record
        sqlx::query!(
            r#"
            INSERT INTO user_external_logins (user_id, provider, provider_user_id)
            VALUES ($1, 'google', $2)
            ON CONFLICT (user_id, provider)
            DO UPDATE SET provider_user_id = $2, updated_at = NOW()
            "#,
            user_id,
            google_user_id
        )
        .execute(&mut *tx)
        .await?;

        // 2. Ensure user_profiles entry exists then update with real_name
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (user_id, real_name)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET real_name = $2, updated_at = NOW()
            "#,
            user_id,
            real_name.as_deref()
        )
        .execute(&mut *tx)
        .await?;

        // 3. Ensure user_preferences entry exists
        // Default to public (true, true) to maintain backward compatibility
        sqlx::query!(
            r#"
            INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
            VALUES ($1, true, true)
            ON CONFLICT (user_id) DO NOTHING
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        // 4. Add 'email-verified' role if not already present (OAuth emails are verified)
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'email-verified'
            ON CONFLICT DO NOTHING
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(())
    }

    async fn update_real_name(&self, user_id: Uuid, real_name: Option<String>) -> Result<()> {
        // Update real_name in user_profiles table
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (user_id, real_name)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET real_name = $2, updated_at = NOW()
            "#,
            user_id,
            real_name.as_deref()
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
        // Update password in user_credentials table
        sqlx::query!(
            r#"
            INSERT INTO user_credentials (user_id, password_hash)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET password_hash = $2, password_updated_at = NOW()
            "#,
            id,
            password_hash
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

    async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        // Reassign user's phrases to system user
        let system_user_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM users WHERE email = 'system@kennwilliamson.org'"
        )
        .fetch_one(&self.pool)
        .await?;

        // Reassign phrases created by this user to system user
        let reassigned_result = sqlx::query(
            "UPDATE phrases SET created_by = $1 WHERE created_by = $2"
        )
        .bind(system_user_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        let reassigned_count = reassigned_result.rows_affected();

        if reassigned_count > 0 {
            log::info!("Reassigned {} phrases from user {} to system user", reassigned_count, user_id);
        }

        // Delete the user - this will cascade delete all other data due to foreign key constraints:
        // - incident_timers (CASCADE DELETE)
        // - user_excluded_phrases (CASCADE DELETE) 
        // - refresh_tokens (CASCADE DELETE)
        // - user_roles (CASCADE DELETE)
        // - phrase_suggestions (CASCADE DELETE)
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        log::info!("Successfully deleted user {} and all associated data", user_id);
        Ok(())
    }

    async fn update_timer_privacy(
        &self,
        user_id: Uuid,
        is_public: bool,
        show_in_list: bool,
    ) -> Result<User> {
        // Validation: Cannot show in list if not public
        if show_in_list && !is_public {
            return Err(anyhow::anyhow!(
                "Cannot enable 'Show in List' when timer is not public"
            ));
        }

        // Update preferences in user_preferences table
        sqlx::query!(
            r#"
            INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id)
            DO UPDATE SET timer_is_public = $2, timer_show_in_list = $3, updated_at = NOW()
            "#,
            user_id,
            is_public,
            show_in_list
        )
        .execute(&self.pool)
        .await?;

        // Return the user
        self.find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    async fn get_users_with_public_timers(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<Vec<UserWithTimer>> {
        let search_pattern = search.map(|s| format!("%{}%", s));

        let users = sqlx::query_as!(
            UserWithTimer,
            r#"
            SELECT DISTINCT ON (u.id)
                u.id,
                u.display_name,
                u.slug,
                u.created_at,
                it.reset_timestamp,
                it.notes
            FROM users u
            INNER JOIN incident_timers it ON u.id = it.user_id
            INNER JOIN user_preferences up ON u.id = up.user_id
            WHERE up.timer_is_public = true
              AND up.timer_show_in_list = true
              AND ($3::text IS NULL OR u.display_name ILIKE $3)
            ORDER BY u.id, it.reset_timestamp DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
            search_pattern.as_deref()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn get_by_slug(&self, slug: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, display_name, slug, active, created_at, updated_at FROM users WHERE slug = $1",
            slug
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
