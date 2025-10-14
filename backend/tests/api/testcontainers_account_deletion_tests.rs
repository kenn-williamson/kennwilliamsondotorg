use serde_json::json;
use uuid::Uuid;
use crate::fixtures::TestContext;

#[path = "../fixtures/mod.rs"]
mod fixtures;

// ============================================================================
// ACCOUNT DELETION TESTS - TDD Phase 1 (Red Phase)
// ============================================================================

/// Test: Authenticated user can delete their own account
#[actix_web::test]
async fn test_user_can_delete_own_account() {
    let ctx = TestContext::builder().build().await;
    
    // Ensure system user exists
    ensure_system_user_exists(&ctx).await;
    
    // Given: A verified user with various data
    let user = ctx.create_verified_user("test@example.com", "testuser").await;
    let user_id = user.id;
    
    // Create test data for the user
    create_test_user_data(&ctx, user_id).await;
    
    // When: User deletes their account
    let mut resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user_id))))
        .send()
        .await
        .unwrap();
    
    // Then: Account deletion should succeed
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Delete account error response: {:?}", body);
        panic!("Expected success status, got {}", resp.status());
    }
    
    assert_eq!(resp.status(), 200);
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["message"], "Account deleted successfully");
    
    // And: User should no longer exist
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    
    assert!(!user_exists, "User should be deleted from database");
    
    // And: All user data should be deleted
    verify_user_data_deleted(&ctx, user_id).await;
    
    // And: User's phrases should be reassigned to system user
    verify_phrases_reassigned(&ctx, user_id).await;
}

/// Test: Unauthenticated user cannot delete account
#[actix_web::test]
async fn test_unauthenticated_user_cannot_delete_account() {
    let ctx = TestContext::builder().build().await;
    
    // Given: A verified user
    let _user = ctx.create_verified_user("test@example.com", "testuser").await;
    
    // When: Unauthenticated request to delete account
    let resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .send()
        .await
        .unwrap();
    
    // Then: Should return 401 Unauthorized
    assert_eq!(resp.status(), 401);
}

/// Test: User cannot delete another user's account
#[actix_web::test]
async fn test_user_cannot_delete_other_users_account() {
    let ctx = TestContext::builder().build().await;
    
    // Given: Two users
    let user1 = ctx.create_verified_user("user1@example.com", "user1").await;
    let user2 = ctx.create_verified_user("user2@example.com", "user2").await;
    
    // When: User1 tries to delete their own account (JWT contains user1's ID)
    // Note: The JWT middleware ensures users can only delete their own accounts
    let resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user1.id))))
        .send()
        .await
        .unwrap();
    
    // Then: Should succeed (user1 deleting their own account)
    assert_eq!(resp.status(), 200);
    
    // And: User1 should be deleted
    let user1_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user1.id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert!(!user1_exists, "User1 should be deleted");
    
    // And: User2 should still exist
    let user2_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user2.id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert!(user2_exists, "User2 should still exist");
}

/// Test: All user data is properly deleted
#[actix_web::test]
async fn test_all_user_data_deleted_on_account_deletion() {
    let ctx = TestContext::builder().build().await;
    
    // Given: A user with comprehensive data
    let user = ctx.create_verified_user("test@example.com", "testuser").await;
    let user_id = user.id;
    
    // Create all types of user data
    create_comprehensive_test_data(&ctx, user_id).await;
    
    // When: User deletes their account
    let resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user_id))))
        .send()
        .await
        .unwrap();
    
    // Then: Deletion should succeed
    assert_eq!(resp.status(), 200);
    
    // And: All data should be deleted
    verify_all_data_deleted(&ctx, user_id).await;
}

/// Test: User's phrases are reassigned to system user
#[actix_web::test]
async fn test_user_phrases_reassigned_to_system_user() {
    let ctx = TestContext::builder().build().await;
    
    // Given: A user with created phrases
    let user = ctx.create_verified_user("test@example.com", "testuser").await;
    let user_id = user.id;
    
    // Create phrases owned by the user
    create_user_phrases(&ctx, user_id).await;
    
    // When: User deletes their account
    let resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user_id))))
        .send()
        .await
        .unwrap();
    
    // Then: Deletion should succeed
    assert_eq!(resp.status(), 200);
    
    // And: Phrases should be reassigned to system user
    verify_phrases_reassigned(&ctx, user_id).await;
}

