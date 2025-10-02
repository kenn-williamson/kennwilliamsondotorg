use super::config::RateLimitConfig;

/// Trait for rate limiting operations to enable mocking in tests
#[async_trait::async_trait]
pub trait RateLimitServiceTrait: Send + Sync {
    /// Check if request should be rate limited
    async fn check_rate_limit(
        &self,
        identifier: &str,
        endpoint: &str,
        config: &RateLimitConfig,
    ) -> Result<bool, redis::RedisError>;

    /// Increment rate limit counters
    async fn increment_rate_limit(
        &self,
        identifier: &str,
        endpoint: &str,
        config: &RateLimitConfig,
    ) -> Result<(), redis::RedisError>;
}
