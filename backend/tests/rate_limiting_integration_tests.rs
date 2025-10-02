use serde_json::json;

mod test_helpers;

// Test rate limiting through actual HTTP requests following existing patterns

#[actix_web::test]
async fn test_rate_limiting_blocks_excessive_requests() {
    let (srv, _pool, _test_container) = test_helpers::create_test_app_with_testcontainers().await;
    
    // Make multiple rapid requests to trigger rate limiting
    // Registration endpoint has very restrictive limits (3/hour, 1 burst)
    let request_body = json!({
        "email": test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User"
    });
    
    // First request should succeed
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    
    // Should succeed initially
    assert!(resp.status().is_success());
    
    // Second request should be rate limited (burst limit = 1)
    let request_body2 = json!({
        "email": test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User 2"
    });
    
    let mut resp2 = srv.post("/backend/public/auth/register")
        .send_json(&request_body2)
        .await
        .unwrap();
    
    // Should be rate limited
    assert_eq!(resp2.status(), 429); // Too Many Requests
}

#[actix_web::test]
async fn test_rate_limiting_allows_normal_usage() {
    let (srv, _pool, _test_container) = test_helpers::create_test_app_with_testcontainers().await;
    
    // Test that normal usage patterns work
    // Health endpoint has generous limits (300/hour, 50 burst)
    let mut resp = srv.get("/backend/public/health")
        .send()
        .await
        .unwrap();
    
    assert!(resp.status().is_success());
    
    // Multiple health checks should work fine
    for _ in 0..5 {
        let mut resp = srv.get("/backend/public/health")
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
    }
}

#[actix_web::test]
async fn test_rate_limiting_different_endpoints_have_different_limits() {
    let (srv, _pool, _test_container) = test_helpers::create_test_app_with_testcontainers().await;
    
    // Test that different endpoints have different rate limits
    // Registration is very restrictive
    let request_body = json!({
        "email": test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User"
    });
    
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    // Second registration should be blocked
    let request_body2 = json!({
        "email": test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User 2"
    });
    
    let mut resp2 = srv.post("/backend/public/auth/register")
        .send_json(&request_body2)
        .await
        .unwrap();
    assert_eq!(resp2.status(), 429);
    
    // But health checks should still work (different endpoint)
    let mut resp3 = srv.get("/backend/public/health")
        .send()
        .await
        .unwrap();
    assert!(resp3.status().is_success());
}
