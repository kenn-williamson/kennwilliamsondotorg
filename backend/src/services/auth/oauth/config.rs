use anyhow::{anyhow, Result};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

// Type alias for BasicClient with endpoints configured
// BasicClient is already a type alias, so we just need to specify the endpoint typestates
pub type ConfiguredBasicClient = oauth2::basic::BasicClient<
    oauth2::EndpointSet,        // HasAuthUrl is set
    oauth2::EndpointNotSet,     // HasDeviceAuthUrl not set
    oauth2::EndpointNotSet,     // HasIntrospectionUrl not set
    oauth2::EndpointNotSet,     // HasRevocationUrl not set
    oauth2::EndpointSet,        // HasTokenUrl is set
>;

/// Google OAuth configuration
#[derive(Clone)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl GoogleOAuthConfig {
    /// Load Google OAuth configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("GOOGLE_CLIENT_ID")
            .map_err(|_| anyhow!("GOOGLE_CLIENT_ID not set"))?;
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .map_err(|_| anyhow!("GOOGLE_CLIENT_SECRET not set"))?;
        let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
            .map_err(|_| anyhow!("GOOGLE_REDIRECT_URI not set"))?;

        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
        })
    }

    /// Create an oauth2 BasicClient configured for Google
    pub fn create_client(&self) -> Result<ConfiguredBasicClient> {
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .map_err(|e| anyhow!("Invalid auth URL: {}", e))?,
            )
            .set_token_uri(
                TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                    .map_err(|e| anyhow!("Invalid token URL: {}", e))?,
            )
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_uri.clone())
                    .map_err(|e| anyhow!("Invalid redirect URI: {}", e))?,
            );

        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_requires_all_variables() {
        // Clear environment
        std::env::remove_var("GOOGLE_CLIENT_ID");
        std::env::remove_var("GOOGLE_CLIENT_SECRET");
        std::env::remove_var("GOOGLE_REDIRECT_URI");

        let result = GoogleOAuthConfig::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_env_success() {
        std::env::set_var("GOOGLE_CLIENT_ID", "test_client_id");
        std::env::set_var("GOOGLE_CLIENT_SECRET", "test_client_secret");
        std::env::set_var("GOOGLE_REDIRECT_URI", "https://localhost/callback");

        let result = GoogleOAuthConfig::from_env();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.client_id, "test_client_id");
        assert_eq!(config.client_secret, "test_client_secret");
        assert_eq!(config.redirect_uri, "https://localhost/callback");
    }

    #[test]
    fn test_create_client_success() {
        let config = GoogleOAuthConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "https://localhost/callback".to_string(),
        };

        let result = config.create_client();
        assert!(result.is_ok());
    }
}
