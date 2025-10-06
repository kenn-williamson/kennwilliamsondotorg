use super::AuthService;
use crate::repositories::traits::incident_timer_repository::IncidentTimerRepository;
use crate::repositories::traits::phrase_repository::PhraseRepository;
use crate::repositories::traits::pkce_storage::PkceStorage;
use crate::repositories::traits::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::traits::user_repository::UserRepository;
use crate::repositories::traits::verification_token_repository::VerificationTokenRepository;
use crate::services::auth::jwt::JwtService;
use crate::services::auth::oauth::GoogleOAuthServiceTrait;
use crate::services::email::EmailService;

/// Builder for AuthService to handle optional dependencies
pub struct AuthServiceBuilder {
    user_repository: Option<Box<dyn UserRepository>>,
    refresh_token_repository: Option<Box<dyn RefreshTokenRepository>>,
    verification_token_repository: Option<Box<dyn VerificationTokenRepository>>,
    email_service: Option<Box<dyn EmailService>>,
    google_oauth_service: Option<Box<dyn GoogleOAuthServiceTrait>>,
    pkce_storage: Option<Box<dyn PkceStorage>>,
    incident_timer_repository: Option<Box<dyn IncidentTimerRepository>>,
    phrase_repository: Option<Box<dyn PhraseRepository>>,
    jwt_secret: Option<String>,
}

impl AuthServiceBuilder {
    pub fn new() -> Self {
        Self {
            user_repository: None,
            refresh_token_repository: None,
            verification_token_repository: None,
            email_service: None,
            google_oauth_service: None,
            pkce_storage: None,
            incident_timer_repository: None,
            phrase_repository: None,
            jwt_secret: None,
        }
    }

    pub fn user_repository(mut self, repo: Box<dyn UserRepository>) -> Self {
        self.user_repository = Some(repo);
        self
    }

    pub fn refresh_token_repository(mut self, repo: Box<dyn RefreshTokenRepository>) -> Self {
        self.refresh_token_repository = Some(repo);
        self
    }

    pub fn verification_token_repository(
        mut self,
        repo: Box<dyn VerificationTokenRepository>,
    ) -> Self {
        self.verification_token_repository = Some(repo);
        self
    }

    pub fn email_service(mut self, service: Box<dyn EmailService>) -> Self {
        self.email_service = Some(service);
        self
    }

    pub fn google_oauth_service(mut self, service: Box<dyn GoogleOAuthServiceTrait>) -> Self {
        self.google_oauth_service = Some(service);
        self
    }

    pub fn pkce_storage(mut self, storage: Box<dyn PkceStorage>) -> Self {
        self.pkce_storage = Some(storage);
        self
    }

    pub fn jwt_secret(mut self, secret: String) -> Self {
        self.jwt_secret = Some(secret);
        self
    }

    pub fn incident_timer_repository(mut self, repo: Box<dyn IncidentTimerRepository>) -> Self {
        self.incident_timer_repository = Some(repo);
        self
    }

    pub fn phrase_repository(mut self, repo: Box<dyn PhraseRepository>) -> Self {
        self.phrase_repository = Some(repo);
        self
    }

    pub fn build(self) -> AuthService {
        let jwt_secret = self.jwt_secret.expect("jwt_secret is required");
        let user_repository = self.user_repository.expect("user_repository is required");
        let refresh_token_repository = self
            .refresh_token_repository
            .expect("refresh_token_repository is required");

        AuthService {
            jwt_service: JwtService::new(jwt_secret),
            user_repository,
            refresh_token_repository,
            verification_token_repository: self.verification_token_repository,
            email_service: self.email_service,
            google_oauth_service: self.google_oauth_service,
            pkce_storage: self.pkce_storage,
            incident_timer_repository: self.incident_timer_repository,
            phrase_repository: self.phrase_repository,
        }
    }
}

impl Default for AuthServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{
        MockRefreshTokenRepository, MockUserRepository, MockVerificationTokenRepository,
    };
    use crate::services::email::MockEmailService;

    #[test]
    fn test_builder_with_required_dependencies() {
        let service = AuthServiceBuilder::new()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .jwt_secret("test-secret".to_string())
            .build();

        // Service should be created without email verification features
        assert!(service.verification_token_repository.is_none());
        assert!(service.email_service.is_none());
    }

    #[test]
    fn test_builder_with_all_dependencies() {
        let service = AuthServiceBuilder::new()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
            .email_service(Box::new(MockEmailService::new()))
            .jwt_secret("test-secret".to_string())
            .build();

        // Service should have all features
        assert!(service.verification_token_repository.is_some());
        assert!(service.email_service.is_some());
    }

    #[test]
    #[should_panic(expected = "jwt_secret is required")]
    fn test_builder_panics_without_jwt_secret() {
        AuthServiceBuilder::new()
            .user_repository(Box::new(MockUserRepository::new()))
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .build();
    }

    #[test]
    #[should_panic(expected = "user_repository is required")]
    fn test_builder_panics_without_user_repository() {
        AuthServiceBuilder::new()
            .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
            .jwt_secret("test-secret".to_string())
            .build();
    }
}
