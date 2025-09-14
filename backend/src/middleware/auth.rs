use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    HttpMessage, Error, Result,
};
use uuid::Uuid;

use crate::services::auth::AuthService;

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
                    "Authorization header must be in format: Bearer <token>"
                ));
            }
        }
        None => {
            log::debug!("No Authorization header found");
            return Err(actix_web::error::ErrorUnauthorized(
                "Authorization header missing"
            ));
        }
    };

    // Get auth service from app data
    let auth_service = req.app_data::<actix_web::web::Data<AuthService>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Auth service not found"))?;

    // Verify token
    log::debug!("Verifying token for request");
    match auth_service.verify_token(token).await {
        Ok(Some(claims)) => {
            // Parse user ID from claims
            let user_id = claims.sub.parse::<Uuid>().map_err(|_| {
                actix_web::error::ErrorUnauthorized("Invalid user ID in token")
            })?;

            log::debug!("Token verified successfully for user: {}", user_id);

            // Store only user ID in request extensions
            // Route handlers can fetch full user details when needed
            req.extensions_mut().insert(user_id);

            // Continue to the handler
            let res = next.call(req).await?;
            Ok(res)
        }
        Ok(None) => {
            log::debug!("Token verification returned None - invalid token");
            Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))
        }
        Err(e) => {
            log::debug!("Token verification failed: {}", e);
            Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))
        }
    }
}

// Note: Route handlers should fetch full user details when needed:
// let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
// let user_details = auth_service.get_current_user(user_id).await?;