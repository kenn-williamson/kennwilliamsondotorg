use anyhow::{anyhow, Result};
use async_trait::async_trait;
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse as _,
};

use super::config::{GoogleOAuthConfig, ConfiguredBasicClient};
use crate::models::oauth::GoogleUserInfo;

/// Trait for Google OAuth operations (allows mocking in tests)
#[async_trait]
pub trait GoogleOAuthServiceTrait: Send + Sync {
    /// Generate Google OAuth authorization URL with PKCE
    /// Optionally accepts a custom state token (for encoding redirect info)
    /// Returns: (auth_url, csrf_token, pkce_verifier)
    async fn get_authorization_url(&self, custom_state: Option<String>) -> Result<(String, CsrfToken, PkceCodeVerifier)>;

    /// Exchange authorization code for access token using PKCE verifier
    async fn exchange_code_for_token(
        &self,
        code: String,
        verifier: PkceCodeVerifier,
    ) -> Result<String>;

    /// Fetch user information from Google using access token
    async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo>;
}

/// Production implementation of Google OAuth service
pub struct GoogleOAuthService {
    client: ConfiguredBasicClient,
}

impl GoogleOAuthService {
    /// Create a new Google OAuth service from configuration
    pub fn new(config: GoogleOAuthConfig) -> Result<Self> {
        let client = config.create_client()?;
        Ok(Self { client })
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let config = GoogleOAuthConfig::from_env()?;
        Self::new(config)
    }
}

#[async_trait]
impl GoogleOAuthServiceTrait for GoogleOAuthService {
    async fn get_authorization_url(&self, custom_state: Option<String>) -> Result<(String, CsrfToken, PkceCodeVerifier)> {
        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate authorization URL with PKCE and scopes
        // Use custom state if provided, otherwise generate random
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(|| {
                if let Some(ref state) = custom_state {
                    CsrfToken::new(state.clone())
                } else {
                    CsrfToken::new_random()
                }
            })
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok((auth_url.to_string(), csrf_token, pkce_verifier))
    }

    async fn exchange_code_for_token(
        &self,
        code: String,
        verifier: PkceCodeVerifier,
    ) -> Result<String> {
        // Use reqwest::Client directly as it implements AsyncHttpClient
        let http_client = reqwest::Client::new();

        // Exchange authorization code for access token
        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| anyhow!("Token exchange failed: {}", e))?;

        let access_token = token_result.access_token().secret().to_string();
        Ok(access_token)
    }

    async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo> {
        // Fetch user info from Google's userinfo endpoint
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch user info: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Google userinfo request failed with status: {}",
                response.status()
            ));
        }

        let user_info: GoogleUserInfo = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse user info: {}", e))?;

        Ok(user_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_oauth_service_creation() {
        let config = GoogleOAuthConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "https://localhost/callback".to_string(),
        };

        let result = GoogleOAuthService::new(config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_authorization_url_generates_valid_url() {
        let config = GoogleOAuthConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "https://localhost/callback".to_string(),
        };

        let service = GoogleOAuthService::new(config).unwrap();
        let result = service.get_authorization_url(None).await;

        assert!(result.is_ok());
        let (url, csrf_token, verifier) = result.unwrap();

        // Verify URL contains expected components
        assert!(url.contains("accounts.google.com/o/oauth2/v2/auth"));
        assert!(url.contains("client_id=test_id"));
        assert!(url.contains("redirect_uri="));
        assert!(url.contains("scope="));
        assert!(url.contains("code_challenge="));
        assert!(url.contains("code_challenge_method=S256"));

        // Verify tokens are generated
        assert!(!csrf_token.secret().is_empty());
        assert!(!verifier.secret().is_empty());
    }

    // Note: Token exchange and user info tests require mocking HTTP calls
    // or integration tests with real Google API. Unit tests use MockGoogleOAuthService.
}

