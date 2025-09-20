use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use backend::models::db::user::User;
use backend::models::db::incident_timer::IncidentTimer;
use backend::models::db::refresh_token::RefreshToken;

/// Safety check to ensure we're using the test database
pub fn verify_test_database_url() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_default();
    if !database_url.contains("kennwilliamson_test") {
        panic!("❌ SAFETY CHECK FAILED: Tests must use test database (kennwilliamson_test), not: {}", database_url);
    }
}

/// Create a test user directly in the database (bypasses auth system)
pub async fn create_test_user_in_db(
    pool: &PgPool,
    email: &str,
    password_hash: &str,  // Pre-hashed for testing
    display_name: &str,
    slug: &str,
) -> Result<User, sqlx::Error> {
    // Use a transaction to ensure atomicity
    let mut tx = pool.begin().await?;
    
    let user_id = Uuid::new_v4();
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, display_name, slug)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .bind(slug)
    .fetch_one(&mut *tx)
    .await?;

    // Add default 'user' role
    let role_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM roles WHERE name = 'user'"
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO user_roles (id, user_id, role_id) VALUES ($1, $2, $3)"
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .bind(role_id)
    .execute(&mut *tx)
    .await?;

    // Commit the transaction
    tx.commit().await?;

    Ok(user)
}

/// Create a test incident timer directly in the database
pub async fn create_test_timer_in_db(
    pool: &PgPool,
    user_id: Uuid,
    reset_timestamp: Option<DateTime<Utc>>,
    notes: Option<&str>,
) -> Result<IncidentTimer, sqlx::Error> {
    let reset_ts = reset_timestamp.unwrap_or_else(|| Utc::now());
    
    let timer_id = Uuid::new_v4();
    sqlx::query_as::<_, IncidentTimer>(
        r#"
        INSERT INTO incident_timers (id, user_id, reset_timestamp, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(timer_id)
    .bind(user_id)
    .bind(reset_ts)
    .bind(notes)
    .fetch_one(pool)
    .await
}

/// Clean up all test data
pub async fn cleanup_test_db(pool: &PgPool) {
    // Delete in order to respect foreign key constraints
    let _ = sqlx::query("DELETE FROM user_excluded_phrases").execute(pool).await;
    let _ = sqlx::query("DELETE FROM phrase_suggestions").execute(pool).await;
    let _ = sqlx::query("DELETE FROM refresh_tokens").execute(pool).await;
    let _ = sqlx::query("DELETE FROM user_roles").execute(pool).await;
    let _ = sqlx::query("DELETE FROM incident_timers").execute(pool).await;
    let _ = sqlx::query("DELETE FROM phrases").execute(pool).await; // Add phrases table
    let _ = sqlx::query("DELETE FROM users").execute(pool).await;
    
    // Reset sequences to avoid ID conflicts
    let _ = sqlx::query("ALTER SEQUENCE IF EXISTS phrases_id_seq RESTART WITH 1").execute(pool).await;
}

/// Generate a simple test password hash (for testing only - not secure!)
pub fn test_password_hash() -> String {
    // Using bcrypt with cost 4 for fast testing
    bcrypt::hash("TestPassword123!", 4).unwrap()
}

/// Generate unique test email
pub fn unique_test_email(prefix: &str) -> String {
    format!("{}{}@test.example", prefix, chrono::Utc::now().timestamp_millis())
}

/// Generate unique test slug  
pub fn unique_test_slug(prefix: &str) -> String {
    format!("{}-{}", prefix, chrono::Utc::now().timestamp_millis())
}

/// Create a test refresh token directly in the database
pub async fn create_test_refresh_token_in_db(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    expires_at: DateTime<Utc>,
    device_info: Option<serde_json::Value>,
) -> Result<RefreshToken, sqlx::Error> {
    let token_id = Uuid::new_v4();
    let device_info_value = device_info.unwrap_or(serde_json::Value::Null);
    
    sqlx::query_as::<_, RefreshToken>(
        r#"
        INSERT INTO refresh_tokens (id, user_id, token_hash, device_info, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(token_id)
    .bind(user_id)
    .bind(token_hash)
    .bind(device_info_value)
    .bind(expires_at)
    .fetch_one(pool)
    .await
}