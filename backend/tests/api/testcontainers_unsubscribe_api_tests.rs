use crate::fixtures::TestContext;
use backend::models::db::unsubscribe_token::email_types;
use backend::repositories::postgres::postgres_unsubscribe_token_repository::PostgresUnsubscribeTokenRepository;
use backend::repositories::traits::unsubscribe_token_repository::UnsubscribeTokenRepository;
use serde_json::json;
use sha2::{Digest, Sha256};

/// Hash a raw token (same as the service does)
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

// ============================================================================
// POST /backend/public/email/unsubscribe - Perform unsubscribe
// ============================================================================

#[actix_web::test]
async fn test_unsubscribe_success() {
    let ctx = TestContext::builder().build().await;

    // Create a user
    let user = ctx
        .create_verified_user(
            &crate::fixtures::unique_test_email(),
            &format!("test-user-{}", uuid::Uuid::new_v4()),
        )
        .await;

    // Create an unsubscribe token in the database
    let raw_token = "a".repeat(64);
    let token_hash = hash_token(&raw_token);
    let unsub_repo = PostgresUnsubscribeTokenRepository::new(ctx.pool.clone());
    unsub_repo
        .create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash)
        .await
        .expect("Failed to create unsubscribe token");

    // Call the unsubscribe endpoint
    let request_body = json!({
        "token": raw_token
    });

    let mut resp = ctx
        .server
        .post("/backend/public/email/unsubscribe")
        .send_json(&request_body)
        .await
        .unwrap();

    println!("Unsubscribe response status: {}", resp.status());
    assert!(resp.status().is_success());

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("success").unwrap(), true);
    assert_eq!(
        body.get("email_type").unwrap(),
        email_types::BLOG_NOTIFICATIONS
    );
    assert!(
        body.get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("Successfully unsubscribed")
    );

    // Verify the user's preference was updated
    let prefs = sqlx::query_scalar::<_, bool>(
        "SELECT notify_blog_posts FROM user_preferences WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_one(&ctx.pool)
    .await
    .expect("Failed to fetch preferences");

    assert!(
        !prefs,
        "notify_blog_posts should be false after unsubscribe"
    );

    // Verify the token was deleted (can't be reused)
    let token_exists = unsub_repo.find_by_token_hash(&token_hash).await.unwrap();
    assert!(token_exists.is_none(), "Token should be deleted after use");
}

#[actix_web::test]
async fn test_unsubscribe_invalid_token_format() {
    let ctx = TestContext::builder().build().await;

    // Try with a token that's too short
    let request_body = json!({
        "token": "abc123"
    });

    let mut resp = ctx
        .server
        .post("/backend/public/email/unsubscribe")
        .send_json(&request_body)
        .await
        .unwrap();

    println!("Invalid token response status: {}", resp.status());
    assert_eq!(resp.status(), 400); // Bad Request

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("success").unwrap(), false);
    assert!(
        body.get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("Invalid")
    );
}

#[actix_web::test]
async fn test_unsubscribe_token_not_found() {
    let ctx = TestContext::builder().build().await;

    // Use a valid-format token that doesn't exist in the database
    let request_body = json!({
        "token": "b".repeat(64)
    });

    let mut resp = ctx
        .server
        .post("/backend/public/email/unsubscribe")
        .send_json(&request_body)
        .await
        .unwrap();

    println!("Token not found response status: {}", resp.status());
    assert_eq!(resp.status(), 404); // Not Found

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("success").unwrap(), false);
    assert!(
        body.get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("not found")
    );
}

// ============================================================================
// GET /backend/public/email/unsubscribe/{token} - Validate token
// ============================================================================

#[actix_web::test]
async fn test_validate_token_success() {
    let ctx = TestContext::builder().build().await;

    // Create a user
    let user = ctx
        .create_verified_user(
            &crate::fixtures::unique_test_email(),
            &format!("test-user-{}", uuid::Uuid::new_v4()),
        )
        .await;

    // Create an unsubscribe token
    let raw_token = "c".repeat(64);
    let token_hash = hash_token(&raw_token);
    let unsub_repo = PostgresUnsubscribeTokenRepository::new(ctx.pool.clone());
    unsub_repo
        .create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash)
        .await
        .expect("Failed to create unsubscribe token");

    // Validate the token
    let mut resp = ctx
        .server
        .get(format!("/backend/public/email/unsubscribe/{}", raw_token))
        .send()
        .await
        .unwrap();

    println!("Validate token response status: {}", resp.status());
    assert!(resp.status().is_success());

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("valid").unwrap(), true);
    assert_eq!(
        body.get("email_type").unwrap(),
        email_types::BLOG_NOTIFICATIONS
    );
}

#[actix_web::test]
async fn test_validate_token_not_found() {
    let ctx = TestContext::builder().build().await;

    // Try to validate a non-existent token
    let raw_token = "d".repeat(64);

    let mut resp = ctx
        .server
        .get(format!("/backend/public/email/unsubscribe/{}", raw_token))
        .send()
        .await
        .unwrap();

    println!("Token not found response status: {}", resp.status());
    assert_eq!(resp.status(), 404);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("valid").unwrap(), false);
    assert!(
        body.get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("not found")
    );
}

#[actix_web::test]
async fn test_validate_token_invalid_format() {
    let ctx = TestContext::builder().build().await;

    // Try to validate a malformed token
    let mut resp = ctx
        .server
        .get("/backend/public/email/unsubscribe/invalid-token")
        .send()
        .await
        .unwrap();

    println!("Invalid format response status: {}", resp.status());
    assert_eq!(resp.status(), 400);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("valid").unwrap(), false);
    assert!(
        body.get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("Invalid")
    );
}

// ============================================================================
// Token cannot be reused tests
// ============================================================================

#[actix_web::test]
async fn test_unsubscribe_token_cannot_be_reused() {
    let ctx = TestContext::builder().build().await;

    // Create a user
    let user = ctx
        .create_verified_user(
            &crate::fixtures::unique_test_email(),
            &format!("test-user-{}", uuid::Uuid::new_v4()),
        )
        .await;

    // Create an unsubscribe token
    let raw_token = "e".repeat(64);
    let token_hash = hash_token(&raw_token);
    let unsub_repo = PostgresUnsubscribeTokenRepository::new(ctx.pool.clone());
    unsub_repo
        .create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash)
        .await
        .expect("Failed to create unsubscribe token");

    // First unsubscribe should succeed
    let request_body = json!({
        "token": raw_token
    });

    let resp = ctx
        .server
        .post("/backend/public/email/unsubscribe")
        .send_json(&request_body)
        .await
        .unwrap();

    assert!(resp.status().is_success());

    // Second attempt should fail (token was deleted)
    let resp = ctx
        .server
        .post("/backend/public/email/unsubscribe")
        .send_json(&request_body)
        .await
        .unwrap();

    assert_eq!(resp.status(), 404, "Token should not be reusable");
}
