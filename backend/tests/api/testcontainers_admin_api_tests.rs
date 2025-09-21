use serde_json::json;
use uuid::Uuid;

// Use consolidated test helpers from test_helpers module

// ============================================================================
// AUTHENTICATION AND AUTHORIZATION TESTS
// ============================================================================

#[actix_web::test]
async fn test_admin_endpoints_require_authentication() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Test stats endpoint without auth
    let mut resp = srv.get("/backend/protected/admin/stats")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401); // Unauthorized
    
    // Test users endpoint without auth
    let mut resp = srv.get("/backend/protected/admin/users")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_admin_endpoints_require_admin_role() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a regular user (not admin)
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Regular User";
    
    let register_request_body = json!({
        "email": email,
        "password": password,
        "display_name": display_name
    });
    
    let mut register_resp = srv.post("/backend/public/auth/register")
        .send_json(&register_request_body)
        .await
        .unwrap();
    
    assert!(register_resp.status().is_success());
    
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let token = register_body.get("token").unwrap().as_str().unwrap();
    
    // Try to access admin endpoint with regular user token
    let mut resp = srv.get("/backend/protected/admin/stats")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 403); // Forbidden - not admin
}

// ============================================================================
// SYSTEM STATISTICS TESTS
// ============================================================================

#[actix_web::test]
async fn test_get_system_stats_success() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.get("/backend/protected/admin/stats")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get stats response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get stats error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("total_users").is_some());
    assert!(body.get("active_users").is_some());
    assert!(body.get("pending_suggestions").is_some());
    assert!(body.get("total_phrases").is_some());
}

// ============================================================================
// USER MANAGEMENT TESTS
// ============================================================================

#[actix_web::test]
async fn test_get_users_success() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.get("/backend/protected/admin/users")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get users response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get users error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("users").is_some());
    assert!(body.get("total").is_some());
    
    let users = body.get("users").unwrap().as_array().unwrap();
    assert!(users.len() >= 1); // At least the admin user
}

#[actix_web::test]
async fn test_get_users_with_search() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Test with search parameter
    let mut resp = srv.get("/backend/protected/admin/users?search=Admin")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("users").is_some());
    assert!(body.get("total").is_some());
}

#[actix_web::test]
async fn test_deactivate_user_success() {
    let (srv, pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Create a regular user to deactivate
    let regular_user = crate::test_helpers::create_test_user_in_db(
        &pool,
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Regular User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.post(&format!("/backend/protected/admin/users/{}/deactivate", regular_user.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Deactivate user response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Deactivate user error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "User deactivated successfully");
}

#[actix_web::test]
async fn test_activate_user_success() {
    let (srv, pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Create a regular user to activate
    let regular_user = crate::test_helpers::create_test_user_in_db(
        &pool,
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Regular User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.post(&format!("/backend/protected/admin/users/{}/activate", regular_user.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Activate user response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Activate user error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "User activated successfully");
}

#[actix_web::test]
async fn test_reset_user_password_success() {
    let (srv, pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Create a regular user to reset password
    let regular_user = crate::test_helpers::create_test_user_in_db(
        &pool,
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Regular User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.post(&format!("/backend/protected/admin/users/{}/reset-password", regular_user.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Reset password response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Reset password error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("new_password").is_some());
}

#[actix_web::test]
async fn test_promote_user_to_admin_success() {
    let (srv, pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Create a regular user to promote
    let regular_user = crate::test_helpers::create_test_user_in_db(
        &pool,
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Regular User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.post(&format!("/backend/protected/admin/users/{}/promote", regular_user.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Promote user response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Promote user error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "User promoted to admin successfully");
}

// ============================================================================
// PHRASE MANAGEMENT TESTS
// ============================================================================

#[actix_web::test]
async fn test_get_all_phrases_success() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.get("/backend/protected/admin/phrases")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get all phrases response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get all phrases error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("phrases").is_some());
    assert!(body.get("total").is_some());
}

#[actix_web::test]
async fn test_create_phrase_success() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let request_body = json!({
        "phrase_text": crate::test_helpers::unique_test_phrase()
    });
    
    let mut resp = srv.post("/backend/protected/admin/phrases")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    
    println!("Create phrase response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Create phrase error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("id").is_some());
    assert!(body.get("phrase_text").is_some());
    assert!(body.get("active").is_some());
}

#[actix_web::test]
async fn test_get_pending_suggestions_success() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    let mut resp = srv.get("/backend/protected/admin/suggestions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get pending suggestions response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get pending suggestions error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("suggestions").is_some());
    assert!(body.get("total").is_some());
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[actix_web::test]
async fn test_deactivate_nonexistent_user() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Try to deactivate a nonexistent user
    let nonexistent_id = Uuid::new_v4();
    let mut resp = srv.post(&format!("/backend/protected/admin/users/{}/deactivate", nonexistent_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Deactivate nonexistent user response status: {}", resp.status());
    if resp.status() != 404 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Deactivate nonexistent user error response: {:?}", body);
    }
    assert_eq!(resp.status(), 200); // Success (idempotent operation)
}

#[actix_web::test]
async fn test_invalid_user_id_format() {
    let (srv, _pool, admin_user, _test_container) = crate::test_helpers::create_test_app_with_admin_user(
        &crate::test_helpers::unique_test_email(),
        &crate::test_helpers::test_password_hash(),
        "Admin User",
        &crate::test_helpers::unique_test_slug(),
    ).await.unwrap();
    
    let token = crate::test_helpers::create_test_jwt_token(&admin_user).await.unwrap();
    
    // Try to deactivate with invalid UUID format
    let mut resp = srv.post("/backend/protected/admin/users/invalid-uuid/deactivate")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Invalid UUID response status: {}", resp.status());
    if resp.status() != 404 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Invalid UUID error response: {:?}", body);
    }
    assert_eq!(resp.status(), 404); // Not Found (route doesn't match)
}
