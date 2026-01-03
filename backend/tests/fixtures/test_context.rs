// Test context and test container fixtures for integration tests
//
// Provides TestContext (full application with services) and TestContainer (just database)
// for different levels of integration testing.

use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use testcontainers::{
    GenericImage, ImageExt, ReuseDirective,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

/// TRUNCATE all test data tables and re-seed for container reuse.
/// This mirrors the seed data from migrations.
const RESET_SQL: &str = r#"
-- Truncate all data tables
TRUNCATE TABLE
    user_excluded_phrases,
    phrase_suggestions,
    phrases,
    incident_timers,
    refresh_tokens,
    verification_tokens,
    password_reset_tokens,
    unsubscribe_tokens,
    access_requests,
    user_roles,
    user_preferences,
    user_profiles,
    user_credentials,
    user_external_logins,
    email_suppressions,
    blog_posts,
    users,
    roles
CASCADE;

-- Re-seed roles (from migrations)
INSERT INTO roles (name, description) VALUES
    ('user', 'Standard user with basic permissions'),
    ('admin', 'Administrator with full permissions'),
    ('email-verified', 'User has verified their email address'),
    ('trusted-contact', 'Trusted contact with access to personal/family content');

-- Re-seed system user (from migrations)
INSERT INTO users (email, slug, display_name)
VALUES ('system@kennwilliamson.org', 'system', 'System');

-- Re-seed phrases (from migrations)
INSERT INTO phrases (phrase_text, created_by)
SELECT phrase_data.phrase, (SELECT id FROM users WHERE email = 'system@kennwilliamson.org' LIMIT 1)
FROM (VALUES
    ('The Watch Continues. The Walls Must Hold.'),
    ('By Grace you stand; by vigilance you endure.'),
    ('A Chain is Forged One Link at a Time. Keep it Unbroken.'),
    ('Each Day Forges a Stronger Will. Temper it with Awareness.'),
    ('This Path is a Razor''s Edge. Walk with Purpose and a Steady Soul.'),
    ('The Spirit is willing, but the flesh is weak. Clothe yourself in vigilance.'),
    ('Complacency is the Serpent in the Garden. Guard the Gates.'),
    ('This number is a monument to your resolve. Protect its foundation.'),
    ('A Garden Untended Grows to Weed. Tend the Soul with Prayer and Watchfulness.'),
    ('Navigating a Steady Course. Be Wary of the Coming Tides.'),
    ('The Beast is Caged, Not Slain. Do Not Forget the Strength of its Bars.'),
    ('The Higher the Ascent, the Sheerer the Ledge. Keep Your Gaze Fixed on the Summit.'),
    ('Kept by Grace, Tested by the World. Stay Close to the Shepherd.'),
    ('This Quiet is a Hard-Won Peace. Do Not Mistake it for a Truce.'),
    ('Refined by the Fire of Trial. May Your Resolve be as Pure as Gold.'),
    ('The Past is a Shoreline Behind You. The Ocean Ahead is Vast and Unseen.'),
    ('A Fortress of Habit Stands One Stone at a Time. Lay Today''s Stone with Care.'),
    ('The Light Holds, but Shadows Lengthen. Stay Rooted in your Conviction.'),
    ('Let This Streak be a Testament, Not a Trophy. A Trophy Gathers Dust; a Testament Must be Lived.'),
    ('Strength is a Gift, Not a Given. Honor it with Continued Discipline.')
) AS phrase_data(phrase);
"#;

// ============================================================================
// TEST CONTAINER - Database-only fixture
// ============================================================================

/// Test container that keeps the container alive for the duration of the test
pub struct TestContainer {
    _container: Box<dyn std::any::Any + Send + Sync>,
    pub pool: PgPool,
    #[allow(dead_code)] // Field is available for debugging and future use
    pub connection_string: String,
}

/// Builder for TestContainer
pub struct TestContainerBuilder {
    // Future: could add configuration options here
}

impl Default for TestContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestContainerBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(self) -> Result<TestContainer> {
        TestContainer::new_internal().await
    }
}

impl TestContainer {
    /// Builder pattern entry point
    pub fn builder() -> TestContainerBuilder {
        TestContainerBuilder::new()
    }

