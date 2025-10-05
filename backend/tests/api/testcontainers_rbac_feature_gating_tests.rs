use serde_json::json;
use sqlx::PgPool;
use crate::test_helpers::TestContext;

/// Test: Login includes roles in JWT claims
#[actix_web::test]
async fn test_login_includes_roles_in_jwt() {
    let ctx = TestContext::builder().build().await;

    // Register a user
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let register_body = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut register_resp = ctx.server
        .post("/backend/public/auth/register")
        .send_json(&register_body)
        .await
        .unwrap();
    assert!(register_resp.status().is_success());

    let register_result: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_result["user"]["id"].as_str().unwrap();

    // Manually assign email-verified role (simulates email verification)
    assign_email_verified_role(&ctx.pool, user_id).await;

    // Login
    let login_body = json!({
        "email": email,
        "password": password
    });

    let mut login_resp = ctx.server
        .post("/backend/public/auth/login")
        .send_json(&login_body)
        .await
        .unwrap();
    assert!(login_resp.status().is_success());

    let login_result: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_result["token"].as_str().unwrap();

    // Verify JWT contains roles using production code path
    let jwt_secret = "test-jwt-secret-for-api-tests".to_string(); // Must match test_helpers.rs:155
    let jwt_service = backend::services::auth::jwt::JwtService::new(jwt_secret);
    let claims = jwt_service.verify_token(token).await.unwrap().unwrap();

    assert!(
        !claims.roles.is_empty(),
        "JWT should contain roles"
    );
    assert!(
        claims.roles.contains(&"email-verified".to_string()),
        "JWT should contain email-verified role"
    );
}

/// Test: Token refresh includes updated roles
#[actix_web::test]
async fn test_token_refresh_includes_updated_roles() {
    let ctx = TestContext::builder().build().await;

    // Register and login
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let register_body = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut register_resp = ctx.server
        .post("/backend/public/auth/register")
        .send_json(&register_body)
        .await
        .unwrap();
    assert!(register_resp.status().is_success());

    let register_result: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_result["user"]["id"].as_str().unwrap();
    let initial_token = register_result["token"].as_str().unwrap();
    let refresh_token = register_result["refresh_token"].as_str().unwrap();

    // Verify initial token has no email-verified role using production code path
    let jwt_secret = "test-jwt-secret-for-api-tests".to_string(); // Must match test_helpers.rs:155
    let jwt_service = backend::services::auth::jwt::JwtService::new(jwt_secret);
    let initial_claims = jwt_service.verify_token(initial_token).await.unwrap().unwrap();

    assert!(
        !initial_claims.roles.contains(&"email-verified".to_string()),
        "Initial token should not have email-verified role"
    );

    // Admin assigns email-verified role
    assign_email_verified_role(&ctx.pool, user_id).await;

    // Refresh token
    let refresh_body = json!({
        "refresh_token": refresh_token
    });

    let mut refresh_resp = ctx.server
        .post("/backend/public/auth/refresh")
        .send_json(&refresh_body)
        .await
        .unwrap();
    assert!(refresh_resp.status().is_success());

    let refresh_result: serde_json::Value = refresh_resp.json().await.unwrap();
    let new_token = refresh_result["token"].as_str().unwrap();

    // Verify refreshed token includes updated role using production code path
    let jwt_secret = "test-jwt-secret-for-api-tests".to_string(); // Must match test_helpers.rs:155
    let jwt_service = backend::services::auth::jwt::JwtService::new(jwt_secret);
    let new_claims = jwt_service.verify_token(new_token).await.unwrap().unwrap();

    assert!(
        new_claims.roles.contains(&"email-verified".to_string()),
        "Refreshed token should include email-verified role"
    );
}

/// Test: Unverified user blocked from creating timer
#[actix_web::test]
async fn test_unverified_user_blocked_from_creating_timer() {
    let ctx = TestContext::builder().build().await;

    // Register user (no email-verified role)
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let register_body = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut register_resp = ctx.server
        .post("/backend/public/auth/register")
        .send_json(&register_body)
        .await
        .unwrap();
    assert!(register_resp.status().is_success());

    let register_result: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_result["token"].as_str().unwrap();

    // Attempt to create timer
    let timer_body = json!({
        "reset_timestamp": "2025-01-01T00:00:00Z",
        "notes": "Test timer"
    });

    let timer_resp = ctx.server
        .post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_body)
        .await
        .unwrap();

    // Assert 403 Forbidden - email verification required
    assert_eq!(timer_resp.status(), 403);
}

