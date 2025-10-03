use super::jwt::JwtService;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::traits::user_repository::UserRepository;
use crate::repositories::traits::verification_token_repository::VerificationTokenRepository;
use crate::services::email::EmailService;
use anyhow::Result;

pub mod builder;
pub mod email_verification;
pub mod login;
pub mod password;
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
    email_service: Option<Box<dyn EmailService>>,
}

impl AuthService {
    /// Create a new AuthService builder
    pub fn builder() -> AuthServiceBuilder {
        AuthServiceBuilder::new()
    }

    /// Legacy constructor for existing tests (will be phased out)
    /// Prefer using AuthService::builder() for new code
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
