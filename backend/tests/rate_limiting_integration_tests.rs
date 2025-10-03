use serde_json::json;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

mod test_helpers;

// Test rate limiting through actual HTTP requests with real Redis backend

#[actix_web::test]
async fn test_rate_limiting_blocks_excessive_requests() {
    // Start Redis container for rate limiting
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let _redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = _redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    // Create test app with Redis-backed rate limiting
    let ctx = test_helpers::TestContext::builder()
        .with_redis(redis_url)
        .build()
        .await;

    // Make multiple rapid requests to trigger rate limiting
    // Registration endpoint has very restrictive limits (3/hour, 1 burst)
    let request_body = json!({
        "email": test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User"
    });

    // First request should succeed
    let resp = ctx.server.post("/backend/public/auth/register")
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

    let resp2 = ctx.server.post("/backend/public/auth/register")
        .send_json(&request_body2)
        .await
        .unwrap();
    
    // Should be rate limited
    assert_eq!(resp2.status(), 429); // Too Many Requests
}

#[actix_web::test]
async fn test_rate_limiting_allows_normal_usage() {
    // Start Redis container for rate limiting
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let _redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = _redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    let ctx = test_helpers::TestContext::builder()
        .with_redis(redis_url)
        .build()
        .await;

    // Test that normal usage patterns work
    // Health endpoint has generous limits (300/hour, 50 burst)
    let resp = ctx.server.get("/backend/public/health")
        .send()
        .await
        .unwrap();

    assert!(resp.status().is_success());

    // Multiple health checks should work fine
    for _ in 0..5 {
        let resp = ctx.server.get("/backend/public/health")
            .send()
            .await
            .unwrap();
        assert!(resp.status().is_success());
    }
}

#[actix_web::test]
async fn test_rate_limiting_different_endpoints_have_different_limits() {
    // Start Redis container for rate limiting
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let _redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = _redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);

    let ctx = test_helpers::TestContext::builder()
        .with_redis(redis_url)
        .build()
        .await;

    // Test that different endpoints have different rate limits
    // Registration is very restrictive
    let request_body = json!({
        "email": test_helpers::unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User"
    });

    let resp = ctx.server.post("/backend/public/auth/register")
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

    let resp2 = ctx.server.post("/backend/public/auth/register")
        .send_json(&request_body2)
        .await
        .unwrap();
    assert_eq!(resp2.status(), 429);

    // But health checks should still work (different endpoint)
    let resp3 = ctx.server.get("/backend/public/health")
        .send()
        .await
        .unwrap();
    assert!(resp3.status().is_success());
}
