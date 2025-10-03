use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage, Result,
};
use uuid::Uuid;

use crate::services::admin::UserManagementService;

pub async fn admin_auth_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    log::debug!(
        "Admin middleware called for: {} {}",
        req.method(),
        req.path()
    );

    // First, run JWT auth middleware to get user_id
    let user_id = match req.extensions().get::<Uuid>() {
        Some(id) => *id,
        None => {
            log::debug!("No user_id found in request extensions - JWT middleware must run first");
            return Err(actix_web::error::ErrorUnauthorized(
                "Authentication required",
            ));
        }
    };

    // Get admin service from app data
    let admin_service = req
        .app_data::<actix_web::web::Data<UserManagementService>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Admin service not found"))?;

    // Check if user is admin
    match admin_service.is_user_admin(user_id).await {
        Ok(true) => {
            log::debug!("User {} is admin, allowing access", user_id);
            let res = next.call(req).await?;
            Ok(res)
        }
        Ok(false) => {
            log::debug!("User {} is not admin, denying access", user_id);
            Err(actix_web::error::ErrorForbidden("Admin access required"))
        }
        Err(e) => {
            log::error!("Failed to check admin status for user {}: {}", user_id, e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to verify admin status",
            ))
        }
    }
}
