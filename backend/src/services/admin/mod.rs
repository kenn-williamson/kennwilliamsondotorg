pub mod user_management;
pub mod phrase_moderation;
pub mod stats;

// Re-export main services but not sub-modules
pub use user_management::UserManagementService;
pub use phrase_moderation::PhraseModerationService;
pub use stats::StatsService;
