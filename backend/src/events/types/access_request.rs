use crate::events::DomainEvent;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::any::Any;
use uuid::Uuid;

/// Event emitted when a user creates an access request
///
/// This event triggers admin notifications via email.
#[derive(Clone, Debug, Serialize)]
pub struct AccessRequestCreatedEvent {
    /// ID of the user requesting access
    pub user_id: Uuid,

    /// Email of the user (for logging/debugging)
    pub user_email: String,

    /// Display name of the user (for email personalization)
    pub user_display_name: String,

    /// User's message explaining why they need access
    pub message: String,

    /// Role being requested (e.g., "trusted-contact")
    pub requested_role: String,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl AccessRequestCreatedEvent {
    /// Create a new AccessRequestCreatedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user requesting access
    /// * `user_email` - Email of the user
    /// * `user_display_name` - Display name for personalization
    /// * `message` - User's request message
    /// * `requested_role` - Role being requested
    pub fn new(
        user_id: Uuid,
        user_email: impl Into<String>,
        user_display_name: impl Into<String>,
        message: impl Into<String>,
        requested_role: impl Into<String>,
    ) -> Self {
        Self {
            user_id,
            user_email: user_email.into(),
            user_display_name: user_display_name.into(),
            message: message.into(),
            requested_role: requested_role.into(),
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

impl DomainEvent for AccessRequestCreatedEvent {
    fn event_type(&self) -> &'static str {
        "access_request.created"
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

/// Event emitted when an admin approves an access request
///
/// This event triggers user notification via email.
/// Event handlers fetch user details as needed.
#[derive(Clone, Debug, Serialize)]
pub struct AccessRequestApprovedEvent {
    /// ID of the user whose request was approved
    pub user_id: Uuid,

    /// Role that was granted
    pub granted_role: String,

    /// Optional reason/message from the admin
    pub admin_reason: Option<String>,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl AccessRequestApprovedEvent {
    /// Create a new AccessRequestApprovedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user
    /// * `granted_role` - Role that was granted
    /// * `admin_reason` - Optional admin message
    pub fn new(
        user_id: Uuid,
        granted_role: impl Into<String>,
        admin_reason: Option<String>,
    ) -> Self {
        Self {
            user_id,
            granted_role: granted_role.into(),
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

impl DomainEvent for AccessRequestApprovedEvent {
    fn event_type(&self) -> &'static str {
        "access_request.approved"
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

/// Event emitted when an admin rejects an access request
///
/// This event triggers user notification via email.
/// Event handlers fetch user details as needed.
#[derive(Clone, Debug, Serialize)]
pub struct AccessRequestRejectedEvent {
    /// ID of the user whose request was rejected
    pub user_id: Uuid,

    /// Optional reason/message from the admin
    pub admin_reason: Option<String>,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl AccessRequestRejectedEvent {
    /// Create a new AccessRequestRejectedEvent
    ///
    /// # Arguments
    /// * `user_id` - ID of the user
    /// * `admin_reason` - Optional admin message
    pub fn new(user_id: Uuid, admin_reason: Option<String>) -> Self {
        Self {
            user_id,
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

impl DomainEvent for AccessRequestRejectedEvent {
    fn event_type(&self) -> &'static str {
        "access_request.rejected"
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
    fn test_access_request_created_event() {
        let user_id = Uuid::new_v4();
        let event = AccessRequestCreatedEvent::new(
            user_id,
            "test@example.com",
            "Test User",
            "I need access please",
            "trusted-contact",
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.user_email, "test@example.com");
        assert_eq!(event.user_display_name, "Test User");
        assert_eq!(event.message, "I need access please");
        assert_eq!(event.requested_role, "trusted-contact");
        assert_eq!(event.event_type(), "access_request.created");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_with_correlation_id() {
        let event = AccessRequestCreatedEvent::new(
            Uuid::new_v4(),
            "test@example.com",
            "Test User",
            "Message",
            "role",
        )
        .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_event_is_cloneable() {
        let event = AccessRequestCreatedEvent::new(
            Uuid::new_v4(),
            "test@example.com",
            "Test User",
            "Message",
            "role",
        );

        let cloned = event.clone();
        assert_eq!(event.user_email, cloned.user_email);
    }

    #[test]
    fn test_event_is_serializable() {
        let event = AccessRequestCreatedEvent::new(
            Uuid::new_v4(),
            "test@example.com",
            "Test User",
            "Message",
            "role",
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_access_request_approved_event() {
        let user_id = Uuid::new_v4();
        let event = AccessRequestApprovedEvent::new(
            user_id,
            "trusted-contact",
            Some("Welcome to the team!".to_string()),
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(event.granted_role, "trusted-contact");
        assert_eq!(event.admin_reason, Some("Welcome to the team!".to_string()));
        assert_eq!(event.event_type(), "access_request.approved");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_access_request_rejected_event() {
        let user_id = Uuid::new_v4();
        let event = AccessRequestRejectedEvent::new(
            user_id,
            Some("Insufficient justification".to_string()),
        );

        assert_eq!(event.user_id, user_id);
        assert_eq!(
            event.admin_reason,
            Some("Insufficient justification".to_string())
        );
        assert_eq!(event.event_type(), "access_request.rejected");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_approved_event_with_correlation_id() {
        let event = AccessRequestApprovedEvent::new(Uuid::new_v4(), "trusted-contact", None)
            .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_rejected_event_with_correlation_id() {
        let event = AccessRequestRejectedEvent::new(Uuid::new_v4(), None)
            .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_approved_event_is_cloneable() {
        let event =
            AccessRequestApprovedEvent::new(Uuid::new_v4(), "role", Some("Reason".to_string()));

        let cloned = event.clone();
        assert_eq!(event.granted_role, cloned.granted_role);
        assert_eq!(event.admin_reason, cloned.admin_reason);
    }

    #[test]
    fn test_rejected_event_is_cloneable() {
        let event = AccessRequestRejectedEvent::new(Uuid::new_v4(), Some("Reason".to_string()));

        let cloned = event.clone();
        assert_eq!(event.user_id, cloned.user_id);
        assert_eq!(event.admin_reason, cloned.admin_reason);
    }

    #[test]
    fn test_approved_event_is_serializable() {
        let event = AccessRequestApprovedEvent::new(
            Uuid::new_v4(),
            "trusted-contact",
            Some("Welcome".to_string()),
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("trusted-contact"));
        assert!(json.contains("Welcome"));
    }

    #[test]
    fn test_rejected_event_is_serializable() {
        let event =
            AccessRequestRejectedEvent::new(Uuid::new_v4(), Some("Not qualified".to_string()));

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("Not qualified"));
    }
}
