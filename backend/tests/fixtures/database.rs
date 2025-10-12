// Database utility fixtures for tests

use sqlx::PgPool;
use anyhow::Result;

// ============================================================================
// USER CREATION
// ============================================================================

/// Creates a test user in the database
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

/// Create a verified user (with email-verified role)
#[allow(dead_code)]
pub async fn create_verified_user(pool: &PgPool, email: &str, slug: &str) -> backend::models::db::User {
    use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
    use backend::repositories::traits::user_repository::{UserRepository, CreateUserData};

    let user_repo = PostgresUserRepository::new(pool.clone());
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
        .execute(pool)
        .await
        .unwrap();

    user
}

/// Create an unverified user (without email-verified role)
#[allow(dead_code)]
pub async fn create_unverified_user(pool: &PgPool, email: &str, slug: &str) -> backend::models::db::User {
    use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
    use backend::repositories::traits::user_repository::{UserRepository, CreateUserData};

    let user_repo = PostgresUserRepository::new(pool.clone());
    let user_data = CreateUserData {
        email: email.to_string(),
        password_hash: "hash".to_string(),
        display_name: slug.to_string(),
        slug: slug.to_string(),
    };

    user_repo.create_user(&user_data).await.unwrap()
}

/// Create an OAuth user (with Google ID and email-verified role)
#[allow(dead_code)]
pub async fn create_oauth_user(pool: &PgPool, email: &str, slug: &str, google_user_id: &str) -> backend::models::db::User {
    use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
    use backend::repositories::traits::user_repository::{UserRepository, CreateOAuthUserData};

    let user_repo = PostgresUserRepository::new(pool.clone());
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
#[allow(dead_code)]
pub async fn get_user_by_id(pool: &PgPool, user_id: uuid::Uuid) -> backend::models::db::User {
    use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
    use backend::repositories::traits::user_repository::UserRepository;

    let user_repo = PostgresUserRepository::new(pool.clone());
    user_repo.find_by_id(user_id).await.unwrap().unwrap()
}

/// Get all users with a specific email
#[allow(dead_code)]
pub async fn get_users_by_email(pool: &PgPool, email: &str) -> Vec<backend::models::db::User> {
    sqlx::query_as::<_, backend::models::db::User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_all(pool)
    .await
    .unwrap()
}

// ============================================================================
// ROLE MANAGEMENT
// ============================================================================

/// Add admin role to a user
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
// REFRESH TOKENS
// ============================================================================

/// Creates a test refresh token in the database
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
// DATABASE CLEANUP
// ============================================================================

/// Cleans up test database
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
// SAFETY CHECKS
// ============================================================================

/// Verifies test database URL is set
#[allow(dead_code)]
pub fn verify_test_database_url() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/kennwilliamson_test".to_string());

    if !database_url.contains("test") {
        panic!("Test database URL must contain 'test' for safety");
    }
}
