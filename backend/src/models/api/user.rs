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

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub roles: Vec<String>,
    pub real_name: Option<String>,
    pub google_user_id: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
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

impl UserResponse {
    pub fn from_user_with_roles(user: User, roles: Vec<String>) -> Self {
        let email_verified = roles.contains(&"email-verified".to_string());

        UserResponse {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            roles,
            real_name: user.real_name,
            google_user_id: user.google_user_id,
            email_verified,
            created_at: user.created_at,
        }
    }
}

// Email verification request/response types
#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerificationEmailRequest {
    // No fields - uses authenticated user from JWT
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
