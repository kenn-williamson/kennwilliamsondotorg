use chrono::{Duration, Utc};

mod fixtures;
use fixtures::TestContext;

/// Test cleanup of expired refresh tokens
#[actix_web::test]
async fn test_cleanup_expired_refresh_tokens() {
    use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
    use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
    use backend::services::cleanup::CleanupService;

    let test_container = fixtures::TestContainer::builder().build()
        .await
        .expect("Failed to create test container");
    let pool = &test_container.pool;

    // Create a test user
    let user = fixtures::UserBuilder::new()
        .with_email(&fixtures::unique_test_email())
        .with_display_name("Cleanup Test User")
        .with_slug(&fixtures::unique_test_slug())
        .with_password("password123")
        .persist(pool)
        .await
        .expect("Failed to create test user");

    // Create expired refresh token
    let expired_time = Utc::now() - Duration::days(8);
    fixtures::create_test_refresh_token_in_db(pool, user.id, "expired_token_1", expired_time)
        .await
        .expect("Failed to create expired token");

    // Create valid refresh token
    let valid_time = Utc::now() + Duration::days(7);
    fixtures::create_test_refresh_token_in_db(pool, user.id, "valid_token_1", valid_time)
        .await
        .expect("Failed to create valid token");

    // Create cleanup service
    let refresh_repo = Box::new(PostgresRefreshTokenRepository::new(pool.clone()));
    let verification_repo = Box::new(PostgresVerificationTokenRepository::new(pool.clone()));
    let password_reset_repo = Box::new(backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository::new(pool.clone()));
    let cleanup_service = CleanupService::new(refresh_repo, verification_repo, password_reset_repo);

    // Run cleanup
    let deleted_count = cleanup_service
        .cleanup_expired_tokens()
        .await
        .expect("Cleanup failed");

    // Verify only expired token was deleted
    assert_eq!(deleted_count, 1);

    // Verify the valid token still exists
    let remaining_tokens: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(pool)
        .await
        .expect("Failed to count tokens");

    assert_eq!(remaining_tokens, 1);
}

