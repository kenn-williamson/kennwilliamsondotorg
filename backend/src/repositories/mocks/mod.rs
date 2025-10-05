#![cfg(feature = "mocks")]

pub mod mock_admin_repository;
pub mod mock_email_suppression_repository;
pub mod mock_incident_timer_repository;
pub mod mock_phrase_repository;
pub mod mock_pkce_storage;
pub mod mock_refresh_token_repository;
pub mod mock_user_repository;
pub mod mock_verification_token_repository;

pub use mock_admin_repository::MockAdminRepository;
#[allow(unused_imports)]
pub use mock_email_suppression_repository::MockEmailSuppressionRepository;
pub use mock_incident_timer_repository::MockIncidentTimerRepository;
pub use mock_phrase_repository::MockPhraseRepository;
pub use mock_pkce_storage::MockPkceStorage;
pub use mock_refresh_token_repository::MockRefreshTokenRepository;
pub use mock_user_repository::MockUserRepository;
pub use mock_verification_token_repository::MockVerificationTokenRepository;