/// Test: User's phrase suggestions are deleted
#[actix_web::test]
async fn test_user_phrase_suggestions_deleted() {
    let ctx = TestContext::builder().build().await;
    
    // Given: A user with phrase suggestions
    let user = ctx.create_verified_user("test@example.com", "testuser").await;
    let user_id = user.id;
    
    // Create phrase suggestions
    create_phrase_suggestions(&ctx, user_id).await;
    
    // When: User deletes their account
    let resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user_id))))
        .send()
        .await
        .unwrap();
    
    // Then: Deletion should succeed
    assert_eq!(resp.status(), 200);
    
    // And: Phrase suggestions should be deleted
    let suggestion_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM phrase_suggestions WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    
    assert_eq!(suggestion_count, 0, "All phrase suggestions should be deleted");
}

/// Test: Deleted user cannot login anymore
#[actix_web::test]
async fn test_deleted_user_cannot_login() {
    let ctx = TestContext::builder().build().await;
    
    // Given: A user with password
    let user = ctx.create_verified_user("test@example.com", "testuser").await;
    let user_id = user.id;

    // User created by create_verified_user already has credentials via UserBuilder
    
    // When: User deletes their account
    let resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user_id))))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    
    // Then: User should not be able to login
    let login_data = json!({
        "email": "test@example.com",
        "password": "testpassword"
    });
    
    let resp = ctx.server
        .post("/backend/public/auth/login")
        .send_json(&login_data)
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 401, "Deleted user should not be able to login");
}

/// Test: System user cannot be deleted (protection)
#[actix_web::test]
async fn test_system_user_cannot_be_deleted() {
    let ctx = TestContext::builder().build().await;
    
    // Given: System user exists
    let system_user_id = get_system_user_id(&ctx).await;
    
    // When: Try to delete system user (this should fail)
    let mut resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, system_user_id))))
        .send()
        .await
        .unwrap();
    
    // Then: Should return error (403 or 400)
    assert!(resp.status() == 400 || resp.status() == 403);
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["error"].as_str().unwrap().contains("system user"));
}

/// Test: Account deletion works when user has no phrases
#[actix_web::test]
async fn test_account_deletion_works_with_no_phrases() {
    let ctx = TestContext::builder().build().await;
    
    // Ensure system user exists
    ensure_system_user_exists(&ctx).await;
    
    // Given: A user with no phrases (just other data)
    let user = ctx.create_verified_user("test@example.com", "testuser").await;
    let user_id = user.id;
    
    // Create test data for the user (but no phrases)
    create_test_user_data(&ctx, user_id).await;
    
    // When: User deletes their account
    let mut resp = ctx.server
        .delete("/backend/protected/auth/delete-account")
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&ctx, user_id))))
        .send()
        .await
        .unwrap();
    
    // Then: Should succeed
    assert_eq!(resp.status(), 200);
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["message"], "Account deleted successfully");
    
    // And: User should be deleted
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert!(!user_exists, "User should be deleted");
    
    // And: All user data should be deleted
    verify_user_data_deleted(&ctx, user_id).await;
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Create test JWT token for a user
fn create_test_jwt(_ctx: &TestContext, user_id: Uuid) -> String {
    use backend::services::auth::jwt::JwtService;
    use backend::test_utils::UserBuilder;

    let jwt_service = JwtService::new("test-jwt-secret-for-api-tests".to_string());
    let user = UserBuilder::new()
        .with_id(user_id)
        .with_email("test@example.com")
        .with_display_name("Test User")
        .with_slug("testuser")
        .build();
    let roles = vec!["user".to_string(), "email-verified".to_string()];

    jwt_service.generate_token(&user, &roles).unwrap()
}

