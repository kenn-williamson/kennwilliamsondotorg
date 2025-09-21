// Removed unused imports
use sqlx::PgPool;
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

/// Wait for database to be ready with retry logic
pub async fn wait_for_database_ready(connection_string: &str) -> PgPool {
    let mut attempt = 0;
    let max_attempts = 10;
    
    while attempt < max_attempts {
        attempt += 1;
        println!("🔍 Database readiness check attempt {}/{}", attempt, max_attempts);
        
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
                        return pool;
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
        
        if attempt < max_attempts {
            let delay = std::cmp::min(1 << attempt, 8); // Exponential backoff, max 8 seconds
            println!("⏳ Waiting {}s before retry...", delay);
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
        }
    }
    
    panic!("Database failed to become ready after {} attempts", max_attempts);
}

/// Create a testcontainers database with proper setup
pub async fn create_testcontainers_database() -> (PgPool, String) {
    let image = GenericImage::new("ghcr.io/fboulnois/pg_uuidv7", "1.6.0")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"))
        .with_env_var("POSTGRES_DB", "testdb")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres");
    
    println!("🚀 Starting pg_uuidv7 container...");
    let _container = image.start().await.expect("Failed to start PostgreSQL container");
    println!("✅ Container started successfully");
    
    let port = _container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable", port);
    
    println!("🔗 Connection string: {}", connection_string);
    
    // Wait for database to be ready with retry logic
    let pool = wait_for_database_ready(&connection_string).await;
    println!("✅ Database connection established");
    
    // Enable pg_uuidv7 extension
    println!("🔧 Enabling pg_uuidv7 extension...");
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7")
        .execute(&pool)
        .await
        .expect("Failed to enable pg_uuidv7 extension");
    println!("✅ pg_uuidv7 extension enabled");
    
    // Run migrations
    println!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("✅ Migrations completed successfully");
    
    (pool, connection_string)
}

// ============================================================================
// TEST APP CREATION
// ============================================================================

/// Create a test app with testcontainers database
pub async fn create_test_app_with_testcontainers() -> (actix_test::TestServer, PgPool, TestContainer) {
    use backend::services::container::ServiceContainer;
    use backend::routes;
    use actix_web::{web, App};
    use actix_test;
    
    let test_container = TestContainer::new().await.expect("Failed to create test container");
    
    // Create service container
    let jwt_secret = "test-jwt-secret-for-api-tests".to_string();
    let container = ServiceContainer::new_development(test_container.pool.clone(), jwt_secret);
    
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
            .configure(routes::configure_app_routes)
    });
    
    (srv, test_container.pool.clone(), test_container)
}

/// Create a test app with a specific user pre-created
pub async fn create_test_app_with_user(
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<(actix_test::TestServer, PgPool, backend::models::db::user::User, TestContainer)> {
    let (srv, pool, test_container) = create_test_app_with_testcontainers().await;

    // Create test user directly in database
    let user = create_test_user_in_db(
        &pool,
        email,
        password_hash,
        display_name,
        slug,
    ).await?;

    Ok((srv, pool, user, test_container))
}

/// Create a test app with admin user pre-created
pub async fn create_test_app_with_admin_user(
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<(actix_test::TestServer, PgPool, backend::models::db::user::User, TestContainer)> {
    let (srv, pool, test_container) = create_test_app_with_testcontainers().await;

    // Create test user
    let user = create_test_user_in_db(
        &pool,
        email,
        password_hash,
        display_name,
        slug,
    ).await?;

    // Add admin role
    add_admin_role_to_user(&pool, user.id).await?;

    Ok((srv, pool, user, test_container))
}

// ============================================================================
// USER CREATION AND MANAGEMENT
// ============================================================================

/// Creates a test user in the database
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
        SELECT u.id, u.email, u.password_hash, u.display_name, u.slug, u.active, u.created_at, u.updated_at,
               COALESCE(ARRAY_AGG(r.name) FILTER (WHERE r.name IS NOT NULL), ARRAY[]::text[]) as roles
        FROM users u
        LEFT JOIN user_roles ur ON u.id = ur.user_id
        LEFT JOIN roles r ON ur.role_id = r.id
        WHERE u.id = $1
        GROUP BY u.id, u.email, u.password_hash, u.display_name, u.slug, u.active, u.created_at, u.updated_at
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}

/// Add admin role to a user
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

// ============================================================================
// JWT TOKEN CREATION
// ============================================================================

/// Creates a JWT token for testing
pub async fn create_test_jwt_token(user: &backend::models::db::user::User) -> Result<String, anyhow::Error> {
    use backend::services::auth::jwt::JwtService;
    
    let jwt_secret = "test-jwt-secret-for-integration-tests".to_string();
    let jwt_service = JwtService::new(jwt_secret);
    
    jwt_service.generate_token(user).map_err(|e| e.into())
}

// ============================================================================
// TEST DATA GENERATION
// ============================================================================

/// Generates a unique test email
pub fn unique_test_email() -> String {
    format!("test_{}@test.com", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

/// Generates a unique test slug
pub fn unique_test_slug() -> String {
    format!("test-user-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

/// Test password hash for testing
pub fn test_password_hash() -> String {
    "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string()
}

/// Generates a unique test phrase
pub fn unique_test_phrase() -> String {
    format!("Test phrase {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

// ============================================================================
// DATABASE CLEANUP
// ============================================================================

/// Cleans up test database
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
    pub connection_string: String,
}

impl TestContainer {
    /// Create a new test container with database setup
    pub async fn new() -> Result<Self> {
        let image = GenericImage::new("ghcr.io/fboulnois/pg_uuidv7", "1.6.0")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"))
            .with_env_var("POSTGRES_DB", "testdb")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres");

        let container = image.start().await.expect("Failed to start PostgreSQL container");
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable", port);
        
        let pool = wait_for_database_ready(&connection_string).await;
        
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
        
        Ok(TestContainer {
            _container: Box::new(container),
            pool,
            connection_string,
        })
    }
}

// ============================================================================
// INTEGRATION TEST HELPERS (Testcontainers approach)
// ============================================================================

/// Create a test app for integration tests using testcontainers
pub async fn create_integration_test_app() -> Result<(actix_test::TestServer, sqlx::PgPool, TestContainer)> {
    let (srv, pool, test_container) = create_test_app_with_testcontainers().await;
    Ok((srv, pool, test_container))
}

/// Create a test app with user for integration tests
pub async fn create_integration_test_app_with_user(
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<(actix_test::TestServer, sqlx::PgPool, backend::models::db::user::User, TestContainer)> {
    let (srv, pool, user, test_container) = create_test_app_with_user(email, password_hash, display_name, slug).await?;
    Ok((srv, pool, user, test_container))
}

/// Create a test app with admin user for integration tests
pub async fn create_integration_test_app_with_admin_user(
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<(actix_test::TestServer, sqlx::PgPool, backend::models::db::user::User, TestContainer)> {
    let (srv, pool, user, test_container) = create_test_app_with_admin_user(email, password_hash, display_name, slug).await?;
    Ok((srv, pool, user, test_container))
}