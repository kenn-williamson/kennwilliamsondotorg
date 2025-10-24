use crate::events::DomainEvent;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::any::Any;
use uuid::Uuid;

/// Event emitted when a user submits a phrase suggestion
///
/// This event triggers admin notifications via email.
/// Event handlers fetch user details as needed.
#[derive(Clone, Debug, Serialize)]
pub struct PhraseSuggestionCreatedEvent {
    /// ID of the user who suggested the phrase
    pub user_id: Uuid,

    /// The suggested phrase text
    pub phrase_text: String,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl PhraseSuggestionCreatedEvent {
    /// Create a new PhraseSuggestionCreatedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user suggesting the phrase
    /// * `phrase_text` - The suggested phrase
    pub fn new(
        user_id: Uuid,
        phrase_text: impl Into<String>,
    ) -> Self {
        Self {
            user_id,
            phrase_text: phrase_text.into(),
            occurred_at: Utc::now(),
            correlation_id: None,
        }
    }

    /// Create a new event with correlation ID
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

impl DomainEvent for PhraseSuggestionCreatedEvent {
    fn event_type(&self) -> &'static str {
        "phrase_suggestion.created"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_boxed(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
}

/// Event emitted when an admin approves a phrase suggestion
///
/// This event triggers user notification via email.
/// Event handlers fetch user details as needed.
#[derive(Clone, Debug, Serialize)]
pub struct PhraseSuggestionApprovedEvent {
    /// ID of the user whose suggestion was approved
    pub user_id: Uuid,

    /// The phrase text that was approved
    pub phrase_text: String,

    /// Optional reason/message from the admin
    pub admin_reason: Option<String>,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl PhraseSuggestionApprovedEvent {
    /// Create a new PhraseSuggestionApprovedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user
    /// * `phrase_text` - The phrase text that was approved
    /// * `admin_reason` - Optional admin message
    pub fn new(
        user_id: Uuid,
        phrase_text: impl Into<String>,
        admin_reason: Option<String>,
    ) -> Self {
        Self {
            user_id,
            phrase_text: phrase_text.into(),
            admin_reason,
            occurred_at: Utc::now(),
            correlation_id: None,
        }
    }

    /// Create a new event with correlation ID
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

impl DomainEvent for PhraseSuggestionApprovedEvent {
    fn event_type(&self) -> &'static str {
        "phrase_suggestion.approved"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_boxed(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
}

/// Event emitted when an admin rejects a phrase suggestion
///
/// This event triggers user notification via email.
/// Event handlers fetch user details as needed.
#[derive(Clone, Debug, Serialize)]
pub struct PhraseSuggestionRejectedEvent {
    /// ID of the user whose suggestion was rejected
    pub user_id: Uuid,

    /// The phrase text that was rejected
    pub phrase_text: String,

    /// Optional reason/message from the admin
    pub admin_reason: Option<String>,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl PhraseSuggestionRejectedEvent {
    /// Create a new PhraseSuggestionRejectedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user
    /// * `phrase_text` - The phrase text that was rejected
    /// * `admin_reason` - Optional admin message
    pub fn new(
        user_id: Uuid,
        phrase_text: impl Into<String>,
        admin_reason: Option<String>,
    ) -> Self {
        Self {
            user_id,
            phrase_text: phrase_text.into(),
            admin_reason,
            occurred_at: Utc::now(),
            correlation_id: None,
        }
    }

    /// Create a new event with correlation ID
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

impl DomainEvent for PhraseSuggestionRejectedEvent {
    fn event_type(&self) -> &'static str {
        "phrase_suggestion.rejected"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_boxed(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_suggestion_created_event() {
        let user_id = Uuid::new_v4();
        let event = PhraseSuggestionCreatedEvent::new(
            user_id,
            "Time is an illusion",
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.phrase_text, "Time is an illusion");
        assert_eq!(event.event_type(), "phrase_suggestion.created");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_with_correlation_id() {
        let event = PhraseSuggestionCreatedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
        )
        .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_event_is_cloneable() {
        let event = PhraseSuggestionCreatedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
        );

        let cloned = event.clone();
        assert_eq!(event.phrase_text, cloned.phrase_text);
    }

    #[test]
    fn test_event_is_serializable() {
        let event = PhraseSuggestionCreatedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("Test phrase"));
    }

    #[test]
    fn test_phrase_suggestion_approved_event() {
        let user_id = Uuid::new_v4();
        let event = PhraseSuggestionApprovedEvent::new(
            user_id,
            "Time is an illusion",
            Some("Great suggestion!".to_string()),
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.phrase_text, "Time is an illusion");
        assert_eq!(event.admin_reason, Some("Great suggestion!".to_string()));
        assert_eq!(event.event_type(), "phrase_suggestion.approved");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_phrase_suggestion_rejected_event() {
        let user_id = Uuid::new_v4();
        let event = PhraseSuggestionRejectedEvent::new(
            user_id,
            "Not appropriate",
            Some("Too controversial".to_string()),
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.phrase_text, "Not appropriate");
        assert_eq!(event.admin_reason, Some("Too controversial".to_string()));
        assert_eq!(event.event_type(), "phrase_suggestion.rejected");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_approved_event_with_correlation_id() {
        let event = PhraseSuggestionApprovedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
            None,
        )
        .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_rejected_event_with_correlation_id() {
        let event = PhraseSuggestionRejectedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
            None,
        )
        .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_approved_event_is_cloneable() {
        let event = PhraseSuggestionApprovedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
            Some("Reason".to_string()),
        );

        let cloned = event.clone();
        assert_eq!(event.phrase_text, cloned.phrase_text);
        assert_eq!(event.admin_reason, cloned.admin_reason);
    }

    #[test]
    fn test_rejected_event_is_cloneable() {
        let event = PhraseSuggestionRejectedEvent::new(
            Uuid::new_v4(),
            "Test phrase",
            Some("Reason".to_string()),
        );

        let cloned = event.clone();
        assert_eq!(event.user_id, cloned.user_id);
        assert_eq!(event.admin_reason, cloned.admin_reason);
    }

    #[test]
    fn test_approved_event_is_serializable() {
        let event = PhraseSuggestionApprovedEvent::new(
            Uuid::new_v4(),
            "Great phrase",
            Some("Welcome".to_string()),
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("Great phrase"));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn test_rejected_event_is_serializable() {
        let event = PhraseSuggestionRejectedEvent::new(
            Uuid::new_v4(),
            "Bad phrase",
            Some("Not appropriate".to_string()),
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("Bad phrase"));
        assert!(json.contains("Not appropriate"));
    }
}
