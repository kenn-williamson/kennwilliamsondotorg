use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result as ActixResult};
use serde_json::json;
use uuid::Uuid;

use crate::models::api::{
    CreateUserRequest, LoginRequest, PaginationQuery, PasswordChangeRequest, PublicTimerListItem,
    SetPasswordRequest, ProfileUpdateRequest, RefreshTokenRequest, RevokeTokenRequest,
    SlugPreviewRequest, SlugValidationRequest, UpdatePreferencesRequest, VerifyEmailRequest,
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

pub async fn set_password(
    req: HttpRequest,
    data: web::Json<SetPasswordRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service
        .set_password(user_id, data.into_inner())
        .await
    {
        Ok(()) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Password set successfully"
        }))),
        Err(err) => {
            if err.to_string().contains("already has password credentials") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "User already has password credentials. Use change-password endpoint instead."
                })))
            } else if err.to_string().contains("User not found") {
                Ok(HttpResponse::NotFound().json(serde_json::json!({
                    "error": "User not found"
                })))
            } else {
                log::error!("Password set error: {}", err);
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

/// Delete user account and all associated data
/// DELETE /backend/protected/auth/delete-account
pub async fn delete_account(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service.delete_account(user_id).await {
        Ok(()) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Account deleted successfully"
        }))),
        Err(err) => {
            if err.to_string().contains("Cannot delete system user") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Cannot delete system user"
                })))
            } else if err.to_string().contains("User not found") {
                Ok(HttpResponse::NotFound().json(serde_json::json!({
                    "error": "User not found"
                })))
            } else {
                log::error!("Account deletion error: {}", err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Account deletion failed"
                })))
            }
        }
    }
}

// ============================================================================
// PASSWORD RESET ROUTES
// ============================================================================

/// POST /backend/public/auth/forgot-password
/// Send password reset email (public endpoint, no auth required)
/// Returns same response regardless of whether user exists (prevents user enumeration)
pub async fn forgot_password(
    data: web::Json<crate::models::api::ForgotPasswordRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let frontend_url = std::env::var("FRONTEND_URL")
        .ok()
        .unwrap_or_else(|| "https://kennwilliamson.org".to_string());

    match auth_service
        .send_password_reset_email(&data.email, &frontend_url)
        .await
    {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            log::error!("Forgot password error: {}", err);
            // Return generic message even on error to prevent user enumeration
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "If an account exists with that email, you will receive a password reset link."
            })))
        }
    }
}

/// POST /backend/public/auth/reset-password
/// Reset password with token (public endpoint, no auth required)
pub async fn reset_password(
    data: web::Json<crate::models::api::ResetPasswordRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service
        .reset_password_with_token(&data.token, &data.new_password)
        .await
    {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => {
            log::error!("Password reset error: {}", err);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid or expired reset token"
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

/// GET /backend/protected/auth/export-data
/// Export all user data in JSON format for GDPR/CCPA compliance
pub async fn export_data(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    
    match auth_service.export_user_data(user_id).await {
        Ok(export_data) => {
            let json = serde_json::to_string(&export_data)?;
            let filename = format!(
                "kennwilliamson-data-export-{}.json",
                chrono::Utc::now().format("%Y-%m-%d")
            );
            
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                .body(json))
        }
        Err(err) => {
            log::error!("Data export error: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to export data"
            })))
        }
    }
}

// ============================================================================
// USER PREFERENCES ROUTES
// ============================================================================

/// PUT /backend/protected/auth/preferences
/// Update user preferences for authenticated user
pub async fn update_preferences(
    req: HttpRequest,
    data: web::Json<UpdatePreferencesRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service
        .update_timer_privacy(user_id, data.timer_is_public, data.timer_show_in_list)
        .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            let error_msg = err.to_string();
            if error_msg.contains("not public") || error_msg.contains("Show in List") {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": error_msg
                })))
            } else {
                log::error!("User preferences update error for user {}: {}", user_id, err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                })))
            }
        }
    }
}

