// Use consolidated test helpers from test_helpers module

// ============================================================================
// HEALTH ENDPOINT TESTS
// ============================================================================

#[actix_web::test]
async fn test_health_endpoint_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/public/health")
        .send()
        .await
        .unwrap();
    
    println!("Health response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Health error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("status").unwrap(), "healthy");
    assert_eq!(body.get("service").unwrap(), "kennwilliamson-backend");
    assert_eq!(body.get("version").unwrap(), "0.1.0");
}

#[actix_web::test]
async fn test_health_db_endpoint_success() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/public/health/db")
        .send()
        .await
        .unwrap();
    
    println!("Health DB response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Health DB error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("status").unwrap(), "healthy");
    assert_eq!(body.get("database").unwrap(), "connected");
    assert_eq!(body.get("service").unwrap(), "kennwilliamson-backend");
    assert_eq!(body.get("version").unwrap(), "0.1.0");
}

#[actix_web::test]
#[allow(unused_mut)]
async fn test_health_endpoints_are_public() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Test basic health endpoint without authentication
    let mut resp = srv.get("/backend/public/health")
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    // Test database health endpoint without authentication
    let mut resp = srv.get("/backend/public/health/db")
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_health_endpoints_response_format() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Test basic health endpoint response format
    let mut resp = srv.get("/backend/public/health")
        .send()
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    let body: serde_json::Value = resp.json().await.unwrap();
    
    // Verify all required fields are present
    assert!(body.get("status").is_some());
    assert!(body.get("service").is_some());
    assert!(body.get("version").is_some());
    
    // Verify no unexpected fields
    let keys: Vec<String> = body.as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"status".to_string()));
    assert!(keys.contains(&"service".to_string()));
    assert!(keys.contains(&"version".to_string()));
}

#[actix_web::test]
async fn test_health_db_endpoint_response_format() {
    let (srv, _pool, _test_container, _email_service) = crate::test_helpers::create_test_app_with_testcontainers().await;
    
    // Test database health endpoint response format
    let mut resp = srv.get("/backend/public/health/db")
        .send()
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    let body: serde_json::Value = resp.json().await.unwrap();
    
    // Verify all required fields are present
    assert!(body.get("status").is_some());
    assert!(body.get("database").is_some());
    assert!(body.get("service").is_some());
    assert!(body.get("version").is_some());
    
    // Verify no unexpected fields
    let keys: Vec<String> = body.as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys.len(), 4);
    assert!(keys.contains(&"status".to_string()));
    assert!(keys.contains(&"database".to_string()));
    assert!(keys.contains(&"service".to_string()));
    assert!(keys.contains(&"version".to_string()));
}
