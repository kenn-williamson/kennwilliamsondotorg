use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    HttpMessage, Error, Result,
};
use uuid::Uuid;
use std::collections::HashSet;

use crate::services::auth::AuthService;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub roles: HashSet<String>,
}

impl AuthUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
    
    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

pub async fn jwt_auth_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
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
            return Err(actix_web::error::ErrorUnauthorized(
                "Authorization header missing"
            ));
        }
    };

    // Get auth service from app data
    let auth_service = req.app_data::<actix_web::web::Data<AuthService>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Auth service not found"))?;

    // Verify token
    match auth_service.verify_token(token).await {
        Ok(Some(claims)) => {
            // Parse user ID from claims
            let user_id = claims.sub.parse::<Uuid>().map_err(|_| {
                actix_web::error::ErrorUnauthorized("Invalid user ID in token")
            })?;

            // Create AuthUser with roles
            let roles: HashSet<String> = claims.roles.into_iter().collect();
            let auth_user = AuthUser {
                id: user_id,
                email: claims.email,
                roles,
            };

            // Store both user ID and full AuthUser in request extensions
            req.extensions_mut().insert(user_id);
            req.extensions_mut().insert(auth_user);

            // Continue to the handler
            let res = next.call(req).await?;
            Ok(res)
        }
        Ok(None) | Err(_) => {
            Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))
        }
    }
}

// TODO: Add role-based authorization middleware later
// For now, check roles in individual route handlers using AuthUser.has_role()