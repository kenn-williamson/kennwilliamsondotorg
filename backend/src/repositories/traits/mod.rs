pub mod access_request_repository;
pub mod admin_repository;
pub mod email_suppression_repository;
pub mod incident_timer_repository;
pub mod password_reset_token_repository;
pub mod phrase_repository;
pub mod pkce_storage;
pub mod refresh_token_repository;
pub mod user_credentials_repository;
pub mod user_external_login_repository;
pub mod user_preferences_repository;
pub mod user_profile_repository;
pub mod user_repository;
pub mod verification_token_repository;

pub use access_request_repository::AccessRequestRepository;
pub use admin_repository::AdminRepository;
pub use incident_timer_repository::IncidentTimerRepository;
pub use password_reset_token_repository::PasswordResetTokenRepository;
pub use phrase_repository::PhraseRepository;
pub use pkce_storage::PkceStorage;
pub use refresh_token_repository::RefreshTokenRepository;
pub use user_repository::UserRepository;
pub use verification_token_repository::VerificationTokenRepository;

// Re-export new trait definitions for use in service layer
#[allow(unused_imports)]
pub use user_credentials_repository::UserCredentialsRepository;
#[allow(unused_imports)]
pub use user_external_login_repository::UserExternalLoginRepository;
#[allow(unused_imports)]
pub use user_preferences_repository::UserPreferencesRepository;
#[allow(unused_imports)]
pub use user_profile_repository::UserProfileRepository;
