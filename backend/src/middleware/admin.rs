use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage, Result,
};

use super::auth::AuthContext;

pub async fn admin_auth_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    log::debug!(
        "Admin middleware called for: {} {}",
        req.method(),
        req.path()
    );

    // Get auth context from JWT middleware
    let auth_ctx = match req.extensions().get::<AuthContext>() {
        Some(ctx) => ctx.clone(),
        None => {
            log::debug!(
                "No auth context found in request extensions - JWT middleware must run first"
            );
            return Err(actix_web::error::ErrorUnauthorized(
                "Authentication required",
            ));
        }
    };

    // Check if user has admin role
    if !auth_ctx.has_role("admin") {
        log::debug!(
            "User {} does not have admin role, denying access",
            auth_ctx.user_id
        );
        return Err(actix_web::error::ErrorForbidden("Admin access required"));
    }

    log::debug!("User {} is admin, allowing access", auth_ctx.user_id);
    let res = next.call(req).await?;
    Ok(res)
}
