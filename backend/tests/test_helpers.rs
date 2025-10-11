// Removed unused imports
use sqlx::PgPool;
use std::sync::Arc;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use anyhow::Result;

// ============================================================================
// TESTCONTAINERS DATABASE SETUP
// ============================================================================


// ============================================================================
// TEST CONTEXT WITH BUILDER PATTERN
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
impl TestContextBuilder {
    pub fn new() -> Self {
        Self { 
            redis_url: None,
            oauth_service: None,
        }
    }

    /// Configure with Redis URL for rate limiting tests
    pub fn with_redis(mut self, url: String) -> Self {
        self.redis_url = Some(url);
        self
    }

    /// Configure with OAuth service for OAuth tests
    pub fn with_oauth(mut self, oauth_service: backend::services::auth::oauth::MockGoogleOAuthService) -> Self {
        self.oauth_service = Some(oauth_service);
        self
    }

    /// Build the test context with all dependencies
    pub async fn build(self) -> TestContext {
        use backend::services::container::ServiceContainer;
        use backend::routes;
        use backend::middleware::rate_limiter::{MockRateLimitService, RedisRateLimitService};
        use actix_web::{web, App};
        use actix_test;
        use std::sync::Arc;
        use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;

        // Set FRONTEND_URL for tests (needed for email verification)
        unsafe {
            std::env::set_var("FRONTEND_URL", "https://localhost");
        }
        use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
        use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
        use backend::repositories::postgres::postgres_incident_timer_repository::PostgresIncidentTimerRepository;
        use backend::repositories::postgres::postgres_phrase_repository::PostgresPhraseRepository;
        use backend::repositories::postgres::postgres_admin_repository::PostgresAdminRepository;
        use backend::services::auth::AuthService;
        use backend::services::email::MockEmailService;
        use backend::services::incident_timer::IncidentTimerService;
        use backend::services::phrase::PhraseService;
        use backend::services::admin::{UserManagementService, PhraseModerationService, StatsService};

        let test_container = TestContainer::builder().build().await.expect("Failed to create test container");
        let jwt_secret = "test-jwt-secret-for-api-tests".to_string();

        // Create mock email service for testing
        let email_service = Arc::new(MockEmailService::new());

        // Create mock OAuth service for testing
        use backend::services::auth::oauth::MockGoogleOAuthService;
        let mock_oauth = self.oauth_service.unwrap_or_else(|| MockGoogleOAuthService::new());

        let auth_service = Arc::new(AuthService::builder()
            .user_repository(Box::new(PostgresUserRepository::new(test_container.pool.clone())))
            .refresh_token_repository(Box::new(PostgresRefreshTokenRepository::new(test_container.pool.clone())))
            .verification_token_repository(Box::new(PostgresVerificationTokenRepository::new(test_container.pool.clone())))
            .incident_timer_repository(Box::new(PostgresIncidentTimerRepository::new(test_container.pool.clone())))
            .phrase_repository(Box::new(PostgresPhraseRepository::new(test_container.pool.clone())))
            .email_service(Box::new(email_service.as_ref().clone()))
            .google_oauth_service(Box::new(mock_oauth))
            .pkce_storage(Box::new(backend::repositories::mocks::MockPkceStorage::new()))
            .jwt_secret(jwt_secret.clone())
            .build());

        let incident_timer_service = Arc::new(IncidentTimerService::new(
            Box::new(PostgresIncidentTimerRepository::new(test_container.pool.clone()))
        ));

        let phrase_service = Arc::new(PhraseService::new(
            Box::new(PostgresPhraseRepository::new(test_container.pool.clone()))
        ));

        let admin_service = Arc::new(UserManagementService::new(
            Box::new(PostgresUserRepository::new(test_container.pool.clone())),
            Box::new(PostgresRefreshTokenRepository::new(test_container.pool.clone())),
            Box::new(PostgresAdminRepository::new(test_container.pool.clone())),
        ));

        let phrase_moderation_service = Arc::new(PhraseModerationService::new(
            Box::new(PostgresPhraseRepository::new(test_container.pool.clone()))
        ));

        let stats_service = Arc::new(StatsService::new(
            Box::new(PostgresPhraseRepository::new(test_container.pool.clone())),
            Box::new(PostgresAdminRepository::new(test_container.pool.clone())),
        ));

        // Use Redis or mock rate limiter depending on configuration
        let rate_limit_service: Arc<dyn backend::middleware::rate_limiter::RateLimitServiceTrait> = if let Some(redis_url) = self.redis_url {
            Arc::new(RedisRateLimitService::new(&redis_url).expect("Failed to create Redis rate limiter"))
        } else {
            Arc::new(MockRateLimitService::new())
        };

        // Create cleanup service
        use backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository;
        let cleanup_service = Arc::new(backend::services::cleanup::CleanupService::new(
            Box::new(PostgresRefreshTokenRepository::new(test_container.pool.clone())),
            Box::new(PostgresVerificationTokenRepository::new(test_container.pool.clone())),
            Box::new(PostgresPasswordResetTokenRepository::new(test_container.pool.clone())),
        ));

        let container = ServiceContainer {
            auth_service,
            incident_timer_service,
            phrase_service,
            admin_service,
            phrase_moderation_service,
            stats_service,
            rate_limit_service,
            cleanup_service,
        };

        // Create test server
        let pool_clone = test_container.pool.clone();
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(pool_clone.clone()))
                .app_data(web::Data::from(container.auth_service.clone()))
                .app_data(web::Data::from(container.incident_timer_service.clone()))
                .app_data(web::Data::from(container.phrase_service.clone()))
                .app_data(web::Data::from(container.admin_service.clone()))
                .app_data(web::Data::from(container.phrase_moderation_service.clone()))
                .app_data(web::Data::from(container.stats_service.clone()))
                .app_data(web::Data::from(container.rate_limit_service.clone()))
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
    pub async fn create_verified_user(&self, email: &str, slug: &str) -> backend::models::db::User {
        use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
        use backend::repositories::traits::user_repository::{UserRepository, CreateUserData};

        let user_repo = PostgresUserRepository::new(self.pool.clone());
        let user_data = CreateUserData {
            email: email.to_string(),
            password_hash: "hash".to_string(),
            display_name: slug.to_string(),
            slug: slug.to_string(),
        };

        let user = user_repo.create_user(&user_data).await.unwrap();

        // Assign email-verified role (using dynamic query for test helpers)
        sqlx::query("INSERT INTO user_roles (user_id, role_id) SELECT $1, id FROM roles WHERE name = 'email-verified'")
            .bind(user.id)
            .execute(&self.pool)
            .await
            .unwrap();

        user
    }

