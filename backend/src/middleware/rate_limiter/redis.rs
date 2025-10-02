use redis::{Client, Commands};
use async_trait::async_trait;

use super::config::RateLimitConfig;
use super::trait_def::RateLimitServiceTrait;

/// Redis-based rate limiting service implementation
#[derive(Clone)]
pub struct RedisRateLimitService {
    redis_client: Client,
}

impl RedisRateLimitService {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            redis_client: client,
        })
    }
}

#[async_trait]
impl RateLimitServiceTrait for RedisRateLimitService {

    /// Check if request should be rate limited
    async fn check_rate_limit(
        &self,
        identifier: &str,
        endpoint: &str,
        config: &RateLimitConfig,
    ) -> Result<bool, redis::RedisError> {
        let mut conn = self.redis_client.get_connection()?;

        // Check hourly limit
        let hourly_key = format!("rate_limit:hourly:{}:{}", identifier, endpoint);
        let hourly_count: u32 = conn.get(&hourly_key).unwrap_or(0);

        if hourly_count >= config.requests_per_hour {
            log::warn!("Rate limit exceeded for {} on {}: {} requests/hour",
                      identifier, endpoint, hourly_count);
            return Ok(true);
        }

        // Check burst limit
        let burst_key = format!("rate_limit:burst:{}:{}", identifier, endpoint);
        let burst_count: u32 = conn.get(&burst_key).unwrap_or(0);

        if burst_count >= config.burst_limit {
            log::warn!("Burst limit exceeded for {} on {}: {} requests/{}s",
                      identifier, endpoint, burst_count, config.burst_window);
            return Ok(true);
        }

        Ok(false)
    }

    /// Increment rate limit counters
    async fn increment_rate_limit(
        &self,
        identifier: &str,
        endpoint: &str,
        config: &RateLimitConfig,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_connection()?;

        // Increment hourly counter
        let hourly_key = format!("rate_limit:hourly:{}:{}", identifier, endpoint);
        let _: () = conn.incr(&hourly_key, 1)?;
        let _: () = conn.expire(&hourly_key, 3600)?; // 1 hour

        // Increment burst counter
        let burst_key = format!("rate_limit:burst:{}:{}", identifier, endpoint);
        let _: () = conn.incr(&burst_key, 1)?;
        let _: () = conn.expire(&burst_key, config.burst_window as i64)?;

        Ok(())
    }
}
