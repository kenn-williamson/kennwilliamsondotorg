use serde_json::json;

// Use consolidated test helpers from test_helpers module

#[actix_web::test]
async fn test_get_user_timers_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user to get proper authentication
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
    
    // Test getting user timers (should be empty initially)
    let mut resp = srv.get("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Get user timers response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get user timers error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_get_user_timers_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/incident-timers")
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_create_timer_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
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
    
    // Create a timer
    let timer_request_body = json!({
        "reset_timestamp": "2024-01-01T12:00:00Z",
        "notes": "Test timer notes"
    });
    
    let mut resp = srv.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request_body)
        .await
        .unwrap();
    
    println!("Create timer response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Create timer error response: {:?}", body);
    }
    assert_eq!(resp.status(), 201); // Created
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("id").is_some());
    assert!(body.get("reset_timestamp").is_some());
    assert_eq!(body.get("notes").unwrap(), "Test timer notes");
    assert!(body.get("created_at").is_some());
    assert!(body.get("updated_at").is_some());
}

#[actix_web::test]
async fn test_create_timer_minimal() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
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
    
    // Create a timer with minimal data
    let timer_request_body = json!({});
    
    let mut resp = srv.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request_body)
        .await
        .unwrap();
    
    println!("Create minimal timer response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Create minimal timer error response: {:?}", body);
    }
    assert_eq!(resp.status(), 201); // Created
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("id").is_some());
    assert!(body.get("reset_timestamp").is_some());
    assert!(body.get("created_at").is_some());
    assert!(body.get("updated_at").is_some());
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_create_timer_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let timer_request_body = json!({
        "reset_timestamp": "2024-01-01T12:00:00Z",
        "notes": "Test timer notes"
    });
    
    let mut resp = srv.post("/backend/protected/incident-timers")
        .send_json(&timer_request_body)
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_update_timer_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
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
    
    // Create a timer first
    let timer_request_body = json!({
        "reset_timestamp": "2024-01-01T12:00:00Z",
        "notes": "Original notes"
    });
    
    let mut create_resp = srv.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request_body)
        .await
        .unwrap();
    
    assert_eq!(create_resp.status(), 201);
    
    let create_body: serde_json::Value = create_resp.json().await.unwrap();
    let timer_id = create_body.get("id").unwrap().as_str().unwrap();
    
    // Update the timer
    let update_request_body = json!({
        "reset_timestamp": "2024-01-02T15:30:00Z",
        "notes": "Updated notes"
    });
    
    let mut resp = srv.put(&format!("/backend/protected/incident-timers/{}", timer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&update_request_body)
        .await
        .unwrap();
    
    println!("Update timer response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Update timer error response: {:?}", body);
    }
    assert_eq!(resp.status(), 200); // OK
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("id").unwrap().as_str().unwrap(), timer_id);
    assert_eq!(body.get("notes").unwrap(), "Updated notes");
    assert!(body.get("updated_at").is_some());
}

#[actix_web::test]
async fn test_update_timer_not_found() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
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
    
    // Try to update a non-existent timer
    let fake_timer_id = "01234567-89ab-cdef-0123-456789abcdef";
    let update_request_body = json!({
        "notes": "Updated notes"
    });
    
    let mut resp = srv.put(&format!("/backend/protected/incident-timers/{}", fake_timer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&update_request_body)
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 404); // Not Found
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "Timer not found");
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_update_timer_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let fake_timer_id = "01234567-89ab-cdef-0123-456789abcdef";
    let update_request_body = json!({
        "notes": "Updated notes"
    });
    
    let mut resp = srv.put(&format!("/backend/protected/incident-timers/{}", fake_timer_id))
        .send_json(&update_request_body)
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_delete_timer_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
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
    
    // Create a timer first
    let timer_request_body = json!({
        "reset_timestamp": "2024-01-01T12:00:00Z",
        "notes": "Timer to be deleted"
    });
    
    let mut create_resp = srv.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request_body)
        .await
        .unwrap();
    
    assert_eq!(create_resp.status(), 201);
    
    let create_body: serde_json::Value = create_resp.json().await.unwrap();
    let timer_id = create_body.get("id").unwrap().as_str().unwrap();
    
    // Delete the timer
    let mut resp = srv.delete(&format!("/backend/protected/incident-timers/{}", timer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    println!("Delete timer response status: {}", resp.status());
    if resp.status() != 204 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Delete timer error response: {:?}", body);
    }
    assert_eq!(resp.status(), 204); // No Content
    
    // Verify timer is deleted by trying to get user timers
    let mut get_resp = srv.get("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(get_resp.status(), 200);
    
    let get_body: serde_json::Value = get_resp.json().await.unwrap();
    assert!(get_body.is_array());
    assert_eq!(get_body.as_array().unwrap().len(), 0);
}

#[actix_web::test]
async fn test_delete_timer_not_found() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
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
    
    // Try to delete a non-existent timer
    let fake_timer_id = "01234567-89ab-cdef-0123-456789abcdef";
    
    let mut resp = srv.delete(&format!("/backend/protected/incident-timers/{}", fake_timer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 404); // Not Found
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "Timer not found");
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_delete_timer_unauthorized() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let fake_timer_id = "01234567-89ab-cdef-0123-456789abcdef";
    
    let mut resp = srv.delete(&format!("/backend/protected/incident-timers/{}", fake_timer_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_get_public_timer_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "Public Test User";
    
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
    
    // Get the user slug from the registration response
    let user = register_body.get("user").unwrap();
    let user_slug = user.get("slug").unwrap().as_str().unwrap();
    
    // Create a timer
    let timer_request_body = json!({
        "reset_timestamp": "2024-01-01T12:00:00Z",
        "notes": "Public timer notes"
    });
    
    let mut create_resp = srv.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request_body)
        .await
        .unwrap();
    
    assert_eq!(create_resp.status(), 201);
    
    // Get the public timer
    let mut resp = srv.get(&format!("/backend/public/{}/incident-timer", user_slug))
        .send()
        .await
        .unwrap();
    
    println!("Get public timer response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Get public timer error response: {:?}", body);
    }
    assert_eq!(resp.status(), 200); // OK
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("id").is_some());
    assert!(body.get("reset_timestamp").is_some());
    assert_eq!(body.get("notes").unwrap(), "Public timer notes");
    assert_eq!(body.get("user_display_name").unwrap(), display_name);
    assert!(body.get("created_at").is_some());
    assert!(body.get("updated_at").is_some());
}

#[actix_web::test]
async fn test_get_public_timer_not_found() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Try to get a timer for a non-existent user
    let fake_slug = "non-existent-user";
    
    let mut resp = srv.get(&format!("/backend/public/{}/incident-timer", fake_slug))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 404); // Not Found
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "No timer found for this user");
}

#[actix_web::test]
async fn test_get_public_timer_no_timers() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // First register a user but don't create any timers
    let email = crate::test_helpers::unique_test_email();
    let password = "TestPassword123!";
    let display_name = "User With No Timers";
    
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
    let user = register_body.get("user").unwrap();
    let user_slug = user.get("slug").unwrap().as_str().unwrap();
    
    // Try to get the public timer (should be 404 since no timers exist)
    let mut resp = srv.get(&format!("/backend/public/{}/incident-timer", user_slug))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 404); // Not Found
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "No timer found for this user");
}
