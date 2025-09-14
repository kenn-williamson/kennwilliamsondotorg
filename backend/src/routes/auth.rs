use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result as ActixResult};
use uuid::Uuid;
use serde_json::json;

use crate::models::api::{CreateUserRequest, LoginRequest, SlugPreviewRequest, RefreshTokenRequest, RevokeTokenRequest};
use crate::services::auth::AuthService;

/// Extract device information from HTTP request headers
/// Handles forwarded headers from proxies/load balancers using Actix Web's built-in support
fn extract_device_info(req: &HttpRequest) -> Option<serde_json::Value> {
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");
    
    // Extract real IP address using Actix Web's built-in forwarded header support
    // This handles Forwarded, X-Forwarded-For, and X-Real-IP headers automatically
    let ip_address = req.connection_info().realip_remote_addr()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| {
            // Fallback to direct connection IP if no forwarded headers
            req.connection_info().peer_addr()
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
    match auth_service.register(data.into_inner(), device_info).await {
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

