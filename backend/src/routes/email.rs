use actix_web::{HttpResponse, Result as ActixResult, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::auth::AuthService;

/// Request body for unsubscribe
#[derive(Debug, Deserialize)]
pub struct UnsubscribeRequest {
    pub token: String,
}

/// Response for unsubscribe
#[derive(Debug, Serialize)]
pub struct UnsubscribeResponse {
    pub success: bool,
    pub message: String,
    pub email_type: Option<String>,
}

/// POST /backend/public/email/unsubscribe
///
/// Unsubscribes a user from email notifications using a token.
/// The token is validated and the corresponding preference is disabled.
pub async fn unsubscribe(
    body: web::Json<UnsubscribeRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    match auth_service.unsubscribe_by_token(&body.token).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UnsubscribeResponse {
            success: true,
            message: "Successfully unsubscribed".to_string(),
            email_type: Some(result.email_type),
        })),
        Err(e) => {
            let error_msg = e.to_string();

            // Determine the appropriate HTTP status code
            if error_msg.contains("Invalid") {
                Ok(HttpResponse::BadRequest().json(UnsubscribeResponse {
                    success: false,
                    message: error_msg,
                    email_type: None,
                }))
            } else if error_msg.contains("not found") {
                Ok(HttpResponse::NotFound().json(UnsubscribeResponse {
                    success: false,
                    message: error_msg,
                    email_type: None,
                }))
            } else {
                log::error!("Unsubscribe error: {}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to process unsubscribe request"
                })))
            }
        }
    }
}

/// GET /backend/public/email/unsubscribe/{token}
///
/// Validate an unsubscribe token without actually unsubscribing.
/// Used by the frontend to display the unsubscribe confirmation page.
pub async fn validate_unsubscribe_token(
    path: web::Path<String>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let token = path.into_inner();

    match auth_service.validate_unsubscribe_token(&token).await {
        Ok(Some(email_type)) => Ok(HttpResponse::Ok().json(json!({
            "valid": true,
            "email_type": email_type
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "valid": false,
            "message": "Token not found or expired"
        }))),
        Err(e) => {
            let error_msg = e.to_string();

            if error_msg.contains("Invalid") {
                Ok(HttpResponse::BadRequest().json(json!({
                    "valid": false,
                    "message": error_msg
                })))
            } else {
                log::error!("Token validation error: {}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to validate token"
                })))
            }
        }
    }
}
