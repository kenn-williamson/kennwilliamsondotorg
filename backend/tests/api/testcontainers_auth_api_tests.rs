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
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper password hashing
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
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
    
    // Now try to login with the same credentials
    let login_request_body = json!({
        "email": email,
        "password": password
    });
    
    let mut resp = srv.post("/backend/public/auth/login")
        .send_json(&login_request_body)
        .await
        .unwrap();
    
    println!("Login response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Login error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert!(body.get("user").is_some());
    
    let returned_user = body.get("user").unwrap();
    assert_eq!(returned_user.get("email").unwrap().as_str().unwrap(), email);
    assert_eq!(returned_user.get("display_name").unwrap().as_str().unwrap(), display_name);
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
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper password hashing
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
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
    
    // Now test getting current user with the JWT token
    let mut resp = srv.get("/backend/protected/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get current user response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get current user error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("email").unwrap().as_str().unwrap(), email);
    assert_eq!(body.get("display_name").unwrap().as_str().unwrap(), display_name);
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
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper password hashing
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Original Name";
    
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
    
    // Now test updating profile
    let request_body = json!({
        "display_name": "Updated Name",
        "slug": "updated-slug"
    });
    
    let mut resp = srv.put("/backend/protected/auth/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    
    println!("Update profile response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Update profile error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("display_name").unwrap(), "Updated Name");
    assert_eq!(body.get("slug").unwrap(), "updated-slug");
}

#[actix_web::test]
async fn test_change_password_success() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper password hashing
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
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
    
    // Now test changing password
    let request_body = json!({
        "current_password": password,
        "new_password": "NewPassword456!"
    });
    
    let mut resp = srv.put("/backend/protected/auth/change-password")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    
    println!("Change password response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Change password error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "Password changed successfully");
}

#[actix_web::test]
async fn test_change_password_wrong_current() {
    let (srv, _pool, _test_container) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper password hashing
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Test User";
    
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
    
    // Now test changing password with wrong current password
    let request_body = json!({
        "current_password": "WrongPassword123!",
        "new_password": "NewPassword456!"
    });
    
    let mut resp = srv.put("/backend/protected/auth/change-password")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    
    println!("Change password wrong current response status: {}", resp.status());
    if resp.status() != 400 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Change password wrong current error response: {:?}", body);
    }
    assert_eq!(resp.status(), 400); // Bad Request
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "Current password is incorrect");
}
