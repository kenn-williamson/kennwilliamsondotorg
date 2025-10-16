use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};

use crate::models::api::CreateAccessRequestRequest;
use crate::services::admin::AccessRequestModerationService;

/// Create a new access request (user-facing, requires authentication)
pub async fn create_access_request(
    access_request_moderation_service: web::Data<AccessRequestModerationService>,
    req: HttpRequest,
    request: web::Json<CreateAccessRequestRequest>,
) -> Result<HttpResponse> {
    // Get user ID from auth middleware
    let user_id = req.extensions().get::<uuid::Uuid>().cloned().unwrap();

    // For now, default to requesting trusted-contact role
    // In the future, we could make this configurable in the request
    let requested_role = "trusted-contact".to_string();

    match access_request_moderation_service
        .create_request(user_id, request.message.clone(), requested_role)
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().json(serde_json::json!({
            "message": "Access request created successfully"
        }))),
        Err(e) => {
            log::error!("Failed to create access request: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create access request"
            })))
        }
    }
}
