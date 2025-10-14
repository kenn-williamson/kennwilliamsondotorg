use crate::models::db::refresh_token::RefreshToken;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use sqlx::PgPool;
use anyhow::Result;

/// Builder for creating RefreshToken instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust,ignore
/// // Minimal token with defaults
/// let token = RefreshTokenBuilder::new()
///     .with_user_id(user_id)
///     .with_token_hash("hash123")
///     .persist(pool).await?;
///
/// // Token expiring in 7 days
/// let token = RefreshTokenBuilder::new()
///     .with_user_id(user_id)
///     .with_token_hash("hash456")
///     .expires_in_days(7)
///     .persist(pool).await?;
///
/// // Token with device info
/// let token = RefreshTokenBuilder::new()
///     .with_user_id(user_id)
///     .with_token_hash("hash789")
///     .with_device_info(json!({"device": "Chrome", "os": "Linux"}))
///     .persist(pool).await?;
/// ```
#[derive(Clone)]
pub struct RefreshTokenBuilder {
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    token_hash: Option<String>,
    device_info: Option<Option<serde_json::Value>>, // Option<Option<...>> to distinguish between "not set" and "explicitly None"
    expires_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    last_used_at: Option<Option<DateTime<Utc>>>,
}

impl RefreshTokenBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            id: None,
            user_id: None,
            token_hash: None,
            device_info: None,
            expires_at: None,
            created_at: None,
            updated_at: None,
            last_used_at: None,
        }
    }

    /// Build the RefreshToken with defaults for any unset fields (in-memory only, no database)
    pub fn build(self) -> RefreshToken {
        let now = Utc::now();

        RefreshToken {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            user_id: self.user_id.unwrap_or_else(Uuid::new_v4),
            token_hash: self.token_hash.unwrap_or_else(|| format!("test_hash_{}", Uuid::new_v4())),
            device_info: self.device_info.unwrap_or(None),
            expires_at: self.expires_at.unwrap_or(now + Duration::days(7)),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
            last_used_at: self.last_used_at.unwrap_or(None),
        }
    }

    /// Persist RefreshToken to database (for integration tests)
    pub async fn persist(self, pool: &PgPool) -> Result<RefreshToken> {
        // Generate defaults
        let now = Utc::now();
        let user_id = self.user_id.expect("user_id is required for persist()");
        let token_hash = self.token_hash.expect("token_hash is required for persist()");
        let device_info = self.device_info.unwrap_or(None);
        let expires_at = self.expires_at.unwrap_or(now + Duration::days(7));
        let last_used_at = self.last_used_at.unwrap_or(None);

        let token = sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at, last_used_at)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *"
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(device_info)
        .bind(expires_at)
        .bind(last_used_at)
        .fetch_one(pool)
        .await?;

        Ok(token)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set a specific token ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the user ID (required for persist())
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the token hash (required for persist())
    pub fn with_token_hash(mut self, token_hash: impl Into<String>) -> Self {
        self.token_hash = Some(token_hash.into());
        self
    }

    /// Set device info
    pub fn with_device_info(mut self, device_info: serde_json::Value) -> Self {
        self.device_info = Some(Some(device_info));
        self
    }

    /// Set device info to None explicitly
    pub fn without_device_info(mut self) -> Self {
        self.device_info = Some(None);
        self
    }

    /// Set the expiration timestamp
    pub fn expires_at(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set the expiration to a relative time from now (days)
    pub fn expires_in_days(mut self, days: i64) -> Self {
        self.expires_at = Some(Utc::now() + Duration::days(days));
        self
    }

    /// Set the expiration to a relative time from now (hours)
    pub fn expires_in_hours(mut self, hours: i64) -> Self {
        self.expires_at = Some(Utc::now() + Duration::hours(hours));
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

    /// Set last_used_at timestamp
    pub fn last_used_at(mut self, last_used_at: DateTime<Utc>) -> Self {
        self.last_used_at = Some(Some(last_used_at));
        self
    }

    /// Set last_used_at to None explicitly
    pub fn never_used(mut self) -> Self {
        self.last_used_at = Some(None);
        self
    }
}

impl Default for RefreshTokenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_valid_token_with_defaults() {
        let token = RefreshTokenBuilder::new().build();

        assert!(!token.id.is_nil());
        assert!(!token.user_id.is_nil());
        assert!(!token.token_hash.is_empty());
        assert!(token.device_info.is_none());
        assert!(token.expires_at > Utc::now());
        assert!(token.last_used_at.is_none());
    }

    #[test]
    fn test_builder_with_user_and_hash() {
        let user_id = Uuid::new_v4();
        let token = RefreshTokenBuilder::new()
            .with_user_id(user_id)
            .with_token_hash("custom_hash")
            .build();

        assert_eq!(token.user_id, user_id);
        assert_eq!(token.token_hash, "custom_hash");
    }

    #[test]
    fn test_builder_expires_in_days() {
        let now = Utc::now();
        let token = RefreshTokenBuilder::new()
            .expires_in_days(30)
            .build();

        // Should be approximately 30 days from now (allow 1 second tolerance)
        let expected = now + Duration::days(30);
        let diff = (token.expires_at - expected).num_seconds().abs();
        assert!(diff <= 1, "Timestamp difference: {} seconds", diff);
    }

    #[test]
    fn test_builder_expires_in_hours() {
        let now = Utc::now();
        let token = RefreshTokenBuilder::new()
            .expires_in_hours(24)
            .build();

        // Should be approximately 24 hours from now (allow 1 second tolerance)
        let expected = now + Duration::hours(24);
        let diff = (token.expires_at - expected).num_seconds().abs();
        assert!(diff <= 1, "Timestamp difference: {} seconds", diff);
    }

    #[test]
    fn test_builder_with_device_info() {
        use serde_json::json;

        let device_info = json!({"device": "Chrome", "os": "Linux"});
        let token = RefreshTokenBuilder::new()
            .with_device_info(device_info.clone())
            .build();

        assert_eq!(token.device_info, Some(device_info));
    }

    #[test]
    fn test_builder_without_device_info() {
        let token = RefreshTokenBuilder::new()
            .without_device_info()
            .build();

        assert!(token.device_info.is_none());
    }

    #[test]
    fn test_builder_last_used_at() {
        let used_time = Utc::now() - Duration::hours(2);
        let token = RefreshTokenBuilder::new()
            .last_used_at(used_time)
            .build();

        assert_eq!(token.last_used_at, Some(used_time));
    }

    #[test]
    fn test_builder_never_used() {
        let token = RefreshTokenBuilder::new()
            .never_used()
            .build();

        assert!(token.last_used_at.is_none());
    }

    #[test]
    fn test_builder_chaining() {
        let user_id = Uuid::new_v4();
        let expires = Utc::now() + Duration::days(15);
        let token = RefreshTokenBuilder::new()
            .with_user_id(user_id)
            .with_token_hash("chained_hash")
            .expires_at(expires)
            .never_used()
            .build();

        assert_eq!(token.user_id, user_id);
        assert_eq!(token.token_hash, "chained_hash");
        assert_eq!(token.expires_at, expires);
        assert!(token.last_used_at.is_none());
    }
}
