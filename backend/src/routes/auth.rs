use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result as ActixResult};
use serde_json::json;
use uuid::Uuid;

use crate::models::api::{
    CreateUserRequest, LoginRequest, PasswordChangeRequest, ProfileUpdateRequest,
    RefreshTokenRequest, RevokeTokenRequest, SlugPreviewRequest, SlugValidationRequest,
    VerifyEmailRequest,
};
use crate::services::auth::AuthService;

/// Extract device information from HTTP request headers
/// Handles forwarded headers from proxies/load balancers using Actix Web's built-in support
fn extract_device_info(req: &HttpRequest) -> Option<serde_json::Value> {
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    // Extract real IP address using Actix Web's built-in forwarded header support
    // This handles Forwarded, X-Forwarded-For, and X-Real-IP headers automatically
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| {
            // Fallback to direct connection IP if no forwarded headers
            req.connection_info()
                .peer_addr()
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        });

    Some(json!({
        "user_agent": user_agent,
        "ip_address": ip_address,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn register(
    data: web::Json<CreateUserRequest>,
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let device_info = extract_device_info(&req);
    let frontend_url = std::env::var("FRONTEND_URL").ok();
    match auth_service
        .register(data.into_inner(), device_info, frontend_url.as_deref())
        .await
    {
        Ok(auth_response) => Ok(HttpResponse::Created().json(auth_response)),
        Err(err) => {
            if err.to_string().contains("duplicate key") {
                Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Email already exists"
                })))
            } else {
                log::error!("Registration error: {}", err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                })))
            }
        }
    }
}

pub async fn login(
    data: web::Json<LoginRequest>,
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let device_info = extract_device_info(&req);
    match auth_service.login(data.into_inner(), device_info).await {
        Ok(Some(auth_response)) => Ok(HttpResponse::Ok().json(auth_response)),
        Ok(None) => Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        }))),
        Err(err) => {
            log::error!("Login error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn preview_slug(
    data: web::Json<SlugPreviewRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service.preview_slug(data.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            log::error!("Slug preview error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn validate_slug(
    query: web::Query<SlugValidationRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service.validate_slug(query.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            log::error!("Slug validation error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn get_current_user(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service.get_current_user(user_id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(err) => {
            log::error!("Get current user error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn refresh(
    data: web::Json<RefreshTokenRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service.refresh_token(data.into_inner()).await {
        Ok(Some(response)) => Ok(HttpResponse::Ok().json(response)),
        Ok(None) => Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid or expired refresh token"
        }))),
        Err(err) => {
            log::error!("Token refresh error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn revoke(
    data: web::Json<RevokeTokenRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service.revoke_refresh_token(data.into_inner()).await {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Token revoked successfully"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Token not found"
        }))),
        Err(err) => {
            log::error!("Token revocation error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn revoke_all(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service.revoke_all_user_tokens(user_id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": format!("Revoked {} tokens", count)
        }))),
        Err(err) => {
            log::error!("Revoke all tokens error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub async fn update_profile(
    req: HttpRequest,
    data: web::Json<ProfileUpdateRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service
        .update_profile(user_id, data.into_inner())
        .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            if err.to_string().contains("Invalid slug format") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid slug format"
                })))
            } else if err.to_string().contains("Slug already taken") {
                Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Username already taken"
                })))
            } else {
                log::error!("Profile update error: {}", err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                })))
            }
        }
    }
}

pub async fn change_password(
    req: HttpRequest,
    data: web::Json<PasswordChangeRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service
        .change_password(user_id, data.into_inner())
        .await
    {
        Ok(()) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Password changed successfully"
        }))),
        Err(err) => {
            if err.to_string().contains("Current password is incorrect") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Current password is incorrect"
                })))
            } else if err.to_string().contains("User not found") {
                Ok(HttpResponse::NotFound().json(serde_json::json!({
                    "error": "User not found"
                })))
            } else {
                log::error!("Password change error: {}", err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                })))
            }
        }
    }
}

/// Send verification email to authenticated user
/// POST /backend/protected/auth/send-verification
pub async fn send_verification_email_handler(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let frontend_url = std::env::var("FRONTEND_URL")
        .ok()
        .unwrap_or_else(|| "https://kennwilliamson.org".to_string());

    match auth_service
        .send_verification_email(user_id, &frontend_url)
        .await
    {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            log::error!("Send verification email error: {}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to send verification email"
            })))
        }
    }
}

/// Verify email with token from email link
/// GET /backend/public/auth/verify-email?token=XXX
pub async fn verify_email_handler(
    query: web::Query<VerifyEmailRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service.verify_email(&query.token).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            log::error!("Email verification error: {}", err);
            Ok(HttpResponse::BadRequest().json(json!({
                "error": "Invalid or expired verification token"
            })))
        }
    }
}

// ============================================================================
// GOOGLE OAUTH ROUTES  
// ============================================================================

/// GET /backend/public/auth/google/url
/// Get Google OAuth authorization URL with PKCE challenge
/// PKCE verifier is stored in Redis and retrieved during callback
pub async fn google_oauth_url(
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.google_oauth_url().await {
        Ok((url, _csrf_token)) => {
            // PKCE verifier is now stored in Redis by the auth service
            // The URL contains the state parameter (csrf_token) for callback validation
            let response = crate::models::api::user::GoogleOAuthUrlResponse { url };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to generate OAuth URL: {}", e);
            Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Google OAuth is not configured"
            })))
        }
    }
}

/// POST /backend/public/auth/google/callback
/// Handle Google OAuth callback with authorization code and state
/// Retrieves PKCE verifier from Redis using state parameter
pub async fn google_oauth_callback(
    auth_service: web::Data<AuthService>,
    payload: web::Json<crate::models::api::user::GoogleOAuthCallbackRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract state parameter - required for PKCE verifier retrieval
    let state = payload.state.clone().ok_or_else(|| {
        actix_web::error::ErrorBadRequest("Missing state parameter")
    })?;

    match auth_service
        .google_oauth_callback(payload.code.clone(), state)
        .await
    {
        Ok(auth_response) => Ok(HttpResponse::Ok().json(auth_response)),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Invalid or expired OAuth state") {
                log::warn!("OAuth callback failed - invalid/expired state: {}", e);
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "OAuth state expired or invalid. Please try again."
                })))
            } else {
                log::error!("OAuth callback failed: {}", e);
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "OAuth authentication failed"
                })))
            }
        }
    }
}
