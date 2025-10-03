use sqlx::PgPool;
use std::sync::Arc;

use crate::repositories::mocks::{
    MockAdminRepository, MockIncidentTimerRepository, MockPhraseRepository,
    MockRefreshTokenRepository, MockUserRepository, MockVerificationTokenRepository,
};
use crate::repositories::postgres::{
    postgres_admin_repository::PostgresAdminRepository,
    postgres_incident_timer_repository::PostgresIncidentTimerRepository,
    postgres_phrase_repository::PostgresPhraseRepository,
    postgres_refresh_token_repository::PostgresRefreshTokenRepository,
    postgres_user_repository::PostgresUserRepository,
    postgres_verification_token_repository::PostgresVerificationTokenRepository,
};

use super::admin::{PhraseModerationService, StatsService, UserManagementService};
use super::auth::AuthService;
use super::cleanup::CleanupService;
use super::email::{MockEmailService, SesEmailService};
use super::incident_timer::IncidentTimerService;
use super::phrase::PhraseService;
use crate::middleware::rate_limiter::{
    MockRateLimitService, RateLimitServiceTrait, RedisRateLimitService,
};

/// Centralized service container for dependency injection
pub struct ServiceContainer {
    // Core services
    pub auth_service: Arc<AuthService>,
    pub incident_timer_service: Arc<IncidentTimerService>,
    pub phrase_service: Arc<PhraseService>,
    pub admin_service: Arc<UserManagementService>,
    pub phrase_moderation_service: Arc<PhraseModerationService>,
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

        // Create services with repository dependencies using builder pattern
        let auth_service = Arc::new(
            AuthService::builder()
                .user_repository(Box::new(PostgresUserRepository::new(pool.clone())))
                .refresh_token_repository(Box::new(PostgresRefreshTokenRepository::new(
                    pool.clone(),
                )))
                .verification_token_repository(Box::new(
                    PostgresVerificationTokenRepository::new(pool.clone()),
                ))
                .email_service(Box::new(SesEmailService::new(from_email, reply_to_email)))
                .jwt_secret(jwt_secret.clone())
                .build(),
        );

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

        let stats_service = Arc::new(StatsService::new(
            Box::new(PostgresPhraseRepository::new(pool.clone())),
            Box::new(PostgresAdminRepository::new(pool.clone())),
        ));

        // Create rate limiting service
        let rate_limit_service: Arc<dyn RateLimitServiceTrait> = Arc::new(
            RedisRateLimitService::new(&redis_url).expect("Failed to create rate limit service"),
        );

        // Create cleanup service
        let cleanup_service = Arc::new(CleanupService::new(
            Box::new(PostgresRefreshTokenRepository::new(pool.clone())),
            Box::new(PostgresVerificationTokenRepository::new(pool.clone())),
        ));

        Self {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            stats_service,
            rate_limit_service,
            cleanup_service,
        }
    }

    /// Factory method for testing with mocks
    pub fn new_for_testing(jwt_secret: String) -> Self {
        // Create services with mock repositories for testing using builder pattern
        let auth_service = Arc::new(
            AuthService::builder()
                .user_repository(Box::new(MockUserRepository::new()))
                .refresh_token_repository(Box::new(MockRefreshTokenRepository::new()))
                .verification_token_repository(Box::new(MockVerificationTokenRepository::new()))
                .email_service(Box::new(MockEmailService::new()))
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

        let stats_service = Arc::new(StatsService::new(
            Box::new(MockPhraseRepository::new()),
            Box::new(MockAdminRepository::new()),
        ));

        // For testing, use mock rate limiting service
        let rate_limit_service: Arc<dyn RateLimitServiceTrait> =
            Arc::new(MockRateLimitService::new());

        // For testing, use mock cleanup service
        let cleanup_service = Arc::new(CleanupService::new(
            Box::new(MockRefreshTokenRepository::new()),
            Box::new(MockVerificationTokenRepository::new()),
        ));

        Self {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
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
    pub fn new_testing(jwt_secret: String) -> Self {
        Self::new_for_testing(jwt_secret)
    }

    /// Production environment - use PostgreSQL with connection pooling
    pub fn new_production(pool: PgPool, jwt_secret: String, redis_url: String) -> Self {
        Self::new(pool, jwt_secret, redis_url)
    }
}
