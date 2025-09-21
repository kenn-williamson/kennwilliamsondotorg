use actix_web::{test, web, App};
use sqlx::PgPool;

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

/// Creates a JWT token for testing
pub async fn create_test_jwt_token(user: &backend::models::db::user::User) -> Result<String, anyhow::Error> {
    use backend::services::auth::jwt::JwtService;
    
    let jwt_secret = "test-jwt-secret-for-integration-tests".to_string();
    let jwt_service = JwtService::new(jwt_secret);
    
    jwt_service.generate_token(user).map_err(|e| e.into())
}

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

// Note: create_test_app and create_test_app_with_user functions removed due to complex return type issues
// Tests should use the integration test helpers in tests/integration/test_app.rs instead

/// Verifies test database URL is set
pub fn verify_test_database_url() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/kennwilliamson_test".to_string());
    
    if !database_url.contains("test") {
        panic!("Test database URL must contain 'test' for safety");
    }
}

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