use serde_json::json;

// Use consolidated test helpers from test_helpers module

#[actix_web::test]
async fn test_register_success() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let request_body = json!({
        "email": crate::test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User"
    });
    
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    
    println!("Registration response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Registration error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert!(body.get("user").is_some());
    
    let user = body.get("user").unwrap();
    assert_eq!(user.get("email").unwrap(), request_body.get("email").unwrap());
    assert_eq!(user.get("display_name").unwrap(), request_body.get("display_name").unwrap());
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let email = crate::test_helpers::unique_test_email();
    let request_body = json!({
        "email": email,
        "password": "TestPassword123!",
        "display_name": "Test User"
    });
    
    // First registration should succeed
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    // Second registration with same email should fail
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    assert_eq!(resp.status(), 409); // Conflict
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("error").is_some());
    assert!(body.get("error").unwrap().as_str().unwrap().contains("Email already exists"));
}

#[actix_web::test]
async fn test_login_success() {
    let (srv, pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = crate::test_helpers::unique_test_email();
    let password_hash = crate::test_helpers::test_password_hash();
    let display_name = "Test User";
    let slug = crate::test_helpers::unique_test_slug();
    
    let user = crate::test_helpers::create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let request_body = json!({
        "email": user.email,
        "password": "TestPassword123!"
    });
    
    let mut resp = srv.post("/backend/public/auth/login")
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert!(body.get("user").is_some());
    
    let returned_user = body.get("user").unwrap();
    assert_eq!(returned_user.get("id").unwrap().as_str().unwrap(), user.id.to_string());
    assert_eq!(returned_user.get("email").unwrap().as_str().unwrap(), user.email);
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let request_body = json!({
        "email": "nonexistent@example.com",
        "password": "WrongPassword123!"
    });
    
    let mut resp = srv.post("/backend/public/auth/login")
        .send_json(&request_body)
        .await
        .unwrap();
    assert_eq!(resp.status(), 401); // Unauthorized
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("error").is_some());
    assert_eq!(body.get("error").unwrap(), "Invalid email or password");
}

#[actix_web::test]
async fn test_get_current_user_success() {
    let (srv, pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = crate::test_helpers::unique_test_email();
    let password_hash = crate::test_helpers::test_password_hash();
    let display_name = "Test User";
    let slug = crate::test_helpers::unique_test_slug();
    
    let user = crate::test_helpers::create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    // Create JWT token for the user
    let token = crate::test_helpers::create_test_jwt_token(&user).await.unwrap();
    
    let mut resp = srv.get("/backend/protected/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("id").unwrap().as_str().unwrap(), user.id.to_string());
    assert_eq!(body.get("email").unwrap().as_str().unwrap(), user.email);
    assert_eq!(body.get("display_name").unwrap().as_str().unwrap(), user.display_name);
}

#[actix_web::test]
async fn test_get_current_user_unauthorized() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/auth/me")
        .send()
        .await
        .unwrap();
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_get_current_user_invalid_token() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/auth/me")
        .insert_header(("Authorization", "Bearer invalid_token"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_update_profile_success() {
    let (srv, pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = crate::test_helpers::unique_test_email();
    let password_hash = crate::test_helpers::test_password_hash();
    let display_name = "Original Name";
    let slug = crate::test_helpers::unique_test_slug();
    
    let user = crate::test_helpers::create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let token = crate::test_helpers::create_test_jwt_token(&user).await.unwrap();
    
    let request_body = json!({
        "display_name": "Updated Name",
        "slug": "updated-slug"
    });
    
    let mut resp = srv.put("/backend/protected/auth/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("display_name").unwrap(), "Updated Name");
    assert_eq!(body.get("slug").unwrap(), "updated-slug");
}

#[actix_web::test]
async fn test_change_password_success() {
    let (srv, pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = crate::test_helpers::unique_test_email();
    let password_hash = crate::test_helpers::test_password_hash();
    let display_name = "Test User";
    let slug = crate::test_helpers::unique_test_slug();
    
    let user = crate::test_helpers::create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let token = crate::test_helpers::create_test_jwt_token(&user).await.unwrap();
    
    let request_body = json!({
        "current_password": "TestPassword123!",
        "new_password": "NewPassword456!"
    });
    
    let mut resp = srv.put("/backend/protected/auth/change-password")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "Password changed successfully");
}

#[actix_web::test]
async fn test_change_password_wrong_current() {
    let (srv, pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = crate::test_helpers::unique_test_email();
    let password_hash = crate::test_helpers::test_password_hash();
    let display_name = "Test User";
    let slug = crate::test_helpers::unique_test_slug();
    
    let user = crate::test_helpers::create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let token = crate::test_helpers::create_test_jwt_token(&user).await.unwrap();
    
    let request_body = json!({
        "current_password": "WrongPassword123!",
        "new_password": "NewPassword456!"
    });
    
    let mut resp = srv.put("/backend/protected/auth/change-password")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    assert_eq!(resp.status(), 400); // Bad Request
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "Current password is incorrect");
}
