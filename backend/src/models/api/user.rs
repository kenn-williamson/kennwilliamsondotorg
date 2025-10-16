use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::db::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Nested profile data in API response
#[derive(Debug, Serialize)]
pub struct ProfileData {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

/// Nested external account data in API response
#[derive(Debug, Serialize)]
pub struct ExternalAccount {
    pub provider: String,
    pub linked_at: DateTime<Utc>,
}

/// Nested preferences data in API response
#[derive(Debug, Serialize)]
pub struct PreferencesData {
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
}

/// User response with nested structure for modularity
/// This structure hides the multi-table implementation from the frontend
/// and makes it easy to add new OAuth providers or preferences without breaking changes
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub roles: Vec<String>,
    pub email_verified: bool,
    pub has_credentials: bool,
    pub created_at: DateTime<Utc>,
    pub profile: Option<ProfileData>,
    pub external_accounts: Vec<ExternalAccount>,
    pub preferences: Option<PreferencesData>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlugPreviewRequest {
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct SlugPreviewResponse {
    pub slug: String,
    pub available: bool,
    pub final_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlugValidationRequest {
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct SlugValidationResponse {
    pub slug: String,
    pub valid: bool,
    pub available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfileUpdateRequest {
    pub display_name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPasswordRequest {
    pub new_password: String,
}

impl UserResponse {
    /// Create UserResponse from core User and roles only (minimal data)
    /// NOTE: This creates an empty response - use AuthService::build_user_response_with_details for populated data
    #[allow(dead_code)]
    pub fn from_user_with_roles(user: User, roles: Vec<String>) -> Self {
        let email_verified = roles.contains(&"email-verified".to_string());

        UserResponse {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            roles,
            email_verified,
            has_credentials: false, // Minimal response doesn't check credentials
            created_at: user.created_at,
            profile: None,
            external_accounts: vec![],
            preferences: None,
        }
    }
}


#[derive(Debug, Serialize)]
pub struct SendVerificationEmailResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyEmailResponse {
    pub message: String,
    pub email_verified: bool,
}

// Password Reset request/response types
#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

// Google OAuth request/response types
#[derive(Debug, Serialize)]
pub struct GoogleOAuthUrlResponse {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleOAuthCallbackRequest {
    pub code: String,
    pub state: Option<String>,
}
