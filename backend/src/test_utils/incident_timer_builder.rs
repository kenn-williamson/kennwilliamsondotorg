use crate::models::db::incident_timer::IncidentTimer;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Builder for creating IncidentTimer instances in tests with sensible defaults.
///
/// # Examples
///
/// ```rust,ignore
/// // Minimal timer with defaults
/// let timer = IncidentTimerBuilder::new()
///     .with_user_id(user_id)
///     .persist(pool).await?;
///
/// // Timer with custom reset timestamp
/// let timer = IncidentTimerBuilder::new()
///     .with_user_id(user_id)
///     .with_reset_timestamp(Utc::now() + Duration::days(7))
///     .persist(pool).await?;
///
/// // Timer with notes
/// let timer = IncidentTimerBuilder::new()
///     .with_user_id(user_id)
///     .with_notes("Test notes")
///     .persist(pool).await?;
/// ```
#[derive(Clone)]
pub struct IncidentTimerBuilder {
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    reset_timestamp: Option<DateTime<Utc>>,
    notes: Option<Option<String>>, // Option<Option<...>> to distinguish between "not set" and "explicitly None"
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl IncidentTimerBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            id: None,
            user_id: None,
            reset_timestamp: None,
            notes: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Build the IncidentTimer with defaults for any unset fields (in-memory only, no database)
    pub fn build(self) -> IncidentTimer {
        let now = Utc::now();

        IncidentTimer {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            user_id: self.user_id.unwrap_or_else(Uuid::new_v4),
            reset_timestamp: self.reset_timestamp.unwrap_or(now),
            notes: self.notes.unwrap_or(None),
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        }
    }

    /// Persist IncidentTimer to database (for integration tests)
    pub async fn persist(self, pool: &PgPool) -> Result<IncidentTimer> {
        // Generate defaults
        let user_id = self.user_id.expect("user_id is required for persist()");
        let reset_timestamp = self.reset_timestamp.unwrap_or_else(Utc::now);
        let notes = self.notes.unwrap_or(None);

        let timer = sqlx::query_as::<_, IncidentTimer>(
            "INSERT INTO incident_timers (user_id, reset_timestamp, notes)
             VALUES ($1, $2, $3)
             RETURNING *",
        )
        .bind(user_id)
        .bind(reset_timestamp)
        .bind(notes)
        .fetch_one(pool)
        .await?;

        Ok(timer)
    }

    // ============================================================================
    // CONFIGURATION METHODS
    // ============================================================================

    /// Set a specific timer ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the user ID (required for persist())
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the reset timestamp
    pub fn with_reset_timestamp(mut self, reset_timestamp: DateTime<Utc>) -> Self {
        self.reset_timestamp = Some(reset_timestamp);
        self
    }

    /// Set the reset timestamp to a relative time from now
    pub fn reset_in_days(mut self, days: i64) -> Self {
        self.reset_timestamp = Some(Utc::now() + Duration::days(days));
        self
    }

    /// Set the reset timestamp to a relative time from now (hours)
    pub fn reset_in_hours(mut self, hours: i64) -> Self {
        self.reset_timestamp = Some(Utc::now() + Duration::hours(hours));
        self
    }

    /// Set the notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(Some(notes.into()));
        self
    }

    /// Set notes to None explicitly
    pub fn without_notes(mut self) -> Self {
        self.notes = Some(None);
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

impl Default for IncidentTimerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_valid_timer_with_defaults() {
        let timer = IncidentTimerBuilder::new().build();

        assert!(!timer.id.is_nil());
        assert!(!timer.user_id.is_nil());
        assert!(timer.notes.is_none());
    }

    #[test]
    fn test_builder_with_user_id() {
        let user_id = Uuid::new_v4();
        let timer = IncidentTimerBuilder::new().with_user_id(user_id).build();

        assert_eq!(timer.user_id, user_id);
    }

    #[test]
    fn test_builder_with_notes() {
        let timer = IncidentTimerBuilder::new().with_notes("Test notes").build();

        assert_eq!(timer.notes, Some("Test notes".to_string()));
    }

    #[test]
    fn test_builder_without_notes() {
        let timer = IncidentTimerBuilder::new().without_notes().build();

        assert!(timer.notes.is_none());
    }

    #[test]
    fn test_builder_reset_in_days() {
        let now = Utc::now();
        let timer = IncidentTimerBuilder::new().reset_in_days(7).build();

        // Should be approximately 7 days from now (allow 1 second tolerance)
        let expected = now + Duration::days(7);
        let diff = (timer.reset_timestamp - expected).num_seconds().abs();
        assert!(diff <= 1, "Timestamp difference: {} seconds", diff);
    }

    #[test]
    fn test_builder_reset_in_hours() {
        let now = Utc::now();
        let timer = IncidentTimerBuilder::new().reset_in_hours(24).build();

        // Should be approximately 24 hours from now (allow 1 second tolerance)
        let expected = now + Duration::hours(24);
        let diff = (timer.reset_timestamp - expected).num_seconds().abs();
        assert!(diff <= 1, "Timestamp difference: {} seconds", diff);
    }

    #[test]
    fn test_builder_chaining() {
        let user_id = Uuid::new_v4();
        let reset_time = Utc::now() + Duration::days(30);
        let timer = IncidentTimerBuilder::new()
            .with_user_id(user_id)
            .with_reset_timestamp(reset_time)
            .with_notes("Chained notes")
            .build();

        assert_eq!(timer.user_id, user_id);
        assert_eq!(timer.reset_timestamp, reset_time);
        assert_eq!(timer.notes, Some("Chained notes".to_string()));
    }
}
