use anyhow::Result;
use async_trait::async_trait;

use super::trait_def::TurnstileServiceTrait;

/// Mock implementation of TurnstileService for testing
///
/// This mock allows tests to control whether verification succeeds or fails,
/// and can also simulate errors to test error handling paths.
pub struct MockTurnstileService {
    /// Whether verification should succeed (true) or fail (false)
    pub should_succeed: bool,
    /// If set, return this error instead of success/failure
    pub error_to_return: Option<String>,
}

impl Default for MockTurnstileService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTurnstileService {
    /// Create a new mock that succeeds by default
    pub fn new() -> Self {
        Self {
            should_succeed: true,
            error_to_return: None,
        }
    }

    /// Create a mock that always succeeds
    pub fn new_success() -> Self {
        Self {
            should_succeed: true,
            error_to_return: None,
        }
    }

    /// Create a mock that always fails verification
    pub fn new_failure() -> Self {
        Self {
            should_succeed: false,
            error_to_return: None,
        }
    }

    /// Create a mock that always returns an error
    pub fn new_error(error_message: String) -> Self {
        Self {
            should_succeed: false,
            error_to_return: Some(error_message),
        }
    }

    /// Set whether verification should succeed
    #[allow(dead_code)]
    pub fn set_should_succeed(&mut self, should_succeed: bool) {
        self.should_succeed = should_succeed;
    }

    /// Set an error to return
    #[allow(dead_code)]
    pub fn set_error(&mut self, error_message: Option<String>) {
        self.error_to_return = error_message;
    }
}

#[async_trait]
impl TurnstileServiceTrait for MockTurnstileService {
    async fn verify_token(&self, _token: &str, _ip_address: &str) -> Result<bool> {
        if let Some(error_msg) = &self.error_to_return {
            return Err(anyhow::anyhow!(error_msg.clone()));
        }
        Ok(self.should_succeed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_new_defaults_to_success() {
        let service = MockTurnstileService::new();
        let result = service.verify_token("test_token", "192.168.1.1").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_new_success() {
        let service = MockTurnstileService::new_success();
        let result = service.verify_token("test_token", "192.168.1.1").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_new_failure() {
        let service = MockTurnstileService::new_failure();
        let result = service.verify_token("test_token", "192.168.1.1").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_new_error() {
        let service = MockTurnstileService::new_error("API Error".to_string());
        let result = service.verify_token("test_token", "192.168.1.1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "API Error");
    }

    #[tokio::test]
    async fn test_mock_set_should_succeed() {
        let mut service = MockTurnstileService::new();
        service.set_should_succeed(false);
        let result = service.verify_token("test_token", "192.168.1.1").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_set_error() {
        let mut service = MockTurnstileService::new();
        service.set_error(Some("Network timeout".to_string()));
        let result = service.verify_token("test_token", "192.168.1.1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Network timeout");
    }
}
