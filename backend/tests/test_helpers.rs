use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use backend::models::user::User;
use backend::models::incident_timer::IncidentTimer;

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
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, password_hash, display_name, slug)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        Uuid::new_v4(),
        email,
        password_hash,
        display_name,
        slug
    )
    .fetch_one(&mut *tx)
    .await?;

    // Add default 'user' role
    let role_id = sqlx::query_scalar!(
        "SELECT id FROM roles WHERE name = 'user'"
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT INTO user_roles (id, user_id, role_id) VALUES ($1, $2, $3)",
        Uuid::new_v4(),
        user.id,
        role_id
    )
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
    
    sqlx::query_as!(
        IncidentTimer,
        r#"
        INSERT INTO incident_timers (id, user_id, reset_timestamp, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        Uuid::new_v4(),
        user_id,
        reset_ts,
        notes
    )
    .fetch_one(pool)
    .await
}

/// Clean up all test data
pub async fn cleanup_test_db(pool: &PgPool) {
    let _ = sqlx::query!("DELETE FROM user_excluded_phrases").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM phrase_suggestions").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM refresh_tokens").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM user_roles").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(pool).await;
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