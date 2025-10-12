use backend::models::db::user::User;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::PgPool;
use anyhow::Result;

/// Builder for creating User instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust
/// // Minimal user with defaults
/// let user = UserBuilder::new().persist(pool);
///
/// // Verified user with custom email
/// let user = UserBuilder::new()
///     .verified()
///     .with_email("test@example.com")
///     .persist(pool);
///
/// // OAuth user
/// let user = UserBuilder::new()
///     .oauth("google_id_123", "Real Name")
///     .with_email("oauth@example.com")
///     .persist(pool);
///
/// // User with public timer
/// let user = UserBuilder::new()
///     .with_public_timer(true, true)
///     .persist(pool);
/// ```
#[derive(Clone)]
pub struct UserBuilder {
    id: Option<Uuid>,
    email: Option<String>,
    password_hash: Option<Option<String>>, // Option<Option<...>> to distinguish between "not set" and "explicitly None"
    display_name: Option<String>,
    slug: Option<String>,
    active: Option<bool>,
    real_name: Option<Option<String>>,
    google_user_id: Option<Option<String>>,
    timer_is_public: Option<bool>,
    timer_show_in_list: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl UserBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            id: None,
            email: None,
            password_hash: None,
            display_name: None,
            slug: None,
            active: None,
            real_name: None,
            google_user_id: None,
            timer_is_public: None,
            timer_show_in_list: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Build the User with defaults for any unset fields (in-memory only, no database)
    pub fn build(self) -> User {
        let now = Utc::now();
        let id = self.id.unwrap_or_else(Uuid::new_v4);
        let slug = self.slug.unwrap_or_else(|| format!("test-user-{}", id));

        User {
            id,
            email: self.email.unwrap_or_else(|| format!("test-{}@example.com", id)),
            password_hash: self.password_hash.unwrap_or(Some("hashed_password".to_string())),
            display_name: self.display_name.unwrap_or("Test User".to_string()),
            slug,
            active: self.active.unwrap_or(true),
            real_name: self.real_name.unwrap_or(None),
            google_user_id: self.google_user_id.unwrap_or(None),
            timer_is_public: self.timer_is_public.unwrap_or(false),
            timer_show_in_list: self.timer_show_in_list.unwrap_or(false),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        }
    }