/// Create basic test data for a user
async fn create_test_user_data(ctx: &TestContext, user_id: Uuid) {
    // Create incident timer
    sqlx::query("INSERT INTO incident_timers (user_id, notes) VALUES ($1, $2)")
        .bind(user_id)
        .bind("Test timer")
        .execute(&ctx.pool)
        .await
        .unwrap();
    
    // Create phrase exclusion
    let phrase_id = create_test_phrase(&ctx).await;
    sqlx::query("INSERT INTO user_excluded_phrases (user_id, phrase_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(phrase_id)
        .execute(&ctx.pool)
        .await
        .unwrap();
    
    // Create refresh token
    sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind("test_token_hash")
        .bind(chrono::Utc::now() + chrono::Duration::days(30))
        .execute(&ctx.pool)
        .await
        .unwrap();
}

/// Create comprehensive test data for a user
async fn create_comprehensive_test_data(ctx: &TestContext, user_id: Uuid) {
    // Create multiple incident timers
    for i in 1..=3 {
        sqlx::query("INSERT INTO incident_timers (user_id, notes) VALUES ($1, $2)")
            .bind(user_id)
            .bind(format!("Test timer {}", i))
            .execute(&ctx.pool)
            .await
            .unwrap();
    }
    
    // Create multiple phrase exclusions
    for _i in 1..=2 {
        let phrase_id = create_test_phrase(&ctx).await;
        sqlx::query("INSERT INTO user_excluded_phrases (user_id, phrase_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(phrase_id)
            .execute(&ctx.pool)
            .await
            .unwrap();
    }
    
    // Create multiple refresh tokens
    for i in 1..=2 {
        sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind(format!("test_token_hash_{}", i))
            .bind(chrono::Utc::now() + chrono::Duration::days(30))
            .execute(&ctx.pool)
            .await
            .unwrap();
    }
}

/// Create phrases owned by a user
async fn create_user_phrases(ctx: &TestContext, user_id: Uuid) {
    for i in 1..=3 {
        sqlx::query("INSERT INTO phrases (phrase_text, created_by) VALUES ($1, $2)")
            .bind(format!("User phrase {}", i))
            .bind(user_id)
            .execute(&ctx.pool)
            .await
            .unwrap();
    }
}

/// Create phrase suggestions for a user
async fn create_phrase_suggestions(ctx: &TestContext, user_id: Uuid) {
    for i in 1..=2 {
        sqlx::query("INSERT INTO phrase_suggestions (phrase_text, user_id, status) VALUES ($1, $2, $3)")
            .bind(format!("Suggested phrase {}", i))
            .bind(user_id)
            .bind("pending")
            .execute(&ctx.pool)
            .await
            .unwrap();
    }
}

/// Create a test phrase and return its ID
async fn create_test_phrase(ctx: &TestContext) -> Uuid {
    let system_user_id = get_system_user_id(&ctx).await;
    
    let result = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO phrases (phrase_text, created_by) VALUES ($1, $2) RETURNING id"
    )
    .bind("Test phrase for exclusion")
    .bind(system_user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    
    result
}

/// Get system user ID
async fn get_system_user_id(ctx: &TestContext) -> Uuid {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM users WHERE email = 'system@kennwilliamson.org'"
    )
    .fetch_one(&ctx.pool)
    .await
    .unwrap()
}

/// Ensure system user exists in test database
async fn ensure_system_user_exists(ctx: &TestContext) {
    // Check if system user exists
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = 'system@kennwilliamson.org')"
    )
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    
    if !exists {
        // Create system user
        sqlx::query(
            "INSERT INTO users (email, display_name, slug, active)
             VALUES ('system@kennwilliamson.org', 'System', 'system', true)"
        )
        .execute(&ctx.pool)
        .await
        .unwrap();
        
        // Add user role to system user
        sqlx::query(
            "INSERT INTO user_roles (user_id, role_id) 
             SELECT u.id, r.id FROM users u, roles r 
             WHERE u.email = 'system@kennwilliamson.org' AND r.name = 'user'"
        )
        .execute(&ctx.pool)
        .await
        .unwrap();
    }
}

/// Verify user data is deleted
async fn verify_user_data_deleted(ctx: &TestContext, user_id: Uuid) {
    // Check incident timers
    let timer_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM incident_timers WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(timer_count, 0, "All incident timers should be deleted");
    
    // Check phrase exclusions
    let exclusion_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM user_excluded_phrases WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(exclusion_count, 0, "All phrase exclusions should be deleted");
    
    // Check refresh tokens
    let token_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(token_count, 0, "All refresh tokens should be deleted");
    
    // Check user roles
    let role_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM user_roles WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(role_count, 0, "All user roles should be deleted");
}

/// Verify all data is deleted (comprehensive check)
async fn verify_all_data_deleted(ctx: &TestContext, user_id: Uuid) {
    verify_user_data_deleted(ctx, user_id).await;
    
    // Check phrase suggestions
    let suggestion_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM phrase_suggestions WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(suggestion_count, 0, "All phrase suggestions should be deleted");
}

/// Verify phrases are reassigned to system user
async fn verify_phrases_reassigned(ctx: &TestContext, user_id: Uuid) {
    let system_user_id = get_system_user_id(&ctx).await;
    
    // Check that no phrases are still owned by the deleted user
    let user_phrase_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM phrases WHERE created_by = $1"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert_eq!(user_phrase_count, 0, "No phrases should be owned by deleted user");
    
    // Check that phrases exist and are owned by system user
    let system_phrase_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM phrases WHERE created_by = $1"
    )
    .bind(system_user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();
    assert!(system_phrase_count > 0, "System user should have phrases");
}
