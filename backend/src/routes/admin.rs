use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::api::{
    CreatePhraseRequest, UpdatePhraseRequest, AdminSuggestionActionRequest,
    PhraseListResponse, SuggestionListResponse
};
use crate::services::phrase::PhraseService;

/// Get all phrases (admin only)
pub async fn get_all_phrases(
    pool: web::Data<PgPool>,
    _req: HttpRequest, // Middleware ensures admin role
    query: web::Query<AdminPhraseQuery>,
) -> Result<HttpResponse> {
    let include_inactive = query.include_inactive.unwrap_or(false);
    let limit = query.limit;
    let offset = query.offset;

    match PhraseService::get_all_phrases(&pool, include_inactive, limit, offset).await {
        Ok(phrases) => {
            let total = phrases.len() as i64;
            let response = PhraseListResponse {
                phrases,
                total,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get all phrases: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get all phrases"
            })))
        }
    }
}

/// Create a new phrase (admin only)
pub async fn create_phrase(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    request: web::Json<CreatePhraseRequest>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match PhraseService::create_phrase(&pool, request.into_inner(), user_id).await {
        Ok(phrase) => Ok(HttpResponse::Created().json(phrase)),
        Err(e) => {
            log::error!("Failed to create phrase: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create phrase"
            })))
        }
    }
}

/// Update a phrase (admin only)
pub async fn update_phrase(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    path: web::Path<Uuid>,
    request: web::Json<UpdatePhraseRequest>,
) -> Result<HttpResponse> {
    let phrase_id = path.into_inner();

    match PhraseService::update_phrase(&pool, phrase_id, request.into_inner()).await {
        Ok(phrase) => Ok(HttpResponse::Ok().json(phrase)),
        Err(e) => {
            log::error!("Failed to update phrase: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update phrase"
            })))
        }
    }
}

/// Deactivate a phrase (admin only)
pub async fn deactivate_phrase(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let phrase_id = path.into_inner();

    let request = UpdatePhraseRequest {
        phrase_text: None,
        active: Some(false),
    };

    match PhraseService::update_phrase(&pool, phrase_id, request).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Phrase deactivated successfully"
        }))),
        Err(e) => {
            log::error!("Failed to deactivate phrase: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to deactivate phrase"
            })))
        }
    }
}

/// Get all pending suggestions (admin only)
pub async fn get_pending_suggestions(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
) -> Result<HttpResponse> {
    match PhraseService::get_pending_suggestions(&pool).await {
        Ok(suggestions) => {
            let total = suggestions.len() as i64;
            let response = SuggestionListResponse {
                suggestions: suggestions.into_iter().map(|s| s.into()).collect(),
                total,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get pending suggestions: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get pending suggestions"
            })))
        }
    }
}

/// Approve a phrase suggestion (admin only)
pub async fn approve_suggestion(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    request: Option<web::Json<AdminSuggestionActionRequest>>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let suggestion_id = path.into_inner();
    let admin_reason = request.and_then(|r| r.admin_reason.clone());

    match PhraseService::approve_suggestion(&pool, suggestion_id, user_id, admin_reason).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Suggestion approved successfully"
        }))),
        Err(e) => {
            log::error!("Failed to approve suggestion: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to approve suggestion"
            })))
        }
    }
}

/// Reject a phrase suggestion (admin only)
pub async fn reject_suggestion(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    request: Option<web::Json<AdminSuggestionActionRequest>>,
) -> Result<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let suggestion_id = path.into_inner();
    let admin_reason = request.and_then(|r| r.admin_reason.clone());

    match PhraseService::reject_suggestion(&pool, suggestion_id, user_id, admin_reason).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Suggestion rejected successfully"
        }))),
        Err(e) => {
            log::error!("Failed to reject suggestion: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to reject suggestion"
            })))
        }
    }
}

// Query parameters for admin endpoints
#[derive(serde::Deserialize)]
pub struct AdminPhraseQuery {
    pub include_inactive: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