/// Test: Verified user can create timer
#[actix_web::test]
async fn test_verified_user_can_create_timer() {
    let ctx = TestContext::builder().build().await;

    // Register user
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let register_body = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut register_resp = ctx.server
        .post("/backend/public/auth/register")
        .send_json(&register_body)
        .await
        .unwrap();
    assert!(register_resp.status().is_success());

    let register_result: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_result["user"]["id"].as_str().unwrap();

    // Manually assign email-verified role
    assign_email_verified_role(&ctx.pool, user_id).await;

    // Login to get token with updated roles
    let login_body = json!({
        "email": email,
        "password": password
    });

    let mut login_resp = ctx.server
        .post("/backend/public/auth/login")
        .send_json(&login_body)
        .await
        .unwrap();
    assert!(login_resp.status().is_success());

    let login_result: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_result["token"].as_str().unwrap();

    // Create timer
    let timer_body = json!({
        "reset_timestamp": "2025-01-01T00:00:00Z",
        "notes": "Test timer"
    });

    let mut timer_resp = ctx.server
        .post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_body)
        .await
        .unwrap();

    // Assert 201 Created
    assert_eq!(timer_resp.status(), 201);

    let timer_result: serde_json::Value = timer_resp.json().await.unwrap();
    assert!(timer_result["id"].is_string());
    assert_eq!(timer_result["notes"].as_str().unwrap(), "Test timer");
}

/// Test: Unverified user blocked from submitting phrase suggestion
#[actix_web::test]
async fn test_unverified_user_blocked_from_phrase_suggestion() {
    let ctx = TestContext::builder().build().await;

    // Register user (no email-verified role)
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let register_body = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut register_resp = ctx.server
        .post("/backend/public/auth/register")
        .send_json(&register_body)
        .await
        .unwrap();
    assert!(register_resp.status().is_success());

    let register_result: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_result["token"].as_str().unwrap();

    // Attempt to submit phrase suggestion
    let suggestion_body = json!({
        "phrase_text": "Test phrase suggestion"
    });

    let suggestion_resp = ctx.server
        .post("/backend/protected/phrases/suggestions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&suggestion_body)
        .await
        .unwrap();

    // Assert 403 Forbidden - email verification required
    assert_eq!(suggestion_resp.status(), 403);
}

/// Test: Verified user can submit phrase suggestion
#[actix_web::test]
async fn test_verified_user_can_submit_phrase_suggestion() {
    let ctx = TestContext::builder().build().await;

    // Register user
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let register_body = json!({
        "email": email,
        "password": password,
        "display_name": "Test User"
    });

    let mut register_resp = ctx.server
        .post("/backend/public/auth/register")
        .send_json(&register_body)
        .await
        .unwrap();
    assert!(register_resp.status().is_success());

    let register_result: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_result["user"]["id"].as_str().unwrap();

    // Manually assign email-verified role
    assign_email_verified_role(&ctx.pool, user_id).await;

    // Login to get token with updated roles
    let login_body = json!({
        "email": email,
        "password": password
    });

    let mut login_resp = ctx.server
        .post("/backend/public/auth/login")
        .send_json(&login_body)
        .await
        .unwrap();
    assert!(login_resp.status().is_success());

    let login_result: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_result["token"].as_str().unwrap();

    // Submit phrase suggestion
    let suggestion_body = json!({
        "phrase_text": "Test phrase suggestion"
    });

    let mut suggestion_resp = ctx.server
        .post("/backend/protected/phrases/suggestions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&suggestion_body)
        .await
        .unwrap();

    // Assert 201 Created
    assert_eq!(suggestion_resp.status(), 201);

    let suggestion_result: serde_json::Value = suggestion_resp.json().await.unwrap();
    assert!(suggestion_result["id"].is_string());
    assert_eq!(
        suggestion_result["phrase_text"].as_str().unwrap(),
        "Test phrase suggestion"
    );
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Manually assign email-verified role to user (simulates email verification or admin action)
async fn assign_email_verified_role(pool: &PgPool, user_id: &str) {
    // Get email-verified role ID
    let role_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM roles WHERE name = 'email-verified'"
    )
    .fetch_one(pool)
    .await
    .expect("Failed to get email-verified role ID");

    let user_uuid = uuid::Uuid::parse_str(user_id).expect("Invalid user ID");

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

