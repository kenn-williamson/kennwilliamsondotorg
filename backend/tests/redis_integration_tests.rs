use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};
use backend::middleware::rate_limit::{RateLimitService, RateLimitConfig};
use backend::middleware::rate_limit_trait::RateLimitServiceTrait;

#[actix_web::test]
async fn test_redis_rate_limiting_integration() {
    // Start Redis container
    let redis_image = GenericImage::new("redis", "alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));
    
    let redis_container = redis_image.start().await.expect("Failed to start Redis container");
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
    let redis_url = format!("redis://127.0.0.1:{}", redis_port);
    
    // Create rate limit service
    let rate_limit_service = RateLimitService::new(&redis_url)
        .expect("Failed to create rate limit service");
    
    // Test configuration
    let config = RateLimitConfig {
        requests_per_hour: 2,
        burst_limit: 1,
        burst_window: 60,
    };
    
    let identifier = "test_user";
    let endpoint = "test_endpoint";
    
    // First request should pass
    let should_limit = rate_limit_service.check_rate_limit(identifier, endpoint, &config).await.unwrap();
    assert!(!should_limit, "First request should not be limited");
    
    // Increment counter
    rate_limit_service.increment_rate_limit(identifier, endpoint, &config).await.unwrap();
    
    // Second request should be limited (burst limit = 1)
    let should_limit = rate_limit_service.check_rate_limit(identifier, endpoint, &config).await.unwrap();
    assert!(should_limit, "Second request should be limited by burst limit");
    
    // Test with different identifier - should not be limited
    let different_identifier = "different_user";
    let should_limit = rate_limit_service.check_rate_limit(different_identifier, endpoint, &config).await.unwrap();
    assert!(!should_limit, "Different user should not be limited");
}