    /// Create an unverified user (without email-verified role)
    pub async fn create_unverified_user(&self, email: &str, slug: &str) -> backend::models::db::User {
        use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
        use backend::repositories::traits::user_repository::{UserRepository, CreateUserData};

        let user_repo = PostgresUserRepository::new(self.pool.clone());
        let user_data = CreateUserData {
            email: email.to_string(),
            password_hash: "hash".to_string(),
            display_name: slug.to_string(),
            slug: slug.to_string(),
        };

        user_repo.create_user(&user_data).await.unwrap()
    }

    /// Create an OAuth user (with Google ID and email-verified role)
    pub async fn create_oauth_user(&self, email: &str, slug: &str, google_user_id: &str) -> backend::models::db::User {
        use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
        use backend::repositories::traits::user_repository::{UserRepository, CreateOAuthUserData};

        let user_repo = PostgresUserRepository::new(self.pool.clone());
        let user_data = CreateOAuthUserData {
            email: email.to_string(),
            display_name: slug.to_string(),
            slug: slug.to_string(),
            real_name: Some("OAuth User".to_string()),
            google_user_id: Some(google_user_id.to_string()),
        };

        user_repo.create_oauth_user(&user_data).await.unwrap()
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
        sqlx::query_as::<_, backend::models::db::User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}

// ============================================================================
// TEST APP CREATION
// ============================================================================





// ============================================================================
// USER CREATION AND MANAGEMENT
// ============================================================================

/// Creates a test user in the database
/// Used in: testcontainers_admin_api_tests.rs (4 times) - e.g. line 175 in test_deactivate_user_success()
#[allow(dead_code)]
pub async fn create_test_user_in_db(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<backend::models::db::user::User, sqlx::Error> {
    use backend::models::db::user::User;
    use sqlx::Row;
    
    // Insert user (let database generate the ID)
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, display_name, slug, active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, true, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .bind(slug)
    .fetch_one(pool)
    .await?;
    
    let user_id: uuid::Uuid = result.get("id");
    
    // Add user role
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) 
         SELECT $1, id FROM roles WHERE name = 'user'",
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    
    // Fetch the created user
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT u.id, u.email, u.password_hash, u.display_name, u.slug, u.active, u.real_name, u.google_user_id, u.timer_is_public, u.timer_show_in_list, u.created_at, u.updated_at
        FROM users u
        WHERE u.id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}

/// Add admin role to a user
/// Used in: Currently unused but available for future test scenarios
#[allow(dead_code)]
pub async fn add_admin_role_to_user(pool: &sqlx::PgPool, user_id: uuid::Uuid) -> Result<()> {
    // Get admin role ID
    let admin_role_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM roles WHERE name = 'admin'"
    )
    .fetch_one(pool)
    .await?;
    
