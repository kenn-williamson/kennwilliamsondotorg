pub mod email_notification_handler;

// Re-export handlers
pub use email_notification_handler::{
    AccessRequestApprovedEmailHandler, AccessRequestEmailNotificationHandler,
    AccessRequestRejectedEmailHandler, PasswordChangedEmailHandler,
    PhraseSuggestionApprovedEmailHandler, PhraseSuggestionEmailNotificationHandler,
    PhraseSuggestionRejectedEmailHandler, ProfileUpdatedEmailHandler, UserRegisteredEmailHandler,
};
