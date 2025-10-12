use crate::fixtures::TestContext;

#[actix_web::test]
async fn test_full_export_workflow() {
    let ctx = TestContext::builder().build().await;
    
    // Create a test user
    let email = crate::fixtures::unique_test_email();
    let slug = crate::fixtures::unique_test_slug();
    let user = ctx.create_verified_user(&email, &slug).await;
    
    let token = crate::fixtures::create_test_jwt_token(&user).await.unwrap();
    
    // Create some test data - incident timer
    let timer_request = serde_json::json!({
        "notes": "Test Description",
        "reset_timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    #[allow(unused_mut)]
    let mut resp = ctx.server.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request)
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    // Test data export
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    
    let body = resp.json::<serde_json::Value>().await.unwrap();

    // Verify export structure
    assert_eq!(body["export_version"], "1.0");
    assert_eq!(body["user"]["id"], user.id.to_string());
    assert_eq!(body["user"]["email"], email);
    assert_eq!(body["user"]["display_name"], user.display_name);
    
    // Verify incident timers are included
    let timers = body["incident_timers"].as_array().unwrap();
    assert_eq!(timers.len(), 1);
    assert_eq!(timers[0]["notes"], "Test Description");
    
    // Verify other arrays are empty or present
    assert!(body["phrase_suggestions"].is_array());
    assert!(body["phrase_exclusions"].is_array());
    assert!(body["active_sessions"].is_array());
    assert!(body["verification_history"].is_array());
}

#[actix_web::test]
async fn test_export_with_all_data_types() {
    let ctx = TestContext::builder().build().await;
    
    // Create a test user
    let email = crate::fixtures::unique_test_email();
    let slug = crate::fixtures::unique_test_slug();
    let user = ctx.create_verified_user(&email, &slug).await;
    
    let token = crate::fixtures::create_test_jwt_token(&user).await.unwrap();
    
    // Create incident timer
    let timer_request = serde_json::json!({
        "notes": "Test Description",
        "reset_timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    #[allow(unused_mut)]
    let mut resp = ctx.server.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&timer_request)
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    // Create phrase suggestion
    let phrase_request = serde_json::json!({
        "phrase_text": "test phrase"
    });
    
    #[allow(unused_mut)]
    let mut resp = ctx.server.post("/backend/protected/phrases/suggestions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&phrase_request)
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    // Test data export
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    
    let export_data = resp.json::<serde_json::Value>().await.unwrap();
    
    // Verify user data
    assert_eq!(export_data["user"]["id"], user.id.to_string());
    assert_eq!(export_data["user"]["email"], email);
    
    // Verify all data types are present
    let timers = export_data["incident_timers"].as_array().unwrap();
    assert_eq!(timers.len(), 1);
    assert_eq!(timers[0]["notes"], "Test Description");
    
    let suggestions = export_data["phrase_suggestions"].as_array().unwrap();
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0]["phrase_text"], "test phrase");
    
    // Verify other arrays are present
    assert!(export_data["phrase_exclusions"].is_array());
    assert!(export_data["active_sessions"].is_array());
    assert!(export_data["verification_history"].is_array());
}

#[actix_web::test]
async fn test_export_data_isolation() {
    let ctx = TestContext::builder().build().await;
    
    // Create two test users
    let email1 = crate::fixtures::unique_test_email();
    let slug1 = crate::fixtures::unique_test_slug();
    let user1 = ctx.create_verified_user(&email1, &slug1).await;
    
    let email2 = crate::fixtures::unique_test_email();
    let slug2 = crate::fixtures::unique_test_slug();
    let user2 = ctx.create_verified_user(&email2, &slug2).await;
    
    let token1 = crate::fixtures::create_test_jwt_token(&user1).await.unwrap();
    let token2 = crate::fixtures::create_test_jwt_token(&user2).await.unwrap();
    
    // Create timer for user 1
    let timer_request1 = serde_json::json!({
        "notes": "User 1 Description",
        "reset_timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    #[allow(unused_mut)]
    let mut resp = ctx.server.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token1)))
        .send_json(&timer_request1)
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    // Create timer for user 2
    let timer_request2 = serde_json::json!({
        "notes": "User 2 Description",
        "reset_timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    #[allow(unused_mut)]
    let mut resp = ctx.server.post("/backend/protected/incident-timers")
        .insert_header(("Authorization", format!("Bearer {}", token2)))
        .send_json(&timer_request2)
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    // Test user 1 export
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token1)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let export_data1 = resp.json::<serde_json::Value>().await.unwrap();
    
    // Verify user 1 only sees their own data
    assert_eq!(export_data1["user"]["id"], user1.id.to_string());
    assert_eq!(export_data1["user"]["email"], email1);
    
    let timers1 = export_data1["incident_timers"].as_array().unwrap();
    assert_eq!(timers1.len(), 1);
    assert_eq!(timers1[0]["notes"], "User 1 Description");
    
    // Test user 2 export
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token2)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    let export_data2 = resp.json::<serde_json::Value>().await.unwrap();
    
    // Verify user 2 only sees their own data
    assert_eq!(export_data2["user"]["id"], user2.id.to_string());
    assert_eq!(export_data2["user"]["email"], email2);
    
    let timers2 = export_data2["incident_timers"].as_array().unwrap();
    assert_eq!(timers2.len(), 1);
    assert_eq!(timers2[0]["notes"], "User 2 Description");
    
    // Verify no cross-contamination
    assert_ne!(export_data1["user"]["id"], export_data2["user"]["id"]);
    assert_ne!(timers1[0]["id"], timers2[0]["id"]);
}