    // Add user-role relationship
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)"
    )
    .bind(user_id)
    .bind(admin_role_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Assign admin role to a user (wrapper for add_admin_role_to_user)
#[allow(dead_code)]
pub async fn assign_admin_role(pool: &sqlx::PgPool, user_id: uuid::Uuid) {
    add_admin_role_to_user(pool, user_id).await.expect("Failed to assign admin role");
}

// ============================================================================
// JWT TOKEN CREATION
// ============================================================================

/// Creates a JWT token for testing with appropriate roles from test context
/// For most tests that don't set up roles in DB, includes email-verified for backward compatibility
/// For admin tests that use create_test_app_with_admin_user, will include admin role
#[allow(dead_code)]
pub async fn create_test_jwt_token(user: &backend::models::db::user::User) -> Result<String, anyhow::Error> {
    use backend::services::auth::jwt::JwtService;

    let jwt_secret = "test-jwt-secret-for-api-tests".to_string();
    let jwt_service = JwtService::new(jwt_secret);

    // For tests, we default to email-verified role for backward compatibility
    // Admin tests should explicitly create tokens with admin role, or we could fetch from DB
    // For simplicity in tests: include both email-verified and admin roles
    // Real production code fetches roles from DB
    jwt_service.generate_token(user, &["email-verified".to_string(), "admin".to_string()]).map_err(|e| e.into())
}

// ============================================================================
// ROLE MANAGEMENT FOR TESTS
// ============================================================================

/// Assign email-verified role to user (for testing purposes)
/// Used to simulate email verification in tests without going through the email flow
#[allow(dead_code)]
pub async fn assign_email_verified_role(pool: &sqlx::PgPool, user_id_str: &str) {
    // Get email-verified role ID
    let role_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM roles WHERE name = 'email-verified'"
    )
    .fetch_one(pool)
    .await
    .expect("Failed to get email-verified role ID");

    let user_uuid = uuid::Uuid::parse_str(user_id_str).expect("Invalid user ID");

    // Assign role to user
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
    )
    .bind(user_uuid)
    .bind(role_id)
    .execute(pool)
    .await
    .expect("Failed to assign email-verified role");
}

// ============================================================================
// TEST DATA GENERATION
// ============================================================================

/// Generates a unique test email
/// Used in: testcontainers_admin_api_tests.rs - e.g. line 36 in test_register_duplicate_email()
#[allow(dead_code)]
pub fn unique_test_email() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("test_{}_{}@test.com", 
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        counter
    )
}

/// Generates a unique test slug
/// Used in: testcontainers_admin_api_tests.rs - e.g. line 76 in test_get_system_stats_success()
#[allow(dead_code)]
pub fn unique_test_slug() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("test-user-{}-{}", 
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        counter
    )
}

/// Test password hash for testing
/// Used in: testcontainers_admin_api_tests.rs (17 times) - e.g. line 166 in test_deactivate_user_success(), line 205 in test_activate_user_success()
#[allow(dead_code)]
pub fn test_password_hash() -> String {
    "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string()
}

