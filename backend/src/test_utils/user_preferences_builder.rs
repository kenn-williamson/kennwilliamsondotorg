use crate::models::db::user_preferences::UserPreferences;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Builder for creating UserPreferences instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust,ignore
/// // Minimal preferences with defaults
/// let prefs = UserPreferencesBuilder::new()
///     .with_user_id(user_id)
///     .build();
///
/// // Public timer
/// let prefs = UserPreferencesBuilder::new()
///     .with_user_id(user_id)
///     .public_timer()
///     .build();
///
/// // Opted out of blog notifications
/// let prefs = UserPreferencesBuilder::new()
///     .with_user_id(user_id)
///     .no_blog_notifications()
///     .build();
/// ```
#[derive(Clone)]
pub struct UserPreferencesBuilder {
    user_id: Option<Uuid>,
    timer_is_public: Option<bool>,
    timer_show_in_list: Option<bool>,
    notify_blog_posts: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl UserPreferencesBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            user_id: None,
            timer_is_public: None,
            timer_show_in_list: None,
            notify_blog_posts: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Build the UserPreferences with defaults for any unset fields (in-memory only)
    pub fn build(self) -> UserPreferences {
        let now = Utc::now();

        UserPreferences {
            user_id: self.user_id.unwrap_or_else(Uuid::new_v4),
            timer_is_public: self.timer_is_public.unwrap_or(false),
            timer_show_in_list: self.timer_show_in_list.unwrap_or(false),
            notify_blog_posts: self.notify_blog_posts.unwrap_or(true), // Default: opted-in
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        }
    }

    /// Persist UserPreferences to database (for integration tests)
    /// Note: User must exist first due to foreign key constraint
    pub async fn persist(self, pool: &PgPool) -> Result<UserPreferences> {
        let user_id = self.user_id.expect("user_id is required for persist()");
        let timer_is_public = self.timer_is_public.unwrap_or(false);
        let timer_show_in_list = self.timer_show_in_list.unwrap_or(false);
        let notify_blog_posts = self.notify_blog_posts.unwrap_or(true);

        let prefs = sqlx::query_as::<_, UserPreferences>(
            "INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list, notify_blog_posts)
             VALUES ($1, $2, $3, $4)
             RETURNING *",
        )
        .bind(user_id)
        .bind(timer_is_public)
        .bind(timer_show_in_list)
        .bind(notify_blog_posts)
        .fetch_one(pool)
        .await?;

        Ok(prefs)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set the user ID (required for persist())
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set timer_is_public
    pub fn timer_is_public(mut self, is_public: bool) -> Self {
        self.timer_is_public = Some(is_public);
        self
    }

    /// Set timer_show_in_list
    pub fn timer_show_in_list(mut self, show_in_list: bool) -> Self {
        self.timer_show_in_list = Some(show_in_list);
        self
    }

    /// Set notify_blog_posts
    pub fn notify_blog_posts(mut self, notify: bool) -> Self {
        self.notify_blog_posts = Some(notify);
        self
    }

    /// Convenience: Make timer public and show in list
    pub fn public_timer(mut self) -> Self {
        self.timer_is_public = Some(true);
        self.timer_show_in_list = Some(true);
        self
    }

    /// Convenience: Make timer private (not public, not in list)
    pub fn private_timer(mut self) -> Self {
        self.timer_is_public = Some(false);
        self.timer_show_in_list = Some(false);
        self
    }

    /// Convenience: Opt out of blog notifications
    pub fn no_blog_notifications(mut self) -> Self {
        self.notify_blog_posts = Some(false);
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
}

impl Default for UserPreferencesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_valid_preferences_with_defaults() {
        let prefs = UserPreferencesBuilder::new().build();

        assert!(!prefs.user_id.is_nil());
        assert!(!prefs.timer_is_public);
        assert!(!prefs.timer_show_in_list);
        assert!(prefs.notify_blog_posts); // Default: opted-in
    }

    #[test]
    fn test_builder_with_user_id() {
        let user_id = Uuid::new_v4();
        let prefs = UserPreferencesBuilder::new().with_user_id(user_id).build();

        assert_eq!(prefs.user_id, user_id);
    }

    #[test]
    fn test_builder_public_timer() {
        let prefs = UserPreferencesBuilder::new().public_timer().build();

        assert!(prefs.timer_is_public);
        assert!(prefs.timer_show_in_list);
    }

    #[test]
    fn test_builder_private_timer() {
        let prefs = UserPreferencesBuilder::new().private_timer().build();

        assert!(!prefs.timer_is_public);
        assert!(!prefs.timer_show_in_list);
    }

    #[test]
    fn test_builder_no_blog_notifications() {
        let prefs = UserPreferencesBuilder::new()
            .no_blog_notifications()
            .build();

        assert!(!prefs.notify_blog_posts);
    }

    #[test]
    fn test_builder_chaining() {
        let user_id = Uuid::new_v4();
        let prefs = UserPreferencesBuilder::new()
            .with_user_id(user_id)
            .public_timer()
            .no_blog_notifications()
            .build();

        assert_eq!(prefs.user_id, user_id);
        assert!(prefs.timer_is_public);
        assert!(prefs.timer_show_in_list);
        assert!(!prefs.notify_blog_posts);
    }

    #[test]
    fn test_builder_individual_settings() {
        let prefs = UserPreferencesBuilder::new()
            .timer_is_public(true)
            .timer_show_in_list(false)
            .notify_blog_posts(false)
            .build();

        assert!(prefs.timer_is_public);
        assert!(!prefs.timer_show_in_list);
        assert!(!prefs.notify_blog_posts);
    }
}
