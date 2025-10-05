use serde::{Deserialize, Serialize};

/// User information from Google OAuth2 userinfo endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    /// Google user ID (subject identifier)
    pub sub: String,
    /// User's email address
    pub email: String,
    /// User's full name from Google profile
    pub name: Option<String>,
    /// User's given name
    pub given_name: Option<String>,
    /// User's family name
    pub family_name: Option<String>,
    /// User's profile picture URL
    pub picture: Option<String>,
    /// Whether Google has verified the email
    pub email_verified: Option<bool>,
    /// User's locale
    pub locale: Option<String>,
}
