use crate::events::DomainEvent;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::any::Any;
use uuid::Uuid;

/// Event emitted when a user changes their password
///
/// This event triggers a security notification email to the user.
#[derive(Clone, Debug, Serialize)]
pub struct PasswordChangedEvent {
    /// ID of the user who changed their password
    pub user_id: Uuid,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl PasswordChangedEvent {
    /// Create a new PasswordChangedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user who changed their password
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
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

impl DomainEvent for PasswordChangedEvent {
    fn event_type(&self) -> &'static str {
        "security.password_changed"
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

/// Event emitted when a user updates their profile
///
/// This event triggers a security notification email to the user.
#[derive(Clone, Debug, Serialize)]
pub struct ProfileUpdatedEvent {
    /// ID of the user who updated their profile
    pub user_id: Uuid,

    /// Previous display name (for email notification showing what changed)
    pub old_display_name: String,

    /// New display name (for email notification)
    pub new_display_name: String,

    /// Previous slug (for email notification showing what changed)
    pub old_slug: String,

    /// New slug (for email notification)
    pub new_slug: String,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl ProfileUpdatedEvent {
    /// Create a new ProfileUpdatedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user who updated their profile
    /// * `old_display_name` - Previous display name
    /// * `new_display_name` - New display name
    /// * `old_slug` - Previous slug
    /// * `new_slug` - New slug
    pub fn new(
        user_id: Uuid,
        old_display_name: impl Into<String>,
        new_display_name: impl Into<String>,
        old_slug: impl Into<String>,
        new_slug: impl Into<String>,
    ) -> Self {
        Self {
            user_id,
            old_display_name: old_display_name.into(),
            new_display_name: new_display_name.into(),
            old_slug: old_slug.into(),
            new_slug: new_slug.into(),
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

impl DomainEvent for ProfileUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "security.profile_updated"
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
    fn test_password_changed_event() {
        let user_id = Uuid::new_v4();
        let event = PasswordChangedEvent::new(user_id);

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.event_type(), "security.password_changed");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_password_changed_with_correlation_id() {
        let event = PasswordChangedEvent::new(Uuid::new_v4())
            .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_password_changed_event_is_cloneable() {
        let event = PasswordChangedEvent::new(Uuid::new_v4());
        let cloned = event.clone();
        assert_eq!(event.user_id, cloned.user_id);
    }

    #[test]
    fn test_password_changed_event_is_serializable() {
        let event = PasswordChangedEvent::new(Uuid::new_v4());
        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("user_id"));
    }

    #[test]
    fn test_profile_updated_event() {
        let user_id = Uuid::new_v4();
        let event = ProfileUpdatedEvent::new(
            user_id,
            "Old Name",
            "New Name",
            "old-slug",
            "new-slug",
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.old_display_name, "Old Name");
        assert_eq!(event.new_display_name, "New Name");
        assert_eq!(event.old_slug, "old-slug");
        assert_eq!(event.new_slug, "new-slug");
        assert_eq!(event.event_type(), "security.profile_updated");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_profile_updated_with_correlation_id() {
        let event = ProfileUpdatedEvent::new(
            Uuid::new_v4(),
            "Old Name",
            "New Name",
            "old-slug",
            "new-slug",
        )
        .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_profile_updated_event_is_cloneable() {
        let event = ProfileUpdatedEvent::new(
            Uuid::new_v4(),
            "Old",
            "New",
            "old",
            "new",
        );

        let cloned = event.clone();
        assert_eq!(event.user_id, cloned.user_id);
        assert_eq!(event.new_display_name, cloned.new_display_name);
        assert_eq!(event.old_display_name, cloned.old_display_name);
    }

    #[test]
    fn test_profile_updated_event_is_serializable() {
        let event = ProfileUpdatedEvent::new(
            Uuid::new_v4(),
            "Old",
            "New",
            "old",
            "new",
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("New"));
        assert!(json.contains("new"));
        assert!(json.contains("Old"));
        assert!(json.contains("old"));
    }

    #[test]
    fn test_profile_updated_event_without_old_values() {
        // Even if display name and slug don't change, we still capture them
        let event = ProfileUpdatedEvent::new(
            Uuid::new_v4(),
            "Same Name",
            "Same Name",
            "same-slug",
            "same-slug",
        );

        assert_eq!(event.old_display_name, "Same Name");
        assert_eq!(event.old_slug, "same-slug");
        assert_eq!(event.new_display_name, "Same Name");
        assert_eq!(event.new_slug, "same-slug");
    }
}
