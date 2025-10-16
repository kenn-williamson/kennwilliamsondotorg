use sqlx::PgPool;
use std::sync::Arc;

#[cfg(feature = "mocks")]
use crate::repositories::mocks::{
    MockAccessRequestRepository, MockAdminRepository, MockIncidentTimerRepository,
    MockPasswordResetTokenRepository, MockPhraseRepository, MockPkceStorage,
    MockRefreshTokenRepository, MockUserRepository, MockVerificationTokenRepository,
};
use crate::repositories::postgres::{
    postgres_access_request_repository::PostgresAccessRequestRepository,
    postgres_admin_repository::PostgresAdminRepository,
    postgres_email_suppression_repository::PostgresEmailSuppressionRepository,
    postgres_incident_timer_repository::PostgresIncidentTimerRepository,
    postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository,
    postgres_phrase_repository::PostgresPhraseRepository,
    postgres_refresh_token_repository::PostgresRefreshTokenRepository,
    postgres_user_credentials_repository::PostgresUserCredentialsRepository,
    postgres_user_external_login_repository::PostgresUserExternalLoginRepository,
    postgres_user_preferences_repository::PostgresUserPreferencesRepository,
    postgres_user_profile_repository::PostgresUserProfileRepository,
    postgres_user_repository::PostgresUserRepository,
    postgres_verification_token_repository::PostgresVerificationTokenRepository,
};
use crate::repositories::redis::RedisPkceStorage;

use super::admin::{
    AccessRequestModerationService, PhraseModerationService, StatsService, UserManagementService,
};
use super::auth::AuthService;
use super::cleanup::CleanupService;
#[cfg(feature = "mocks")]
use super::email::MockEmailService;
use super::email::SesEmailService;
use super::incident_timer::IncidentTimerService;
use super::phrase::PhraseService;
#[cfg(feature = "mocks")]
use crate::middleware::rate_limiter::MockRateLimitService;
use crate::middleware::rate_limiter::{RateLimitServiceTrait, RedisRateLimitService};

/// Centralized service container for dependency injection
pub struct ServiceContainer {
    // Core services
    pub auth_service: Arc<AuthService>,
    pub incident_timer_service: Arc<IncidentTimerService>,
    pub phrase_service: Arc<PhraseService>,
    pub admin_service: Arc<UserManagementService>,
    pub phrase_moderation_service: Arc<PhraseModerationService>,
    pub access_request_moderation_service: Arc<AccessRequestModerationService>,
    pub stats_service: Arc<StatsService>,
    pub rate_limit_service: Arc<dyn RateLimitServiceTrait>,
    pub cleanup_service: Arc<CleanupService>,
}

impl ServiceContainer {
    /// Create service container for development/production with PostgreSQL
    pub fn new(pool: PgPool, jwt_secret: String, redis_url: String) -> Self {
        // Load email service configuration from environment
        let from_email = std::env::var("SES_FROM_EMAIL")
            .unwrap_or_else(|_| "noreply@kennwilliamson.org".to_string());
        let reply_to_email = std::env::var("SES_REPLY_TO_EMAIL").ok();

        // Create email service with suppression checking
        let suppression_repo = Box::new(PostgresEmailSuppressionRepository::new(pool.clone()));
        let email_service = SesEmailService::with_suppression(
            from_email,
            reply_to_email,
            suppression_repo,
        );

        // Create Google OAuth service (optional - only if env vars present)
        let google_oauth_service = super::auth::oauth::GoogleOAuthService::from_env().ok();

        // Create PKCE storage for OAuth flows
        let pkce_storage = RedisPkceStorage::new(&redis_url)
            .expect("Failed to create PKCE storage");

        // Create services with repository dependencies using builder pattern
        let mut auth_builder = AuthService::builder()
            .user_repository(Box::new(PostgresUserRepository::new(pool.clone())))
            .credentials_repository(Box::new(PostgresUserCredentialsRepository::new(pool.clone())))
            .external_login_repository(Box::new(PostgresUserExternalLoginRepository::new(pool.clone())))
            .profile_repository(Box::new(PostgresUserProfileRepository::new(pool.clone())))
            .preferences_repository(Box::new(PostgresUserPreferencesRepository::new(pool.clone())))
            .refresh_token_repository(Box::new(PostgresRefreshTokenRepository::new(
                pool.clone(),
            )))
            .verification_token_repository(Box::new(
                PostgresVerificationTokenRepository::new(pool.clone()),
            ))
            .password_reset_token_repository(Box::new(
                PostgresPasswordResetTokenRepository::new(pool.clone()),
            ))
            .incident_timer_repository(Box::new(PostgresIncidentTimerRepository::new(
                pool.clone(),
            )))
            .phrase_repository(Box::new(PostgresPhraseRepository::new(pool.clone())))
            .email_service(Box::new(email_service))
            .pkce_storage(Box::new(pkce_storage))
            .jwt_secret(jwt_secret.clone());

        // Add OAuth service if configured
        if let Some(oauth_svc) = google_oauth_service {
            auth_builder = auth_builder.google_oauth_service(Box::new(oauth_svc));
        }

        let auth_service = Arc::new(auth_builder.build());

        let incident_timer_service = Arc::new(IncidentTimerService::new(Box::new(
            PostgresIncidentTimerRepository::new(pool.clone()),
        )));

        let phrase_service = Arc::new(PhraseService::new(Box::new(PostgresPhraseRepository::new(
            pool.clone(),
        ))));

        let admin_service = Arc::new(UserManagementService::new(
            Box::new(PostgresUserRepository::new(pool.clone())),
            Box::new(PostgresRefreshTokenRepository::new(pool.clone())),
            Box::new(PostgresAdminRepository::new(pool.clone())),
        ));

        let phrase_moderation_service = Arc::new(PhraseModerationService::new(Box::new(
            PostgresPhraseRepository::new(pool.clone()),
        )));

        let access_request_moderation_service = Arc::new(AccessRequestModerationService::new(
            Box::new(PostgresAccessRequestRepository::new(pool.clone())),
        ));

        let stats_service = Arc::new(StatsService::new(
            Box::new(PostgresPhraseRepository::new(pool.clone())),
            Box::new(PostgresAdminRepository::new(pool.clone())),
            Box::new(PostgresAccessRequestRepository::new(pool.clone())),
        ));

        // Create rate limiting service
        let rate_limit_service: Arc<dyn RateLimitServiceTrait> = Arc::new(
            RedisRateLimitService::new(&redis_url).expect("Failed to create rate limit service"),
        );

        // Create cleanup service
        let cleanup_service = Arc::new(CleanupService::new(
            Box::new(PostgresRefreshTokenRepository::new(pool.clone())),
            Box::new(PostgresVerificationTokenRepository::new(pool.clone())),
            Box::new(PostgresPasswordResetTokenRepository::new(pool.clone())),
        ));

        Self {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            access_request_moderation_service,
            stats_service,
            rate_limit_service,
            cleanup_service,
        }
    }