    /// Internal constructor - use builder() instead
    async fn new_internal() -> Result<Self> {
        let mut current_container: Option<Box<dyn std::any::Any + Send + Sync>> = None;
        let mut total_attempts = 0;
        let max_total_attempts = 15; // 3 containers × 5 attempts each

        loop {
            total_attempts += 1;
            if total_attempts > max_total_attempts {
                return Err(anyhow::anyhow!(
                    "Failed to setup database after {} total attempts (3 containers × 5 attempts each)",
                    max_total_attempts
                ));
            }
            // Clean up the previous container if it exists
            if let Some(old_container) = current_container.take() {
                // The container will be automatically cleaned up when dropped
                drop(old_container);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Brief pause
            }

            // Use slot-based container name for parallel test support
            // Each nextest slot gets its own reusable container
            // See: https://nexte.st/docs/configuration/env-vars/
            let slot_id =
                std::env::var("NEXTEST_TEST_GLOBAL_SLOT").unwrap_or_else(|_| "0".to_string());
            let container_name = format!("test-postgres-{}", slot_id);

            // Create image with reuse enabled for faster test execution
            let image = GenericImage::new("ghcr.io/fboulnois/pg_uuidv7", "1.6.0")
                .with_exposed_port(5432.tcp())
                .with_wait_for(WaitFor::message_on_stdout(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_DB", "testdb")
                .with_env_var("POSTGRES_USER", "postgres")
                .with_env_var("POSTGRES_PASSWORD", "postgres")
                .with_container_name(&container_name)
                .with_reuse(ReuseDirective::Always);

            let container = match image.start().await {
                Ok(container) => container,
                Err(e) => {
                    eprintln!("❌ Failed to start container: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    continue;
                }
            };

            let port = match container.get_host_port_ipv4(5432).await {
                Ok(port) => port,
                Err(e) => {
                    eprintln!("❌ Failed to get port: {}", e);
                    // Store container for cleanup
                    current_container = Some(Box::new(container));
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    continue;
                }
            };

            let connection_string = format!(
                "postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable",
                port
            );

            // Try to connect with restart logic (tries 5 times, then signals restart)
            match Self::wait_for_database_ready_with_restart(&connection_string).await {
                Ok(pool) => {
                    // Enable pg_uuidv7 extension
                    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7")
                        .execute(&pool)
                        .await
                        .expect("Failed to enable pg_uuidv7 extension");

                    // Run migrations (idempotent - won't re-run if already done)
                    sqlx::migrate!("./migrations")
                        .run(&pool)
                        .await
                        .expect("Failed to run migrations");

                    // TRUNCATE all data for clean slate (reused containers have old data)
                    sqlx::raw_sql(RESET_SQL)
                        .execute(&pool)
                        .await
                        .expect("Failed to reset database");

                    return Ok(TestContainer {
                        _container: Box::new(container),
                        pool,
                        connection_string,
                    });
                }
                Err(e) => {
                    eprintln!("❌ Database setup failed: {}", e);
                    // Store container for cleanup on next iteration
                    current_container = Some(Box::new(container));
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    /// Wait for database to be ready with container restart logic
    async fn wait_for_database_ready_with_restart(connection_string: &str) -> Result<PgPool> {
        let mut attempt = 0;
        let attempts_per_container = 5; // Try 5 times before restarting container

        loop {
            attempt += 1;

            match sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(std::time::Duration::from_secs(10))
                .idle_timeout(std::time::Duration::from_secs(600))
                .connect(connection_string)
                .await
            {
                Ok(pool) => {
                    // Test the connection
                    match sqlx::query("SELECT 1").fetch_one(&pool).await {
                        Ok(_) => {
                            return Ok(pool);
                        }
                        Err(_e) => {
                            // Query failed, will retry
                        }
                    }
                }
                Err(_e) => {
                    // Connection failed, will retry
                }
            }

            // If we've tried 5 times, signal that we need to restart the container
            if attempt % attempts_per_container == 0 {
                eprintln!(
                    "⚠️  Database unresponsive after {} attempts, restarting container...",
                    attempt
                );
                return Err(anyhow::anyhow!(
                    "Container restart needed after {} attempts",
                    attempt
                ));
            }

            let delay = std::cmp::min(1 << (attempt % attempts_per_container), 8); // Exponential backoff, max 8 seconds
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
        }
    }
}

// ============================================================================
// TEST CONTEXT - Full application fixture with services
// ============================================================================

/// Test context that holds all test dependencies
/// Provides a clean interface for integration tests with builder pattern
#[allow(dead_code)]
pub struct TestContext {
    pub server: actix_test::TestServer,
    pub pool: PgPool,
    pub email_service: Arc<backend::services::email::MockEmailService>,
    _container: TestContainer,
}

/// Builder for TestContext
#[allow(dead_code)]
pub struct TestContextBuilder {
    redis_url: Option<String>,
    oauth_service: Option<backend::services::auth::oauth::MockGoogleOAuthService>,
}

#[allow(dead_code)]
impl Default for TestContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestContextBuilder {
    pub fn new() -> Self {
        Self {
            redis_url: None,
            oauth_service: None,
        }
    }

    /// Configure with Redis URL for rate limiting tests
    #[allow(dead_code)]
    pub fn with_redis(mut self, url: String) -> Self {
        self.redis_url = Some(url);
        self
    }

    /// Configure with OAuth service for OAuth tests
    #[allow(dead_code)]
    pub fn with_oauth(
        mut self,
        oauth_service: backend::services::auth::oauth::MockGoogleOAuthService,
    ) -> Self {
        self.oauth_service = Some(oauth_service);
        self
    }

    /// Build the test context with all dependencies
    #[allow(dead_code)]
    pub async fn build(self) -> TestContext {
        use actix_test;
        use actix_web::{App, web};
        use backend::middleware::rate_limiter::{MockRateLimitService, RedisRateLimitService};
        use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
        use backend::routes;
        use backend::services::container::ServiceContainer;
        use std::sync::Arc;

        // Set FRONTEND_URL for tests (needed for email verification)
        unsafe {
            std::env::set_var("FRONTEND_URL", "https://localhost");
        }
        use backend::events::event_bus::InMemoryEventBus;
        use backend::events::handlers::UserRegisteredEmailHandler;
        use backend::events::types::UserRegisteredEvent;
        use backend::events::{EventBus, EventPublisher};
        use backend::repositories::postgres::postgres_access_request_repository::PostgresAccessRequestRepository;
        use backend::repositories::postgres::postgres_admin_repository::PostgresAdminRepository;
        use backend::repositories::postgres::postgres_incident_timer_repository::PostgresIncidentTimerRepository;
        use backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository;
        use backend::repositories::postgres::postgres_phrase_repository::PostgresPhraseRepository;
        use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
        use backend::repositories::postgres::postgres_unsubscribe_token_repository::PostgresUnsubscribeTokenRepository;
        use backend::repositories::postgres::postgres_user_credentials_repository::PostgresUserCredentialsRepository;
        use backend::repositories::postgres::postgres_user_external_login_repository::PostgresUserExternalLoginRepository;
        use backend::repositories::postgres::postgres_user_preferences_repository::PostgresUserPreferencesRepository;
        use backend::repositories::postgres::postgres_user_profile_repository::PostgresUserProfileRepository;
        use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
        use backend::services::admin::{
            AccessRequestModerationService, PhraseModerationService, StatsService,
            UserManagementService,
        };
        use backend::services::auth::AuthService;
        use backend::services::email::MockEmailService;
        use backend::services::incident_timer::IncidentTimerService;
        use backend::services::phrase::PhraseService;

        let test_container = TestContainer::builder()
            .build()
            .await
            .expect("Failed to create test container");
        let jwt_secret = "test-jwt-secret-for-api-tests".to_string();

        // Create mock email service for testing
        let email_service = Arc::new(MockEmailService::new());

        // Create event bus and register UserRegisteredEmailHandler
        let mut event_bus = InMemoryEventBus::new();

        let user_registered_handler = UserRegisteredEmailHandler::new(
            Arc::new(PostgresVerificationTokenRepository::new(
                test_container.pool.clone(),
            )),
            email_service.clone(),
            "https://localhost",
        );
        event_bus
            .register_handler::<UserRegisteredEvent>(Box::new(user_registered_handler))
            .expect("Failed to register UserRegisteredEmailHandler");

        let event_publisher: Arc<dyn EventPublisher> = Arc::new(event_bus);

        // Create mock OAuth service for testing
        let mock_oauth = self.oauth_service.unwrap_or_default();

        let auth_service = Arc::new(
            AuthService::builder()
                .user_repository(Box::new(PostgresUserRepository::new(
                    test_container.pool.clone(),
                )))
                .refresh_token_repository(Box::new(PostgresRefreshTokenRepository::new(
                    test_container.pool.clone(),
                )))
                .verification_token_repository(Box::new(PostgresVerificationTokenRepository::new(
                    test_container.pool.clone(),
                )))
                .password_reset_token_repository(Box::new(
                    PostgresPasswordResetTokenRepository::new(test_container.pool.clone()),
                ))
                .incident_timer_repository(Box::new(PostgresIncidentTimerRepository::new(
                    test_container.pool.clone(),
                )))
                .phrase_repository(Box::new(PostgresPhraseRepository::new(
                    test_container.pool.clone(),
                )))
                .credentials_repository(Box::new(PostgresUserCredentialsRepository::new(
                    test_container.pool.clone(),
                )))
                .external_login_repository(Box::new(PostgresUserExternalLoginRepository::new(
                    test_container.pool.clone(),
                )))
                .profile_repository(Box::new(PostgresUserProfileRepository::new(
                    test_container.pool.clone(),
                )))
                .preferences_repository(Box::new(PostgresUserPreferencesRepository::new(
                    test_container.pool.clone(),
                )))
                .unsubscribe_token_repository(Box::new(PostgresUnsubscribeTokenRepository::new(
                    test_container.pool.clone(),
                )))
                .email_service(Box::new(email_service.as_ref().clone()))
                .google_oauth_service(Box::new(mock_oauth))
                .pkce_storage(Box::new(
                    backend::repositories::mocks::MockPkceStorage::new(),
                ))
                .event_publisher(event_publisher)
                .jwt_secret(jwt_secret.clone())
                .build(),
        );

        let incident_timer_service = Arc::new(IncidentTimerService::new(Box::new(
            PostgresIncidentTimerRepository::new(test_container.pool.clone()),
        )));

        let phrase_service = Arc::new(PhraseService::new(Box::new(PostgresPhraseRepository::new(
            test_container.pool.clone(),
        ))));

        let admin_service = Arc::new(UserManagementService::new(
            Box::new(PostgresUserRepository::new(test_container.pool.clone())),
            Box::new(PostgresRefreshTokenRepository::new(
                test_container.pool.clone(),
            )),
            Box::new(PostgresAdminRepository::new(test_container.pool.clone())),
        ));

        let phrase_moderation_service = Arc::new(PhraseModerationService::new(Box::new(
            PostgresPhraseRepository::new(test_container.pool.clone()),
        )));

        let access_request_moderation_service =
            Arc::new(AccessRequestModerationService::new(Box::new(
                PostgresAccessRequestRepository::new(test_container.pool.clone()),
            )));

        let stats_service = Arc::new(StatsService::new(
            Box::new(PostgresPhraseRepository::new(test_container.pool.clone())),
            Box::new(PostgresAdminRepository::new(test_container.pool.clone())),
            Box::new(PostgresAccessRequestRepository::new(
                test_container.pool.clone(),
            )),
        ));

        // Use Redis or mock rate limiter depending on configuration
        let rate_limit_service: Arc<dyn backend::middleware::rate_limiter::RateLimitServiceTrait> =
            if let Some(redis_url) = self.redis_url {
                Arc::new(
                    RedisRateLimitService::new(&redis_url)
                        .expect("Failed to create Redis rate limiter"),
                )
            } else {
                Arc::new(MockRateLimitService::new())
            };

        // Create cleanup service
        let cleanup_service = Arc::new(backend::services::cleanup::CleanupService::new(
            Box::new(PostgresRefreshTokenRepository::new(
                test_container.pool.clone(),
            )),
            Box::new(PostgresVerificationTokenRepository::new(
                test_container.pool.clone(),
            )),
            Box::new(PostgresPasswordResetTokenRepository::new(
                test_container.pool.clone(),
            )),
        ));

        // Create blog service for API testing
        use backend::repositories::mocks::MockImageStorage;
        use backend::repositories::postgres::postgres_blog_repository::PostgresBlogRepository;
        use backend::services::blog::BlogService;

        let blog_service = Arc::new(
            BlogService::builder()
                .with_repository(Box::new(PostgresBlogRepository::new(
                    test_container.pool.clone(),
                )))
                .with_image_storage(Box::new(MockImageStorage::new()))
                .build()
                .expect("Failed to build BlogService"),
        );

        // Create feed service for API testing
        use backend::services::feed::FeedService;

        let feed_service = Arc::new(
            FeedService::builder()
                .with_repository(Box::new(PostgresBlogRepository::new(
                    test_container.pool.clone(),
                )))
                .build()
                .expect("Failed to build FeedService"),
        );

        // Create turnstile service for testing (always succeeds)
        use backend::services::turnstile::{MockTurnstileService, TurnstileServiceTrait};
        let turnstile_service: Arc<dyn TurnstileServiceTrait> =
            Arc::new(MockTurnstileService::new_success());

        let container = ServiceContainer {
            auth_service,
            blog_service,
            feed_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            access_request_moderation_service,
            stats_service,
            rate_limit_service,
            turnstile_service,
            cleanup_service,
        };

        // Create test server
        let pool_clone = test_container.pool.clone();
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(pool_clone.clone()))
                .app_data(web::Data::from(container.auth_service.clone()))
                .app_data(web::Data::from(container.blog_service.clone()))
                .app_data(web::Data::from(container.feed_service.clone()))
                .app_data(web::Data::from(container.incident_timer_service.clone()))
                .app_data(web::Data::from(container.phrase_service.clone()))
                .app_data(web::Data::from(container.admin_service.clone()))
                .app_data(web::Data::from(container.phrase_moderation_service.clone()))
                .app_data(web::Data::from(
                    container.access_request_moderation_service.clone(),
                ))
                .app_data(web::Data::from(container.stats_service.clone()))
                .app_data(web::Data::from(container.rate_limit_service.clone()))
                .app_data(web::Data::from(container.turnstile_service.clone()))
                .configure(routes::configure_app_routes)
        });

        TestContext {
            server: srv,
            pool: test_container.pool.clone(),
            email_service,
            _container: test_container,
        }
    }
}

#[allow(dead_code)]
impl TestContext {
    pub fn builder() -> TestContextBuilder {
        TestContextBuilder::new()
    }

    /// Create a verified user (with email-verified role)
    /// Uses UserBuilder pattern for resilient test fixtures
    pub async fn create_verified_user(&self, email: &str, slug: &str) -> backend::models::db::User {
        use backend::test_utils::UserBuilder;

        let user = UserBuilder::new()
            .with_email(email)
            .with_slug(slug)
            .with_display_name(slug)
            .with_password("test_password")
            .persist(&self.pool)
            .await
            .expect("Failed to create verified user");

        // Assign email-verified role
        sqlx::query("INSERT INTO user_roles (user_id, role_id) SELECT $1, id FROM roles WHERE name = 'email-verified'")
            .bind(user.id)
            .execute(&self.pool)
            .await
            .expect("Failed to assign email-verified role");

        user
    }

    /// Create an unverified user (without email-verified role)
    /// Uses UserBuilder pattern for resilient test fixtures
    pub async fn create_unverified_user(
        &self,
        email: &str,
        slug: &str,
    ) -> backend::models::db::User {
        use backend::test_utils::UserBuilder;

        UserBuilder::new()
            .with_email(email)
            .with_slug(slug)
            .with_display_name(slug)
            .with_password("test_password")
            .persist(&self.pool)
            .await
            .expect("Failed to create unverified user")
    }

    /// Create an OAuth user (with Google ID and email-verified role)
    /// Uses UserBuilder pattern for resilient test fixtures
    pub async fn create_oauth_user(
        &self,
        email: &str,
        slug: &str,
        google_user_id: &str,
    ) -> backend::models::db::User {
        use backend::test_utils::UserBuilder;

        let user = UserBuilder::new()
            .oauth(google_user_id, "OAuth User")
            .with_email(email)
            .with_slug(slug)
            .with_display_name(slug)
            .persist(&self.pool)
            .await
            .expect("Failed to create OAuth user");

        // OAuth users get email-verified role automatically
        sqlx::query("INSERT INTO user_roles (user_id, role_id) SELECT $1, id FROM roles WHERE name = 'email-verified'")
            .bind(user.id)
            .execute(&self.pool)
            .await
            .expect("Failed to assign email-verified role");

        user
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, user_id: uuid::Uuid) -> backend::models::db::User {
        use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
        use backend::repositories::traits::user_repository::UserRepository;

        let user_repo = PostgresUserRepository::new(self.pool.clone());
        user_repo.find_by_id(user_id).await.unwrap().unwrap()
    }

    /// Get all users with a specific email
    pub async fn get_users_by_email(&self, email: &str) -> Vec<backend::models::db::User> {
        sqlx::query_as::<_, backend::models::db::User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
}