/// Test cleanup of expired verification tokens
#[actix_web::test]
async fn test_cleanup_expired_verification_tokens() {
    use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
    use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
    use backend::services::cleanup::CleanupService;

    let test_container = fixtures::TestContainer::builder().build()
        .await
        .expect("Failed to create test container");
    let pool = &test_container.pool;

    // Create a test user
    let user = fixtures::UserBuilder::new()
        .with_email(&fixtures::unique_test_email())
        .with_display_name("Verification Cleanup Test User")
        .with_slug(&fixtures::unique_test_slug())
        .with_password("password123")
        .persist(pool)
        .await
        .expect("Failed to create test user");

    // Create expired verification token
    let expired_time = Utc::now() - Duration::hours(25); // Expired (assuming 24h validity)
    sqlx::query(
        "INSERT INTO verification_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind("expired_verification_token")
    .bind(expired_time)
    .execute(pool)
    .await
    .expect("Failed to create expired verification token");

    // Create valid verification token
    let valid_time = Utc::now() + Duration::hours(23);
    sqlx::query(
        "INSERT INTO verification_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind("valid_verification_token")
    .bind(valid_time)
    .execute(pool)
    .await
    .expect("Failed to create valid verification token");

    // Create cleanup service
    let refresh_repo = Box::new(PostgresRefreshTokenRepository::new(pool.clone()));
    let verification_repo = Box::new(PostgresVerificationTokenRepository::new(pool.clone()));
    let password_reset_repo = Box::new(backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository::new(pool.clone()));
    let cleanup_service = CleanupService::new(refresh_repo, verification_repo, password_reset_repo);

    // Run cleanup
    let deleted_count = cleanup_service
        .cleanup_expired_tokens()
        .await
        .expect("Cleanup failed");

    // Verify only expired token was deleted
    assert_eq!(deleted_count, 1);

    // Verify the valid token still exists
    let remaining_tokens: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM verification_tokens WHERE user_id = $1")
            .bind(user.id)
            .fetch_one(pool)
            .await
            .expect("Failed to count tokens");

    assert_eq!(remaining_tokens, 1);
}

/// Test cleanup with both expired refresh and verification tokens
#[actix_web::test]
async fn test_cleanup_both_token_types() {
    use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
    use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
    use backend::services::cleanup::CleanupService;

    let test_container = fixtures::TestContainer::builder().build()
        .await
        .expect("Failed to create test container");
    let pool = &test_container.pool;

    // Create a test user
    let user = fixtures::UserBuilder::new()
        .with_email(&fixtures::unique_test_email())
        .with_display_name("Both Tokens Cleanup Test User")
        .with_slug(&fixtures::unique_test_slug())
        .with_password("password123")
        .persist(pool)
        .await
        .expect("Failed to create test user");

    // Create 3 expired refresh tokens
    let expired_time = Utc::now() - Duration::days(8);
    for i in 1..=3 {
        fixtures::create_test_refresh_token_in_db(
            pool,
            user.id,
            &format!("expired_refresh_{}", i),
            expired_time,
        )
        .await
        .expect("Failed to create expired refresh token");
    }

    // Create 2 expired verification tokens
    let expired_verification_time = Utc::now() - Duration::hours(25);
    for i in 1..=2 {
        sqlx::query(
            "INSERT INTO verification_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        )
        .bind(user.id)
        .bind(format!("expired_verification_{}", i))
        .bind(expired_verification_time)
        .execute(pool)
        .await
        .expect("Failed to create expired verification token");
    }

    // Create 1 valid refresh token
    let valid_refresh_time = Utc::now() + Duration::days(7);
    fixtures::create_test_refresh_token_in_db(
        pool,
        user.id,
        "valid_refresh",
        valid_refresh_time,
    )
    .await
    .expect("Failed to create valid refresh token");

    // Create 1 valid verification token
    let valid_verification_time = Utc::now() + Duration::hours(23);
    sqlx::query(
        "INSERT INTO verification_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind("valid_verification")
    .bind(valid_verification_time)
    .execute(pool)
    .await
    .expect("Failed to create valid verification token");

    // Create cleanup service
    let refresh_repo = Box::new(PostgresRefreshTokenRepository::new(pool.clone()));
    let verification_repo = Box::new(PostgresVerificationTokenRepository::new(pool.clone()));
    let password_reset_repo = Box::new(backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository::new(pool.clone()));
    let cleanup_service = CleanupService::new(refresh_repo, verification_repo, password_reset_repo);

    // Run cleanup
    let deleted_count = cleanup_service
        .cleanup_expired_tokens()
        .await
        .expect("Cleanup failed");

    // Verify 5 expired tokens were deleted (3 refresh + 2 verification)
    assert_eq!(deleted_count, 5);

    // Verify valid tokens still exist
    let remaining_refresh: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1")
            .bind(user.id)
            .fetch_one(pool)
            .await
            .expect("Failed to count refresh tokens");

    let remaining_verification: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM verification_tokens WHERE user_id = $1")
            .bind(user.id)
            .fetch_one(pool)
            .await
            .expect("Failed to count verification tokens");

    assert_eq!(remaining_refresh, 1);
    assert_eq!(remaining_verification, 1);
}

/// Test cleanup with no expired tokens
#[actix_web::test]
async fn test_cleanup_no_expired_tokens() {
    use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
    use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
    use backend::services::cleanup::CleanupService;

    let test_container = fixtures::TestContainer::builder().build()
        .await
        .expect("Failed to create test container");
    let pool = &test_container.pool;

    // Create a test user
    let user = fixtures::UserBuilder::new()
        .with_email(&fixtures::unique_test_email())
        .with_display_name("No Expired Tokens Test User")
        .with_slug(&fixtures::unique_test_slug())
        .with_password("password123")
        .persist(pool)
        .await
        .expect("Failed to create test user");

    // Create only valid tokens
    let valid_time = Utc::now() + Duration::days(7);
    fixtures::create_test_refresh_token_in_db(pool, user.id, "valid_token", valid_time)
        .await
        .expect("Failed to create valid token");

    // Create cleanup service
    let refresh_repo = Box::new(PostgresRefreshTokenRepository::new(pool.clone()));
    let verification_repo = Box::new(PostgresVerificationTokenRepository::new(pool.clone()));
    let password_reset_repo = Box::new(backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository::new(pool.clone()));
    let cleanup_service = CleanupService::new(refresh_repo, verification_repo, password_reset_repo);

    // Run cleanup
    let deleted_count = cleanup_service
        .cleanup_expired_tokens()
        .await
        .expect("Cleanup failed");

    // Verify no tokens were deleted
    assert_eq!(deleted_count, 0);

    // Verify the valid token still exists
    let remaining_tokens: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1")
            .bind(user.id)
            .fetch_one(pool)
            .await
            .expect("Failed to count tokens");

    assert_eq!(remaining_tokens, 1);
}

/// Test cleanup with empty database
#[actix_web::test]
async fn test_cleanup_empty_database() {
    use backend::repositories::postgres::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
    use backend::repositories::postgres::postgres_verification_token_repository::PostgresVerificationTokenRepository;
    use backend::services::cleanup::CleanupService;

    let test_container = fixtures::TestContainer::builder().build()
        .await
        .expect("Failed to create test container");
    let pool = &test_container.pool;

    // Create cleanup service
    let refresh_repo = Box::new(PostgresRefreshTokenRepository::new(pool.clone()));
    let verification_repo = Box::new(PostgresVerificationTokenRepository::new(pool.clone()));
    let password_reset_repo = Box::new(backend::repositories::postgres::postgres_password_reset_token_repository::PostgresPasswordResetTokenRepository::new(pool.clone()));
    let cleanup_service = CleanupService::new(refresh_repo, verification_repo, password_reset_repo);

    // Run cleanup on empty database
    let deleted_count = cleanup_service
        .cleanup_expired_tokens()
        .await
        .expect("Cleanup failed");

    // Verify no tokens were deleted
    assert_eq!(deleted_count, 0);
}