    /// Factory method for testing with mocks
    #[cfg(feature = "mocks")]
    pub fn new_for_testing(jwt_secret: String) -> Self {
        // Create services with mock repositories for testing using builder pattern
        let auth_service = Arc::new(
            AuthService::builder()
                .user_repository(Box::new(MockUserRepository::new()))
                .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
                .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
                .incident_timer_repository(Box::new(MockIncidentTimerRepository::new()))
                .phrase_repository(Box::new(MockPhraseRepository::new()))
                .email_service(Box::new(MockEmailService::new()))
                .pkce_storage(Box::new(MockPkceStorage::new()))
                .jwt_secret(jwt_secret.clone())
                .build(),
        );

        let incident_timer_service = Arc::new(IncidentTimerService::new(Box::new(
            MockIncidentTimerRepository::new(),
        )));

        let phrase_service = Arc::new(PhraseService::new(Box::new(MockPhraseRepository::new())));

        let admin_service = Arc::new(UserManagementService::new(
            Box::new(MockUserRepository::new()),
            Box::new(MockRefreshTokenRepository::new()),
            Box::new(MockAdminRepository::new()),
        ));

        let phrase_moderation_service = Arc::new(PhraseModerationService::new(Box::new(
            MockPhraseRepository::new(),
        )));

        let access_request_moderation_service = Arc::new(AccessRequestModerationService::new(
            Box::new(MockAccessRequestRepository::new()),
        ));

        let stats_service = Arc::new(StatsService::new(
            Box::new(MockPhraseRepository::new()),
            Box::new(MockAdminRepository::new()),
            Box::new(MockAccessRequestRepository::new()),
        ));

        // For testing, use mock rate limiting service
        let rate_limit_service: Arc<dyn RateLimitServiceTrait> =
            Arc::new(MockRateLimitService::new());

        // For testing, use mock cleanup service
        let cleanup_service = Arc::new(CleanupService::new(
            Box::new(MockRefreshTokenRepository::new()),
            Box::new(MockVerificationTokenRepository::new()),
            Box::new(MockPasswordResetTokenRepository::new()),
        ));

        Self {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            access_request_moderation_service,
            stats_service,
            rate_limit_service,
            cleanup_service,
        }
    }

    /// Development environment - use PostgreSQL
    pub fn new_development(pool: PgPool, jwt_secret: String, redis_url: String) -> Self {
        Self::new(pool, jwt_secret, redis_url)
    }

    /// Testing environment - use mocks
    #[cfg(feature = "mocks")]
    pub fn new_testing(jwt_secret: String) -> Self {
        Self::new_for_testing(jwt_secret)
    }

    /// Production environment - use PostgreSQL with connection pooling
    pub fn new_production(pool: PgPool, jwt_secret: String, redis_url: String) -> Self {
        Self::new(pool, jwt_secret, redis_url)
    }
}