/// Generates a unique test phrase
/// Used in: Currently unused but available for future test scenarios
#[allow(dead_code)]
pub fn unique_test_phrase() -> String {
    format!("Test phrase {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

// ============================================================================
// DATABASE CLEANUP
// ============================================================================

/// Cleans up test database
/// Used in: Currently unused but available for future test scenarios
#[allow(dead_code)]
pub async fn cleanup_test_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Clean up test data
    sqlx::query("DELETE FROM user_excluded_phrases").execute(pool).await?;
    sqlx::query("DELETE FROM phrase_suggestions").execute(pool).await?;
    sqlx::query("DELETE FROM phrases").execute(pool).await?;
    sqlx::query("DELETE FROM incident_timers").execute(pool).await?;
    sqlx::query("DELETE FROM refresh_tokens").execute(pool).await?;
    sqlx::query("DELETE FROM user_roles").execute(pool).await?;
    sqlx::query("DELETE FROM users").execute(pool).await?;
    
    Ok(())
}

// ============================================================================
// REFRESH TOKEN UTILITIES
// ============================================================================

/// Creates a test refresh token in the database
/// Used in: refresh_token_validation.rs - e.g. line 123 in test_refresh_token_expiration()
#[allow(dead_code)]
pub async fn create_test_refresh_token_in_db(
    pool: &PgPool,
    user_id: uuid::Uuid,
    token_hash: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<backend::models::db::refresh_token::RefreshToken, sqlx::Error> {
    use backend::models::db::refresh_token::RefreshToken;
    
    let refresh_token = RefreshToken {
        id: uuid::Uuid::new_v4(),
        user_id,
        token_hash: token_hash.to_string(),
        device_info: None,
        expires_at,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_used_at: None,
    };
    
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, device_info, expires_at, created_at, updated_at, last_used_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(refresh_token.id)
    .bind(refresh_token.user_id)
    .bind(&refresh_token.token_hash)
    .bind(&refresh_token.device_info)
    .bind(refresh_token.expires_at)
    .bind(refresh_token.created_at)
    .bind(refresh_token.updated_at)
    .bind(refresh_token.last_used_at)
    .execute(pool)
    .await?;
    
    Ok(refresh_token)
}

// ============================================================================
// SAFETY CHECKS
// ============================================================================

/// Verifies test database URL is set
/// Used in: Currently unused but available for future test scenarios
#[allow(dead_code)]
pub fn verify_test_database_url() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/kennwilliamson_test".to_string());
    
    if !database_url.contains("test") {
        panic!("Test database URL must contain 'test' for safety");
    }
}

// ============================================================================
// TEST CONTAINER LIFECYCLE
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
                return Err(anyhow::anyhow!("Failed to setup database after {} total attempts (3 containers × 5 attempts each)", max_total_attempts));
            }
            // Clean up the previous container if it exists
            if let Some(old_container) = current_container.take() {
                println!("🧹 Cleaning up previous container...");
                // The container will be automatically cleaned up when dropped
                drop(old_container);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Brief pause
            }
            
            println!("🚀 Starting PostgreSQL container...");
            
            // Create a fresh image configuration for each attempt
            let image = GenericImage::new("ghcr.io/fboulnois/pg_uuidv7", "1.6.0")
                .with_exposed_port(5432.tcp())
                .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"))
                .with_env_var("POSTGRES_DB", "testdb")
                .with_env_var("POSTGRES_USER", "postgres")
                .with_env_var("POSTGRES_PASSWORD", "postgres");
            
            let container = match image.start().await {
                Ok(container) => container,
                Err(e) => {
                    println!("❌ Failed to start container: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    continue;
                }
            };
            
            let port = match container.get_host_port_ipv4(5432).await {
                Ok(port) => port,
                Err(e) => {
                    println!("❌ Failed to get port: {}", e);
                    // Store container for cleanup
                    current_container = Some(Box::new(container));
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    continue;
                }
            };
            
            let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable", port);
            
            // Try to connect with restart logic (tries 5 times, then signals restart)
            match Self::wait_for_database_ready_with_restart(&connection_string).await {
                Ok(pool) => {
                    // Enable pg_uuidv7 extension
                    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7")
                        .execute(&pool)
                        .await
                        .expect("Failed to enable pg_uuidv7 extension");
                    
                    // Run migrations
                    sqlx::migrate!("./migrations")
                        .run(&pool)
                        .await
                        .expect("Failed to run migrations");
                    
                    return Ok(TestContainer {
                        _container: Box::new(container),
                        pool,
                        connection_string,
                    });
                },
                Err(e) => {
                    println!("❌ Database setup failed: {}", e);
                    // Store container for cleanup on next iteration
                    current_container = Some(Box::new(container));
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }
    
    /// Wait for database to be ready with container restart logic
    async fn wait_for_database_ready_with_restart(
        connection_string: &str
    ) -> Result<PgPool> {
        let mut attempt = 0;
        let attempts_per_container = 5; // Try 5 times before restarting container
        
        loop {
            attempt += 1;
            println!("🔍 Database readiness check attempt {}", attempt);
            
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
                            println!("✅ Database is ready!");
                            return Ok(pool);
                        },
                        Err(e) => {
                            println!("⚠️  Connection established but query failed: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("❌ Connection failed: {}", e);
                }
            }
            
            // If we've tried 5 times, signal that we need to restart the container
            if attempt % attempts_per_container == 0 {
                println!("🔄 Container appears unresponsive after {} attempts, will restart...", attempt);
                return Err(anyhow::anyhow!("Container restart needed after {} attempts", attempt));
            }
            
            let delay = std::cmp::min(1 << (attempt % attempts_per_container), 8); // Exponential backoff, max 8 seconds
            println!("⏳ Waiting {}s before retry...", delay);
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
        }
    }
}

// ============================================================================
// INTEGRATION TEST HELPERS (Testcontainers approach)
// ============================================================================
