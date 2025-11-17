use crate::models::db::phrase::{Phrase, PhraseSuggestion};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Builder for creating Phrase instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust,ignore
/// // Minimal phrase with defaults
/// let phrase = PhraseBuilder::new()
///     .with_created_by(admin_id)
///     .persist(pool).await?;
///
/// // Phrase with custom text
/// let phrase = PhraseBuilder::new()
///     .with_text("Custom motivational phrase")
///     .with_created_by(admin_id)
///     .persist(pool).await?;
///
/// // Inactive phrase
/// let phrase = PhraseBuilder::new()
///     .with_created_by(admin_id)
///     .inactive()
///     .persist(pool).await?;
/// ```
#[derive(Clone)]
pub struct PhraseBuilder {
    id: Option<Uuid>,
    phrase_text: Option<String>,
    active: Option<bool>,
    created_by: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl PhraseBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            id: None,
            phrase_text: None,
            active: None,
            created_by: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Build the Phrase with defaults for any unset fields (in-memory only, no database)
    pub fn build(self) -> Phrase {
        let now = Utc::now();
        let id = self.id.unwrap_or_else(Uuid::new_v4);

        Phrase {
            id,
            phrase_text: self
                .phrase_text
                .unwrap_or_else(|| format!("Test phrase {}", id)),
            active: self.active.unwrap_or(true),
            created_by: self.created_by.unwrap_or_else(Uuid::new_v4),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        }
    }

