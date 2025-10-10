use super::jwt::JwtService;
use crate::repositories::traits::incident_timer_repository::IncidentTimerRepository;
use crate::repositories::traits::password_reset_token_repository::PasswordResetTokenRepository;
use crate::repositories::traits::phrase_repository::PhraseRepository;
use crate::repositories::traits::pkce_storage::PkceStorage;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::traits::user_repository::UserRepository;
use crate::repositories::traits::verification_token_repository::VerificationTokenRepository;
use crate::services::auth::oauth::GoogleOAuthServiceTrait;
use crate::services::email::EmailService;
use anyhow::Result;

pub mod builder;
pub mod account_deletion;
pub mod data_export;
pub mod email_verification;
pub mod login;
pub mod oauth;
pub mod password;
pub mod password_reset;
pub mod profile;
pub mod refresh_token;
pub mod register;
pub mod slug;

pub use builder::AuthServiceBuilder;

pub struct AuthService {
    jwt_service: JwtService,
    user_repository: Box<dyn UserRepository>,
    refresh_token_repository: Box<dyn RefreshTokenRepository>,
    verification_token_repository: Option<Box<dyn VerificationTokenRepository>>,
    password_reset_token_repository: Option<Box<dyn PasswordResetTokenRepository>>,
    email_service: Option<Box<dyn EmailService>>,
    google_oauth_service: Option<Box<dyn GoogleOAuthServiceTrait>>,
    pkce_storage: Option<Box<dyn PkceStorage>>,
    incident_timer_repository: Option<Box<dyn IncidentTimerRepository>>,
    phrase_repository: Option<Box<dyn PhraseRepository>>,
}

impl AuthService {
    /// Create a new AuthService builder
    pub fn builder() -> AuthServiceBuilder {
        AuthServiceBuilder::new()
    }

    /// Legacy constructor for existing tests (will be phased out)
    /// Prefer using AuthService::builder() for new code
    #[allow(dead_code)] // Legacy API for existing tests
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        refresh_token_repository: Box<dyn RefreshTokenRepository>,
        jwt_secret: String,
    ) -> Self {
        Self::builder()
            .user_repository(user_repository)
            .refresh_token_repository(refresh_token_repository)
            .jwt_secret(jwt_secret)
            .build()
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<super::jwt::Claims>> {
        self.jwt_service.verify_token(token).await
    }
}