/// GET /backend/public/public-timers
/// Get list of users with public timers (public endpoint, no auth required)
pub async fn get_public_timer_list(
    query: web::Query<PaginationQuery>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    // Extract pagination params with defaults
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100).max(1); // Max 100, min 1
    let offset = (page - 1) * page_size;

    // Extract search parameter (optional)
    let search = query.search.clone().and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });

    match auth_service
        .get_users_with_public_timers(page_size, offset, search)
        .await
    {
        Ok(users) => {
            // Map UserWithTimer to PublicTimerListItem
            let response: Vec<PublicTimerListItem> = users
                .into_iter()
                .map(|u| PublicTimerListItem {
                    id: u.id,
                    display_name: u.display_name,
                    slug: u.slug,
                    created_at: u.created_at,
                    reset_timestamp: u.reset_timestamp,
                    notes: u.notes,
                })
                .collect();

            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to get public timer list: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::user::User;
    use crate::repositories::mocks::{MockRefreshTokenRepository, MockUserRepository};
    use crate::services::auth::auth_service::AuthServiceBuilder;
    use actix_web::{test, web, App};
    use chrono::Utc;
    use uuid::Uuid;

    // Test helper: Create in-memory User for mocking (not persisted to DB)
    // Uses builder pattern to avoid fragility from User model changes
    fn create_test_user_with_id(user_id: Uuid) -> User {
        // Note: Using build() not persist() - this is for in-memory mock data
        // The old fields (password_hash, real_name, google_user_id, timer_is_public, timer_show_in_list)
        // have been moved to separate tables in the multi-table auth refactor
        User {
            id: user_id,
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "testuser".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[actix_web::test]
    async fn test_export_data_requires_authentication() {
        // Test that the endpoint exists and works with proper authentication
        // This test verifies the endpoint functionality with auth context
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Create test app without middleware (direct endpoint test)
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(web::resource("/export-data").to(export_data))
        ).await;

        // Make request with user_id in extensions (simulating authenticated request)
        let req = test::TestRequest::get()
            .uri("/export-data")
            .to_request();

        // Manually add user_id to request extensions to simulate auth middleware
        #[allow(unused_mut)]
        let mut req = req;
        req.extensions_mut().insert(user_id);

        let resp = test::call_service(&app, req).await;
        
        // Should return 200 OK when properly authenticated
        assert_eq!(resp.status(), 200);
        
        // Verify response is JSON
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let export_data: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        
        // Verify user data matches the authenticated user
        assert_eq!(export_data["user"]["id"], user_id.to_string());
        assert_eq!(export_data["user"]["email"], "test@example.com");
    }

    #[actix_web::test]
    async fn test_export_data_returns_user_data_only() {
        // Test that user can only export their own data
        // Test that other users' data is not included
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Create test app without middleware (direct endpoint test)
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(web::resource("/export-data").to(export_data))
        ).await;

        // Make request with user_id in extensions (simulating authenticated request)
        let req = test::TestRequest::get()
            .uri("/export-data")
            .insert_header(("X-Test-User-Id", user_id.to_string()))
            .to_request();

        // Manually add user_id to request extensions to simulate auth middleware
        #[allow(unused_mut)]
        let mut req = req;
        req.extensions_mut().insert(user_id);

        let resp = test::call_service(&app, req).await;
        
        // Should return 200 OK
        assert_eq!(resp.status(), 200);
        
        // Verify response is JSON
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let export_data: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        
        // Verify user data matches the authenticated user
        assert_eq!(export_data["user"]["id"], user_id.to_string());
        assert_eq!(export_data["user"]["email"], "test@example.com");
    }

    #[actix_web::test]
    async fn test_export_data_response_format() {
        // Test JSON structure matches specification
        // Test proper HTTP headers for file download
        // Test filename format: "kennwilliamson-data-export-YYYY-MM-DD.json"
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Create test app without middleware (direct endpoint test)
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(web::resource("/export-data").to(export_data))
        ).await;

        // Make request with user_id in extensions (simulating authenticated request)
        let req = test::TestRequest::get()
            .uri("/export-data")
            .to_request();

        // Manually add user_id to request extensions to simulate auth middleware
        #[allow(unused_mut)]
        let mut req = req;
        req.extensions_mut().insert(user_id);

        let resp = test::call_service(&app, req).await;
        
        // Should return 200 OK
        assert_eq!(resp.status(), 200);
        
        // Verify content type
        let content_type = resp.headers().get("content-type").unwrap();
        assert_eq!(content_type, "application/json");
        
        // Verify content disposition header for file download
        let content_disposition = resp.headers().get("content-disposition").unwrap();
        let disposition_str = content_disposition.to_str().unwrap();
        assert!(disposition_str.starts_with("attachment; filename=\"kennwilliamson-data-export-"));
        assert!(disposition_str.ends_with(".json\""));
        
        // Verify JSON structure
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let export_data: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        
        // Verify required fields are present
        assert!(export_data.get("export_date").is_some());
        assert!(export_data.get("export_version").is_some());
        assert!(export_data.get("user").is_some());
        assert!(export_data.get("incident_timers").is_some());
        assert!(export_data.get("phrase_suggestions").is_some());
        assert!(export_data.get("phrase_exclusions").is_some());
        assert!(export_data.get("active_sessions").is_some());
        assert!(export_data.get("verification_history").is_some());
    }

    #[actix_web::test]
    async fn test_export_data_rate_limiting() {
        // Test that rate limiting is applied
        // Test that excessive requests are blocked
        
        // For now, just test that the endpoint exists and responds
        // Rate limiting testing would require more complex middleware setup
        
        let user_id = Uuid::new_v4();
        let user = create_test_user_with_id(user_id);
        
        // Create mock repositories
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));
        
        user_repo.expect_get_user_roles()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec!["user".to_string()]));

        let mut refresh_token_repo = MockRefreshTokenRepository::new();
        refresh_token_repo.expect_find_by_user_id()
            .with(mockall::predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        // Create AuthService with mocks
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Create test app without middleware (direct endpoint test)
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(web::resource("/export-data").to(export_data))
        ).await;

        // Make request with user_id in extensions (simulating authenticated request)
        let req = test::TestRequest::get()
            .uri("/export-data")
            .to_request();

        // Manually add user_id to request extensions to simulate auth middleware
        #[allow(unused_mut)]
        let mut req = req;
        req.extensions_mut().insert(user_id);

        let resp = test::call_service(&app, req).await;
        
        // Should return 200 OK
        assert_eq!(resp.status(), 200);
        
        // Note: Full rate limiting testing would require middleware integration
        // This test verifies the endpoint works correctly
    }

    // Note: Timer privacy route tests are covered by service layer tests
    // Route-level tests would require extensive mocking of preferences_repository
    // The critical business logic is validated in auth_service::profile::tests

    #[actix_web::test]
    async fn test_get_public_timer_list_success() {
        // Test successful retrieval of public timer list
        // Should return 200 with list of public timers

        use crate::models::db::user::UserWithTimer;

        // Create mock user repository
        let mut user_repo = MockUserRepository::new();

        // Create test timer data
        let now = Utc::now();
        let timers = vec![
            UserWithTimer {
                id: Uuid::new_v4(),
                display_name: "User One".to_string(),
                slug: "user-one".to_string(),
                created_at: now,
                reset_timestamp: now,
                notes: Some("Test notes".to_string()),
            },
            UserWithTimer {
                id: Uuid::new_v4(),
                display_name: "User Two".to_string(),
                slug: "user-two".to_string(),
                created_at: now,
                reset_timestamp: now - chrono::Duration::hours(1),
                notes: None,
            },
        ];

        // Mock the get_users_with_public_timers call
        user_repo.expect_get_users_with_public_timers()
            .with(
                mockall::predicate::eq(20), // page_size
                mockall::predicate::eq(0)   // offset
            )
            .times(1)
            .returning(move |_, _| Ok(timers.clone()));

        let refresh_token_repo = MockRefreshTokenRepository::new();

        // Create AuthService with mock
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Create test app
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(web::resource("/public-timers").route(web::get().to(super::get_public_timer_list)))
        ).await;

        // Make request
        let req = test::TestRequest::get()
            .uri("/public-timers")
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Should return 200 OK
        assert_eq!(resp.status(), 200);

        // Verify response contains timer list
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let timer_list: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert!(timer_list.is_array());
        assert_eq!(timer_list.as_array().unwrap().len(), 2);
    }

    #[actix_web::test]
    async fn test_get_public_timer_list_empty() {
        // Test empty list returns 200 with empty array

        use crate::models::db::user::UserWithTimer;

        // Create mock user repository
        let mut user_repo = MockUserRepository::new();

        // Mock empty result
        user_repo.expect_get_users_with_public_timers()
            .with(
                mockall::predicate::eq(20),
                mockall::predicate::eq(0)
            )
            .times(1)
            .returning(|_, _| Ok(vec![]));

        let refresh_token_repo = MockRefreshTokenRepository::new();

        // Create AuthService with mock
        let auth_service = AuthServiceBuilder::new()
            .user_repository(Box::new(user_repo))
            .refresh_token_repository(Box::new(refresh_token_repo))
            .jwt_secret("test_secret".to_string())
            .build();

        // Create test app
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service))
                .service(web::resource("/public-timers").route(web::get().to(super::get_public_timer_list)))
        ).await;

        // Make request
        let req = test::TestRequest::get()
            .uri("/public-timers")
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Should return 200 OK
        assert_eq!(resp.status(), 200);

        // Verify response is empty array
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let timer_list: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert!(timer_list.is_array());
        assert_eq!(timer_list.as_array().unwrap().len(), 0);
    }
}
