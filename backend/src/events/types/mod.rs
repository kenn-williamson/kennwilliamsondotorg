pub mod access_request;
pub mod phrase_suggestion;
pub mod security_notification;

// Re-export event types
pub use access_request::{
    AccessRequestApprovedEvent, AccessRequestCreatedEvent, AccessRequestRejectedEvent,
};
pub use phrase_suggestion::{
    PhraseSuggestionApprovedEvent, PhraseSuggestionCreatedEvent, PhraseSuggestionRejectedEvent,
};
pub use security_notification::{PasswordChangedEvent, ProfileUpdatedEvent, UserRegisteredEvent};