    /// Persist Phrase to database (for integration tests)
    pub async fn persist(self, pool: &PgPool) -> Result<Phrase> {
        // Generate defaults
        let phrase_text = self
            .phrase_text
            .unwrap_or_else(|| format!("Test phrase {}", Uuid::new_v4()));
        let active = self.active.unwrap_or(true);
        let created_by = self
            .created_by
            .expect("created_by is required for persist()");

        let phrase = sqlx::query_as::<_, Phrase>(
            "INSERT INTO phrases (phrase_text, active, created_by)
             VALUES ($1, $2, $3)
             RETURNING *",
        )
        .bind(phrase_text)
        .bind(active)
        .bind(created_by)
        .fetch_one(pool)
        .await?;

        Ok(phrase)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set a specific phrase ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the phrase text
    pub fn with_text(mut self, phrase_text: impl Into<String>) -> Self {
        self.phrase_text = Some(phrase_text.into());
        self
    }

    /// Set the active status
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Mark phrase as inactive
    pub fn inactive(self) -> Self {
        self.active(false)
    }

    /// Set the creator user ID (required for persist())
    pub fn with_created_by(mut self, created_by: Uuid) -> Self {
        self.created_by = Some(created_by);
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

impl Default for PhraseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating PhraseSuggestion instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust,ignore
/// // Minimal suggestion with defaults
/// let suggestion = PhraseSuggestionBuilder::new()
///     .with_user_id(user_id)
///     .persist(pool).await?;
///
/// // Approved suggestion
/// let suggestion = PhraseSuggestionBuilder::new()
///     .with_user_id(user_id)
///     .with_text("Suggested phrase")
///     .approved(admin_id, "Good suggestion")
///     .persist(pool).await?;
///
/// // Rejected suggestion
/// let suggestion = PhraseSuggestionBuilder::new()
///     .with_user_id(user_id)
///     .rejected(admin_id, "Duplicate")
///     .persist(pool).await?;
/// ```
#[derive(Clone)]
pub struct PhraseSuggestionBuilder {
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    phrase_text: Option<String>,
    status: Option<String>,
    admin_id: Option<Option<Uuid>>, // Option<Option<...>> to distinguish between "not set" and "explicitly None"
    admin_reason: Option<Option<String>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl PhraseSuggestionBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            id: None,
            user_id: None,
            phrase_text: None,
            status: None,
            admin_id: None,
            admin_reason: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Build the PhraseSuggestion with defaults for any unset fields (in-memory only, no database)
    pub fn build(self) -> PhraseSuggestion {
        let now = Utc::now();
        let id = self.id.unwrap_or_else(Uuid::new_v4);

        PhraseSuggestion {
            id,
            user_id: self.user_id.unwrap_or_else(Uuid::new_v4),
            phrase_text: self
                .phrase_text
                .unwrap_or_else(|| format!("Test suggestion {}", id)),
            status: self.status.unwrap_or_else(|| "pending".to_string()),
            admin_id: self.admin_id.unwrap_or(None),
            admin_reason: self.admin_reason.unwrap_or(None),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        }
    }

    /// Persist PhraseSuggestion to database (for integration tests)
    pub async fn persist(self, pool: &PgPool) -> Result<PhraseSuggestion> {
        // Generate defaults
        let user_id = self.user_id.expect("user_id is required for persist()");
        let phrase_text = self
            .phrase_text
            .unwrap_or_else(|| format!("Test suggestion {}", Uuid::new_v4()));
        let status = self.status.unwrap_or_else(|| "pending".to_string());
        let admin_id = self.admin_id.unwrap_or(None);
        let admin_reason = self.admin_reason.unwrap_or(None);

        let suggestion = sqlx::query_as::<_, PhraseSuggestion>(
            "INSERT INTO phrase_suggestions (user_id, phrase_text, status, admin_id, admin_reason)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(user_id)
        .bind(phrase_text)
        .bind(status)
        .bind(admin_id)
        .bind(admin_reason)
        .fetch_one(pool)
        .await?;

        Ok(suggestion)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set a specific suggestion ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the user ID (required for persist())
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the phrase text
    pub fn with_text(mut self, phrase_text: impl Into<String>) -> Self {
        self.phrase_text = Some(phrase_text.into());
        self
    }

    /// Set the status (pending/approved/rejected)
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Mark suggestion as pending
    pub fn pending(self) -> Self {
        self.with_status("pending")
    }

    /// Mark suggestion as approved with admin details
    pub fn approved(mut self, admin_id: Uuid, reason: impl Into<String>) -> Self {
        self.status = Some("approved".to_string());
        self.admin_id = Some(Some(admin_id));
        self.admin_reason = Some(Some(reason.into()));
        self
    }

    /// Mark suggestion as rejected with admin details
    pub fn rejected(mut self, admin_id: Uuid, reason: impl Into<String>) -> Self {
        self.status = Some("rejected".to_string());
        self.admin_id = Some(Some(admin_id));
        self.admin_reason = Some(Some(reason.into()));
        self
    }

    /// Set admin ID
    pub fn with_admin_id(mut self, admin_id: Uuid) -> Self {
        self.admin_id = Some(Some(admin_id));
        self
    }

    /// Set admin reason
    pub fn with_admin_reason(mut self, admin_reason: impl Into<String>) -> Self {
        self.admin_reason = Some(Some(admin_reason.into()));
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

impl Default for PhraseSuggestionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_builder_creates_valid_phrase_with_defaults() {
        let phrase = PhraseBuilder::new().build();

        assert!(!phrase.id.is_nil());
        assert!(!phrase.phrase_text.is_empty());
        assert!(phrase.active);
        assert!(!phrase.created_by.is_nil());
    }

    #[test]
    fn test_phrase_builder_with_custom_text() {
        let phrase = PhraseBuilder::new().with_text("Custom phrase").build();

        assert_eq!(phrase.phrase_text, "Custom phrase");
    }

    #[test]
    fn test_phrase_builder_inactive() {
        let phrase = PhraseBuilder::new().inactive().build();

        assert!(!phrase.active);
    }

    #[test]
    fn test_phrase_builder_with_creator() {
        let creator_id = Uuid::new_v4();
        let phrase = PhraseBuilder::new().with_created_by(creator_id).build();

        assert_eq!(phrase.created_by, creator_id);
    }

    #[test]
    fn test_phrase_suggestion_builder_creates_valid_suggestion_with_defaults() {
        let suggestion = PhraseSuggestionBuilder::new().build();

        assert!(!suggestion.id.is_nil());
        assert!(!suggestion.user_id.is_nil());
        assert!(!suggestion.phrase_text.is_empty());
        assert_eq!(suggestion.status, "pending");
        assert!(suggestion.admin_id.is_none());
        assert!(suggestion.admin_reason.is_none());
    }

    #[test]
    fn test_phrase_suggestion_builder_approved() {
        let admin_id = Uuid::new_v4();
        let suggestion = PhraseSuggestionBuilder::new()
            .with_text("Great suggestion")
            .approved(admin_id, "Excellent idea")
            .build();

        assert_eq!(suggestion.phrase_text, "Great suggestion");
        assert_eq!(suggestion.status, "approved");
        assert_eq!(suggestion.admin_id, Some(admin_id));
        assert_eq!(suggestion.admin_reason, Some("Excellent idea".to_string()));
    }

    #[test]
    fn test_phrase_suggestion_builder_rejected() {
        let admin_id = Uuid::new_v4();
        let suggestion = PhraseSuggestionBuilder::new()
            .rejected(admin_id, "Duplicate")
            .build();

        assert_eq!(suggestion.status, "rejected");
        assert_eq!(suggestion.admin_id, Some(admin_id));
        assert_eq!(suggestion.admin_reason, Some("Duplicate".to_string()));
    }

    #[test]
    fn test_phrase_suggestion_builder_pending() {
        let suggestion = PhraseSuggestionBuilder::new().pending().build();

        assert_eq!(suggestion.status, "pending");
    }

    #[test]
    fn test_phrase_suggestion_builder_chaining() {
        let user_id = Uuid::new_v4();
        let suggestion = PhraseSuggestionBuilder::new()
            .with_user_id(user_id)
            .with_text("Chained suggestion")
            .pending()
            .build();

        assert_eq!(suggestion.user_id, user_id);
        assert_eq!(suggestion.phrase_text, "Chained suggestion");
        assert_eq!(suggestion.status, "pending");
    }
}
