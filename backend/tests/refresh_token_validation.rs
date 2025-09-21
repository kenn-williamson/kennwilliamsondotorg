use chrono::{Duration, Utc};

mod test_helpers;

/// Test the complete refresh token flow to ensure refactor preserved functionality
#[actix_web::test]
async fn test_refresh_token_complete_flow() {
    let (srv, _pool, _test_container) = test_helpers::create_test_app_with_testcontainers().await;

    // Step 1: Register a new user
    let test_email = test_helpers::unique_test_email();
    let register_request_body = serde_json::json!({
        "email": test_email.clone(),
        "password": "password123",
        "display_name": "Refresh Test User"
    });

    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert_eq!(register_resp.status().as_u16(), 201);
    
    let register_data: serde_json::Value = register_resp.json().await.unwrap();
    let refresh_token = register_data["refresh_token"].as_str().unwrap();
    let jwt_token = register_data["token"].as_str().unwrap();
    
    // Verify we got both tokens
    assert!(!refresh_token.is_empty());
    assert!(!jwt_token.is_empty());
    assert_ne!(refresh_token, jwt_token);

    // Step 2: Use refresh token to get new JWT
    // Add delay to ensure different timestamps (JWT uses second precision)
    tokio::time::sleep(tokio::time::Duration::from_millis(1100)).await;
    
    let refresh_request_body = serde_json::json!({
        "refresh_token": refresh_token
    });

    let mut refresh_resp = srv.post("/backend/public/auth/refresh")
        .send_json(&refresh_request_body)
        .await
        .unwrap();
    
    assert_eq!(refresh_resp.status().as_u16(), 200);
    
    let refresh_data: serde_json::Value = refresh_resp.json().await.unwrap();
    let new_jwt_token = refresh_data["token"].as_str().unwrap();
    let new_refresh_token = refresh_data["refresh_token"].as_str().unwrap();
    
    // Verify we got new tokens
    assert!(!new_jwt_token.is_empty());
    assert!(!new_refresh_token.is_empty());
    assert_ne!(new_jwt_token, jwt_token); // Should be different JWT
    assert_ne!(new_refresh_token, refresh_token); // Should be different refresh token

    // Step 3: Verify new JWT works for authenticated request
    let mut me_resp = srv.get("/backend/protected/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", new_jwt_token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(me_resp.status().as_u16(), 200);
    
    let me_data: serde_json::Value = me_resp.json().await.unwrap();
    assert_eq!(me_data["email"], test_email);
    assert_eq!(me_data["display_name"], "Refresh Test User");

    // Step 4: Verify old refresh token is invalidated
    let old_refresh_request_body = serde_json::json!({
        "refresh_token": refresh_token
    });

    let mut old_refresh_resp = srv.post("/backend/public/auth/refresh")
        .send_json(&old_refresh_request_body)
        .await
        .unwrap();
    
    assert_eq!(old_refresh_resp.status().as_u16(), 401); // Should be unauthorized

    // Step 5: Test login flow still works
    let login_request_body = serde_json::json!({
        "email": test_email,
        "password": "password123"
    });

    let mut login_resp = srv.post("/backend/public/auth/login")
        .send_json(&login_request_body)
        .await
        .unwrap();
    
    assert_eq!(login_resp.status().as_u16(), 200);
    
    let login_data: serde_json::Value = login_resp.json().await.unwrap();
    assert!(login_data["refresh_token"].as_str().unwrap().len() > 0);
    assert!(login_data["token"].as_str().unwrap().len() > 0);
}

/// Test refresh token expiration handling
#[actix_web::test]
async fn test_refresh_token_expiration() {
    let (srv, pool, _test_container) = test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a user
    let user = test_helpers::create_test_user_in_db(
        &pool,
        &test_helpers::unique_test_email(),
        "$2b$04$test_hash",
        "Expiry Test User",
        &test_helpers::unique_test_slug()
    ).await
    .expect("Failed to create test user");

    // Create an expired refresh token directly in the database
    let expired_time = Utc::now() - Duration::days(8); // 8 days ago (expired)
    let token_hash = "expired_token_hash";
    
    test_helpers::create_test_refresh_token_in_db(
        &pool,
        user.id,
        token_hash,
        expired_time,
    ).await
    .expect("Failed to insert expired token");

    // Try to use expired token
    let refresh_request_body = serde_json::json!({
        "refresh_token": "expired_token_hash"
    });

    let mut refresh_resp = srv.post("/backend/public/auth/refresh")
        .send_json(&refresh_request_body)
        .await
        .unwrap();
    
    assert_eq!(refresh_resp.status().as_u16(), 401); // Should be unauthorized
}

/// Test refresh token with invalid token
#[actix_web::test]
async fn test_refresh_token_invalid() {
    let (srv, _pool, _test_container) = test_helpers::create_test_app_with_testcontainers().await;

    // Try to use non-existent token
    let refresh_request_body = serde_json::json!({
        "refresh_token": "nonexistent_token"
    });

    let mut refresh_resp = srv.post("/backend/public/auth/refresh")
        .send_json(&refresh_request_body)
        .await
        .unwrap();
    
    assert_eq!(refresh_resp.status().as_u16(), 401); // Should be unauthorized
}
