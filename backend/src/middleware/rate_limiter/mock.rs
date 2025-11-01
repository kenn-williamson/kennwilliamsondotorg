use async_trait::async_trait;

use super::config::RateLimitConfig;
use super::trait_def::RateLimitServiceTrait;

/// Mock implementation of RateLimitService for testing
pub struct MockRateLimitService {
    pub should_limit: bool,
    pub increment_called: bool,
    pub check_called: bool,
    pub last_identifier: Option<String>,
    pub last_endpoint: Option<String>,
}

impl Default for MockRateLimitService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockRateLimitService {
    pub fn new() -> Self {
        Self {
            should_limit: false,
            increment_called: false,
            check_called: false,
            last_identifier: None,
            last_endpoint: None,
        }
    }

    #[allow(dead_code)]
    pub fn set_should_limit(&mut self, should_limit: bool) {
        self.should_limit = should_limit;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.should_limit = false;
        self.increment_called = false;
        self.check_called = false;
        self.last_identifier = None;
        self.last_endpoint = None;
    }
}

#[async_trait]
impl RateLimitServiceTrait for MockRateLimitService {
    async fn check_rate_limit(
        &self,
        identifier: &str,
        endpoint: &str,
        _config: &RateLimitConfig,
    ) -> Result<bool, redis::RedisError> {
        // Store the parameters for verification
        let mut mock = self.clone();
        mock.check_called = true;
        mock.last_identifier = Some(identifier.to_string());
        mock.last_endpoint = Some(endpoint.to_string());

        Ok(self.should_limit)
    }

    async fn increment_rate_limit(
        &self,
        identifier: &str,
        endpoint: &str,
        _config: &RateLimitConfig,
    ) -> Result<(), redis::RedisError> {
        // Store the parameters for verification
        let mut mock = self.clone();
        mock.increment_called = true;
        mock.last_identifier = Some(identifier.to_string());
        mock.last_endpoint = Some(endpoint.to_string());

        Ok(())
    }
}

impl Clone for MockRateLimitService {
    fn clone(&self) -> Self {
        Self {
            should_limit: self.should_limit,
            increment_called: self.increment_called,
            check_called: self.check_called,
            last_identifier: self.last_identifier.clone(),
            last_endpoint: self.last_endpoint.clone(),
        }
    }
}
