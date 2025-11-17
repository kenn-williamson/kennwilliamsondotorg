use anyhow::{Result, anyhow};
use async_trait::async_trait;
use oauth2::{CsrfToken, PkceCodeVerifier};
use std::sync::{Arc, Mutex};

use super::GoogleOAuthServiceTrait;
use crate::models::oauth::GoogleUserInfo;

/// Mock Google OAuth service for testing
#[derive(Clone)]
#[allow(dead_code)] // Testing infrastructure - used in test files
pub struct MockGoogleOAuthService {
    state: Arc<Mutex<MockState>>,
}

#[derive(Default)]
#[allow(dead_code)] // Testing infrastructure - internal state for MockGoogleOAuthService
struct MockState {
    /// Whether get_authorization_url should fail
    pub url_should_fail: bool,
    /// Whether exchange_code_for_token should fail
    pub exchange_should_fail: bool,
    /// Whether get_user_info should fail
    pub user_info_should_fail: bool,
    /// Mock user info to return
    pub mock_user_info: Option<GoogleUserInfo>,
    /// Mock access token to return
    pub mock_access_token: Option<String>,
}

impl Default for MockGoogleOAuthService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockGoogleOAuthService {
    #[allow(dead_code)] // Testing infrastructure API
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState::default())),
        }
    }

    /// Configure mock to fail on get_authorization_url
    #[allow(dead_code)] // Testing infrastructure API
    pub fn with_url_failure(self) -> Self {
        self.state.lock().unwrap().url_should_fail = true;
        self
    }

    /// Configure mock to fail on exchange_code_for_token
    #[allow(dead_code)] // Testing infrastructure API
    pub fn with_exchange_failure(self) -> Self {
        self.state.lock().unwrap().exchange_should_fail = true;
        self
    }

    /// Configure mock to fail on get_user_info
    #[allow(dead_code)] // Testing infrastructure API
    pub fn with_user_info_failure(self) -> Self {
        self.state.lock().unwrap().user_info_should_fail = true;
        self
    }

    /// Configure mock to return specific user info
    #[allow(dead_code)] // Testing infrastructure API
    pub fn with_user_info(self, user_info: GoogleUserInfo) -> Self {
        self.state.lock().unwrap().mock_user_info = Some(user_info);
        self
    }

    /// Configure mock to return specific access token
    #[allow(dead_code)] // Testing infrastructure API
    pub fn with_access_token(self, token: String) -> Self {
        self.state.lock().unwrap().mock_access_token = Some(token);
        self
    }
}

#[async_trait]
impl GoogleOAuthServiceTrait for MockGoogleOAuthService {
    async fn get_authorization_url(
        &self,
        custom_state: Option<String>,
    ) -> Result<(String, CsrfToken, PkceCodeVerifier)> {
        let state = self.state.lock().unwrap();
        if state.url_should_fail {
            return Err(anyhow!("Mock URL generation failure"));
        }

        // Use custom state if provided, otherwise use default mock state
        let csrf_token_value = custom_state.unwrap_or_else(|| "mock_csrf_token".to_string());

        // Return mock URL with all expected query parameters for testing
        let url = format!(
            "https://accounts.google.com/o/oauth2/v2/auth\
            ?client_id=mock_client_id\
            &redirect_uri=https%3A%2F%2Flocalhost%2Fcallback\
            &response_type=code\
            &scope=openid+email+profile\
            &state={}\
            &code_challenge=mock_code_challenge\
            &code_challenge_method=S256",
            csrf_token_value
        );

        Ok((
            url,
            CsrfToken::new(csrf_token_value),
            PkceCodeVerifier::new("mock_pkce_verifier".to_string()),
        ))
    }

    async fn exchange_code_for_token(
        &self,
        _code: String,
        _verifier: PkceCodeVerifier,
    ) -> Result<String> {
        let state = self.state.lock().unwrap();
        if state.exchange_should_fail {
            return Err(anyhow!("Mock token exchange failure"));
        }

        Ok(state
            .mock_access_token
            .clone()
            .unwrap_or_else(|| "mock_access_token".to_string()))
    }

    async fn get_user_info(&self, _access_token: &str) -> Result<GoogleUserInfo> {
        let state = self.state.lock().unwrap();
        if state.user_info_should_fail {
            return Err(anyhow!("Mock user info fetch failure"));
        }

        Ok(state
            .mock_user_info
            .clone()
            .unwrap_or_else(|| GoogleUserInfo {
                sub: "mock_google_user_id".to_string(),
                email: "mock@example.com".to_string(),
                name: Some("Mock User".to_string()),
                email_verified: Some(true),
                picture: Some("https://example.com/mock_user.jpg".to_string()),
                given_name: Some("Mock".to_string()),
                family_name: Some("User".to_string()),
                locale: Some("en".to_string()),
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_default_behavior() {
        let mock = MockGoogleOAuthService::new();

        // Test URL generation
        let result = mock.get_authorization_url(None).await;
        assert!(result.is_ok());

        // Test token exchange
        let result = mock
            .exchange_code_for_token(
                "code".to_string(),
                PkceCodeVerifier::new("verifier".to_string()),
            )
            .await;
        assert!(result.is_ok());

        // Test user info
        let result = mock.get_user_info("token").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_with_failures() {
        let mock = MockGoogleOAuthService::new()
            .with_url_failure()
            .with_exchange_failure()
            .with_user_info_failure();

        assert!(mock.get_authorization_url(None).await.is_err());
        assert!(
            mock.exchange_code_for_token(
                "code".to_string(),
                PkceCodeVerifier::new("verifier".to_string())
            )
            .await
            .is_err()
        );
        assert!(mock.get_user_info("token").await.is_err());
    }

    #[tokio::test]
    async fn test_mock_with_custom_user_info() {
        let custom_user_info = GoogleUserInfo {
            sub: "custom_id".to_string(),
            email: "custom@example.com".to_string(),
            name: Some("Custom Name".to_string()),
            given_name: None,
            family_name: None,
            picture: None,
            email_verified: Some(false),
            locale: None,
        };

        let mock = MockGoogleOAuthService::new().with_user_info(custom_user_info.clone());

        let result = mock.get_user_info("token").await.unwrap();
        assert_eq!(result.sub, "custom_id");
        assert_eq!(result.email, "custom@example.com");
        assert_eq!(result.email_verified, Some(false));
    }
}
