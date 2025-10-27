use actix_web::{post, web, HttpResponse};

use crate::services::webhooks::SnsHandler;
use crate::services::webhooks::SnsMessage;
use crate::repositories::postgres::postgres_email_suppression_repository::PostgresEmailSuppressionRepository;
use sqlx::PgPool;

/// Handle AWS SNS webhook notifications for SES bounces and complaints
#[post("/webhooks/ses")]
async fn handle_ses_webhook(
    body: web::Json<SnsMessage>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let sns_message = body.into_inner();

    // Create SNS handler with suppression repository
    let suppression_repo = Box::new(PostgresEmailSuppressionRepository::new(pool.get_ref().clone()));
    let handler = SnsHandler::new(suppression_repo);

    // Handle different SNS message types
    match sns_message.message_type.as_str() {
        "SubscriptionConfirmation" => {
            // Auto-confirm SNS subscription
            match handler.handle_subscription_confirmation(&sns_message).await {
                Ok(_) => {
                    log::info!("SNS subscription confirmed for topic: {}", sns_message.topic_arn);
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "subscription_confirmed"
                    }))
                }
                Err(e) => {
                    log::error!("Failed to confirm SNS subscription: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to confirm subscription"
                    }))
                }
            }
        }
        "Notification" => {
            // Handle bounce or complaint notification
            match handler.handle_notification(&sns_message).await {
                Ok(_) => {
                    log::info!("Processed SNS notification: {}", sns_message.message_id);
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "notification_processed"
                    }))
                }
                Err(e) => {
                    log::error!("Failed to process SNS notification: {}", e);
                    // Return 200 OK even on errors to prevent SNS retries
                    // Log the error for investigation
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "notification_received",
                        "note": "Processing error logged"
                    }))
                }
            }
        }
        _ => {
            log::warn!("Unknown SNS message type: {}", sns_message.message_type);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Unknown message type"
            }))
        }
    }
}

/// Configure webhook routes
pub fn configure_webhook_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_ses_webhook);
}