#[actix_web::test]
async fn test_export_data_empty_user() {
    let ctx = TestContext::builder().build().await;
    
    // Create a test user
    let email = crate::fixtures::unique_test_email();
    let slug = crate::fixtures::unique_test_slug();
    let user = ctx.create_verified_user(&email, &slug).await;
    
    let token = crate::fixtures::create_test_jwt_token(&user).await.unwrap();
    
    // Test data export (no additional data created)
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    
    let export_data = resp.json::<serde_json::Value>().await.unwrap();
    
    assert_eq!(export_data["user"]["id"], user.id.to_string());
    assert_eq!(export_data["user"]["email"], email);
    assert_eq!(export_data["incident_timers"].as_array().unwrap().len(), 0);
    assert_eq!(export_data["phrase_suggestions"].as_array().unwrap().len(), 0);
    assert_eq!(export_data["phrase_exclusions"].as_array().unwrap().len(), 0);
    assert_eq!(export_data["active_sessions"].as_array().unwrap().len(), 0);
    assert_eq!(export_data["verification_history"].as_array().unwrap().len(), 0);
}

#[actix_web::test]
async fn test_export_data_response_headers() {
    let ctx = TestContext::builder().build().await;
    
    // Create a test user
    let email = crate::fixtures::unique_test_email();
    let slug = crate::fixtures::unique_test_slug();
    let user = ctx.create_verified_user(&email, &slug).await;
    
    let token = crate::fixtures::create_test_jwt_token(&user).await.unwrap();
    
    // Test data export
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    
    assert_eq!(resp.status(), 200);
    
    // Verify response headers
    let content_type = resp.headers().get("content-type").unwrap();
    assert_eq!(content_type, "application/json");
    
    let content_disposition = resp.headers().get("content-disposition").unwrap();
    assert!(content_disposition.to_str().unwrap().contains("attachment"));
    assert!(content_disposition.to_str().unwrap().contains("filename="));
    assert!(content_disposition.to_str().unwrap().contains(".json"));
}

#[actix_web::test]
async fn test_export_data_requires_authentication() {
    let ctx = TestContext::builder().build().await;
    
    // Test data export without authentication
    #[allow(unused_mut)]
    let mut resp = ctx.server.get("/backend/protected/auth/export-data")
        .send()
        .await
        .unwrap();
    
    // Should return 401 Unauthorized
    assert_eq!(resp.status(), 401);
}