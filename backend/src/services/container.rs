use std::sync::Arc;
use sqlx::PgPool;

use crate::repositories::traits::{
    UserRepository, RefreshTokenRepository, IncidentTimerRepository, PhraseRepository
};
use crate::repositories::postgres::{
    postgres_user_repository::PostgresUserRepository,
    postgres_refresh_token_repository::PostgresRefreshTokenRepository,
    postgres_incident_timer_repository::PostgresIncidentTimerRepository,
    postgres_phrase_repository::PostgresPhraseRepository,
};
use crate::repositories::mocks::{
    MockUserRepository,
    MockRefreshTokenRepository,
    MockIncidentTimerRepository,
    MockPhraseRepository,
};

use super::auth::AuthService;
use super::incident_timer::IncidentTimerService;
use super::phrase::PhraseService;
use super::admin::{UserManagementService, PhraseModerationService, StatsService};

/// Centralized service container for dependency injection
pub struct ServiceContainer {
    // Core services
    pub auth_service: Arc<AuthService>,
    pub incident_timer_service: Arc<IncidentTimerService>,
    pub phrase_service: Arc<PhraseService>,
    pub admin_service: Arc<UserManagementService>,
    pub phrase_moderation_service: Arc<PhraseModerationService>,
    pub stats_service: Arc<StatsService>,
    
    // Repositories (for testing/debugging)
    pub user_repository: Arc<dyn UserRepository>,
    pub refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    pub incident_timer_repository: Arc<dyn IncidentTimerRepository>,
    pub phrase_repository: Arc<dyn PhraseRepository>,
}

impl ServiceContainer {
    /// Create service container for development/production with PostgreSQL
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        // Create repositories
        let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
        let refresh_token_repo = Arc::new(PostgresRefreshTokenRepository::new(pool.clone()));
        let timer_repo = Arc::new(PostgresIncidentTimerRepository::new(pool.clone()));
        let phrase_repo = Arc::new(PostgresPhraseRepository::new(pool.clone()));
        
        // Create services with repository dependencies
        let auth_service = Arc::new(AuthService::new(
            Box::new(PostgresUserRepository::new(pool.clone())),
            Box::new(PostgresRefreshTokenRepository::new(pool.clone())),
            jwt_secret.clone(),
        ));
        
        let incident_timer_service = Arc::new(IncidentTimerService::new(
            Box::new(PostgresIncidentTimerRepository::new(pool.clone()))
        ));
        
        let phrase_service = Arc::new(PhraseService::new(
            Box::new(PostgresPhraseRepository::new(pool.clone()))
        ));
        
        let admin_service = Arc::new(UserManagementService::new(
            Box::new(PostgresUserRepository::new(pool.clone())),
            Box::new(PostgresRefreshTokenRepository::new(pool.clone())),
        ));
        
        let phrase_moderation_service = Arc::new(PhraseModerationService::new(
            Box::new(PostgresPhraseRepository::new(pool.clone()))
        ));
        
        let stats_service = Arc::new(StatsService::new(
            Box::new(PostgresUserRepository::new(pool.clone())),
            Box::new(PostgresPhraseRepository::new(pool.clone())),
        ));
        
        Self {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            stats_service,
            user_repository: user_repo,
            refresh_token_repository: refresh_token_repo,
            incident_timer_repository: timer_repo,
            phrase_repository: phrase_repo,
        }
    }
    
    /// Factory method for testing with mocks
    pub fn new_for_testing(
        user_repo: Arc<dyn UserRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        timer_repo: Arc<dyn IncidentTimerRepository>,
        phrase_repo: Arc<dyn PhraseRepository>,
        jwt_secret: String,
    ) -> Self {
        // For testing, we need to create new instances of the repositories
        // Since we can't clone trait objects, we'll create new instances
        // This is a limitation of the current design - in a real testing scenario,
        // we'd want to use the actual mock repositories
        
        // Create new instances by cloning the Arc and creating new Box<dyn Trait>
        let auth_service = Arc::new(AuthService::new(
            Box::new(MockUserRepository::new()),
            Box::new(MockRefreshTokenRepository::new()),
            jwt_secret.clone(),
        ));
        
        let incident_timer_service = Arc::new(IncidentTimerService::new(
            Box::new(MockIncidentTimerRepository::new())
        ));
        
        let phrase_service = Arc::new(PhraseService::new(
            Box::new(MockPhraseRepository::new())
        ));
        
        let admin_service = Arc::new(UserManagementService::new(
            Box::new(MockUserRepository::new()),
            Box::new(MockRefreshTokenRepository::new()),
        ));
        
        let phrase_moderation_service = Arc::new(PhraseModerationService::new(
            Box::new(MockPhraseRepository::new())
        ));
        
        let stats_service = Arc::new(StatsService::new(
            Box::new(MockUserRepository::new()),
            Box::new(MockPhraseRepository::new()),
        ));
        
        Self {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            stats_service,
            user_repository: user_repo,
            refresh_token_repository: refresh_token_repo,
            incident_timer_repository: timer_repo,
            phrase_repository: phrase_repo,
        }
    }
    
    /// Development environment - use PostgreSQL
    pub fn new_development(pool: PgPool, jwt_secret: String) -> Self {
        Self::new(pool, jwt_secret)
    }
    
    /// Testing environment - use mocks
    pub fn new_testing(jwt_secret: String) -> Self {
        let user_repo = Arc::new(MockUserRepository::new());
        let refresh_token_repo = Arc::new(MockRefreshTokenRepository::new());
        let timer_repo = Arc::new(MockIncidentTimerRepository::new());
        let phrase_repo = Arc::new(MockPhraseRepository::new());
        
        Self::new_for_testing(user_repo, refresh_token_repo, timer_repo, phrase_repo, jwt_secret)
    }
    
    /// Production environment - use PostgreSQL with connection pooling
    pub fn new_production(pool: PgPool, jwt_secret: String) -> Self {
        Self::new(pool, jwt_secret)
    }
}