    /// Persist User to database with multi-table setup (for integration tests)
    /// Creates entries in users, user_credentials, user_external_logins, user_profiles, and user_preferences
    pub async fn persist(self, pool: &PgPool) -> Result<User> {
        // Start transaction for atomic multi-table creation
        let mut tx = pool.begin().await?;

        // Generate defaults
        let id = self.id.unwrap_or_else(Uuid::new_v4);
        let email = self.email.unwrap_or_else(|| format!("test-{}@example.com", id));
        let display_name = self.display_name.unwrap_or("Test User".to_string());
        let slug = self.slug.unwrap_or_else(|| format!("test-user-{}", id));

        // 1. Create user (core identity)
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, display_name, slug)
             VALUES ($1, $2, $3)
             RETURNING *"
        )
        .bind(&email)
        .bind(&display_name)
        .bind(&slug)
        .fetch_one(&mut *tx)
        .await?;

        // 2. Create credentials if password provided
        if let Some(Some(password)) = self.password_hash {
            // Hash if not already hashed (starts with $2b$ = bcrypt)
            let hash = if password.starts_with("$2b$") {
                password
            } else {
                bcrypt::hash(password, 4)?  // Low cost for tests
            };

            sqlx::query(
                "INSERT INTO user_credentials (user_id, password_hash)
                 VALUES ($1, $2)"
            )
            .bind(user.id)
            .bind(hash)
            .execute(&mut *tx)
            .await?;
        }

        // 3. Create external logins if OAuth IDs provided
        if let Some(Some(google_id)) = self.google_user_id {
            sqlx::query(
                "INSERT INTO user_external_logins (user_id, provider, provider_user_id, linked_at)
                 VALUES ($1, 'google', $2, NOW())"
            )
            .bind(user.id)
            .bind(google_id)
            .execute(&mut *tx)
            .await?;
        }

        // 4. Create profile if real_name provided
        if let Some(Some(real_name)) = self.real_name {
            sqlx::query(
                "INSERT INTO user_profiles (user_id, real_name)
                 VALUES ($1, $2)"
            )
            .bind(user.id)
            .bind(real_name)
            .execute(&mut *tx)
            .await?;
        }

        // 5. Create preferences (always - required for all users)
        let timer_public = self.timer_is_public.unwrap_or(false);
        let timer_in_list = self.timer_show_in_list.unwrap_or(false);

        sqlx::query(
            "INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
             VALUES ($1, $2, $3)"
        )
        .bind(user.id)
        .bind(timer_public)
        .bind(timer_in_list)
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(user)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set a specific user ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the email address
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set the password hash (Some value)
    pub fn with_password(mut self, password_hash: impl Into<String>) -> Self {
        self.password_hash = Some(Some(password_hash.into()));
        self
    }

    /// Set password_hash to None (for OAuth users)
    pub fn without_password(mut self) -> Self {
        self.password_hash = Some(None);
        self
    }

    /// Set the display name
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// Set the slug
    pub fn with_slug(mut self, slug: impl Into<String>) -> Self {
        self.slug = Some(slug.into());
        self
    }

    /// Set active status
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Mark user as inactive
    pub fn inactive(self) -> Self {
        self.active(false)
    }

    /// Set real name
    pub fn with_real_name(mut self, real_name: impl Into<String>) -> Self {
        self.real_name = Some(Some(real_name.into()));
        self
    }

    /// Set Google user ID
    pub fn with_google_id(mut self, google_user_id: impl Into<String>) -> Self {
        self.google_user_id = Some(Some(google_user_id.into()));
        self
    }

    /// Set timer privacy settings
    pub fn with_public_timer(mut self, is_public: bool, show_in_list: bool) -> Self {
        self.timer_is_public = Some(is_public);
        self.timer_show_in_list = Some(show_in_list);
        self
    }

    /// Set created_at timestamp
    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Set updated_at timestamp
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    // ============================================================================
    // CONVENIENCE PRESETS
    // ============================================================================

    /// Create a verified user (convenience method for common scenario)
    /// Note: This only sets the user fields. You still need to add the
    /// "email-verified" role separately if testing role-based access.
    pub fn verified(self) -> Self {
        // Verified users are just regular users with roles assigned in the database
        // This method exists for semantic clarity in tests
        self
    }

    /// Create an OAuth user (no password, has Google ID and real name)
    pub fn oauth(mut self, google_user_id: impl Into<String>, real_name: impl Into<String>) -> Self {
        self.password_hash = Some(None);
        self.google_user_id = Some(Some(google_user_id.into()));
        self.real_name = Some(Some(real_name.into()));
        self
    }

    /// Create an admin user preset (convenience)
    /// Note: This only sets the user fields. You still need to add the
    /// "admin" role separately if testing role-based access.
    pub fn admin(self) -> Self {
        self.with_display_name("Admin User")
            .with_slug("admin-user")
    }

    /// Create a password user ready to persist
    pub fn password_user(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self::new()
            .with_email(email)
            .with_password(password)
    }

    /// Create an OAuth user ready to persist
    pub fn oauth_user(email: impl Into<String>, provider_id: impl Into<String>) -> Self {
        Self::new()
            .with_email(email)
            .with_google_id(provider_id)
            .without_password()
    }

    /// Create a hybrid user (password + OAuth)
    pub fn hybrid_user(
        email: impl Into<String>,
        password: impl Into<String>,
        google_id: impl Into<String>
    ) -> Self {
        Self::new()
            .with_email(email)
            .with_password(password)
            .with_google_id(google_id)
    }
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_valid_user_with_defaults() {
        let user = UserBuilder::new().build();

        assert!(!user.email.is_empty());
        assert!(user.password_hash.is_some());
        assert_eq!(user.display_name, "Test User");
        assert!(!user.slug.is_empty());
        assert!(user.active);
        assert!(!user.timer_is_public);
        assert!(!user.timer_show_in_list);
    }

    #[test]
    fn test_builder_with_custom_email() {
        let user = UserBuilder::new()
            .with_email("custom@example.com")
            .build();

        assert_eq!(user.email, "custom@example.com");
    }

    #[test]
    fn test_builder_oauth_preset() {
        let user = UserBuilder::new()
            .oauth("google_123", "John Doe")
            .with_email("oauth@example.com")
            .build();

        assert_eq!(user.email, "oauth@example.com");
        assert!(user.password_hash.is_none());
        assert_eq!(user.google_user_id, Some("google_123".to_string()));
        assert_eq!(user.real_name, Some("John Doe".to_string()));
    }

    #[test]
    fn test_builder_verified_user() {
        let user = UserBuilder::new()
            .verified()
            .with_email("verified@example.com")
            .build();

        assert_eq!(user.email, "verified@example.com");
        assert!(user.active);
    }

    #[test]
    fn test_builder_public_timer() {
        let user = UserBuilder::new()
            .with_public_timer(true, true)
            .build();

        assert!(user.timer_is_public);
        assert!(user.timer_show_in_list);
    }

    #[test]
    fn test_builder_inactive_user() {
        let user = UserBuilder::new()
            .inactive()
            .build();

        assert!(!user.active);
    }

    #[test]
    fn test_builder_specific_id() {
        let id = Uuid::new_v4();
        let user = UserBuilder::new()
            .with_id(id)
            .build();

        assert_eq!(user.id, id);
    }

    #[test]
    fn test_builder_without_password() {
        let user = UserBuilder::new()
            .without_password()
            .build();

        assert!(user.password_hash.is_none());
    }

    #[test]
    fn test_builder_chaining() {
        let user = UserBuilder::new()
            .with_email("chain@example.com")
            .with_display_name("Chained User")
            .with_slug("chained-user")
            .with_real_name("Real Name")
            .active(true)
            .build();

        assert_eq!(user.email, "chain@example.com");
        assert_eq!(user.display_name, "Chained User");
        assert_eq!(user.slug, "chained-user");
        assert_eq!(user.real_name, Some("Real Name".to_string()));
        assert!(user.active);
    }
}
