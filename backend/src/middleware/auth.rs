use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage, Result,
};
use uuid::Uuid;

use crate::services::auth::AuthService;

/// Authentication context containing user ID and roles from JWT
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub roles: Vec<String>,
}

impl AuthContext {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Require a specific role, returning 403 Forbidden if not present
    pub fn require_role(&self, role: &str) -> Result<(), actix_web::Error> {
        if self.has_role(role) {
            Ok(())
        } else {
            match role {
                "email-verified" => Err(actix_web::error::ErrorForbidden(
                    "Email not verified. Please check your inbox to verify your email address.",
                )),
                "admin" => Err(actix_web::error::ErrorForbidden("Admin access required")),
                _ => Err(actix_web::error::ErrorForbidden(format!(
                    "Required role '{}' not found",
                    role
                ))),
            }
        }
    }
}

pub async fn jwt_auth_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    log::debug!("JWT middleware called for: {} {}", req.method(), req.path());

    // Extract token from Authorization header
    let token = match req.headers().get("Authorization") {
        Some(auth_header) => {
            let auth_str = auth_header.to_str().map_err(|_| {
                actix_web::error::ErrorUnauthorized("Invalid authorization header encoding")
            })?;

            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                token
            } else {
                return Err(actix_web::error::ErrorUnauthorized(
                    "Authorization header must be in format: Bearer <token>",
                ));
            }
        }
        None => {
            log::debug!("No Authorization header found");
            return Err(actix_web::error::ErrorUnauthorized(
                "Authorization header missing",
            ));
        }
    };

    // Get auth service from app data
    let auth_service = req
        .app_data::<actix_web::web::Data<AuthService>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Auth service not found"))?;

    // Verify token
    log::debug!("Verifying token for request");
    match auth_service.verify_token(token).await {
        Ok(Some(claims)) => {
            // Parse user ID from claims
            let user_id = claims
                .sub
                .parse::<Uuid>()
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user ID in token"))?;

            log::debug!(
                "Token verified successfully for user: {} with roles: {:?}",
                user_id,
                claims.roles
            );

            // Store AuthContext with user ID and roles in request extensions
            let auth_context = AuthContext {
                user_id,
                roles: claims.roles,
            };
            req.extensions_mut().insert(auth_context.clone());

            // Also insert Uuid for backward compatibility with existing route handlers
            req.extensions_mut().insert(user_id);

            // Continue to the handler
            let res = next.call(req).await?;
            Ok(res)
        }
        Ok(None) => {
            log::debug!("Token verification returned None - invalid token");
            Err(actix_web::error::ErrorUnauthorized(
                "Invalid or expired token",
            ))
        }
        Err(e) => {
            log::debug!("Token verification failed: {}", e);
            Err(actix_web::error::ErrorUnauthorized(
                "Invalid or expired token",
            ))
        }
    }
}

// Note: Route handlers can access authentication context:
// let auth_ctx = req.extensions().get::<AuthContext>().cloned().unwrap();
// let user_id = auth_ctx.user_id;
// if auth_ctx.has_role("admin") { ... }
//
// For backward compatibility, user_id can still be accessed directly:
// let user_id = req.extensions().get::<AuthContext>().map(|ctx| ctx.user_id).unwrap();
