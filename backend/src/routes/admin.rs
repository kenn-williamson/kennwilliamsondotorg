use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::api::{
    CreatePhraseRequest, UpdatePhraseRequest,
    PhraseListResponse, UserListResponse, SystemStatsResponse,
    PendingSuggestionsResponse, PasswordResetResponse, UserSearchQuery, AdminActionRequest
};
use crate::services::phrase::PhraseService;
use crate::services::admin::{UserManagementService, StatsService, PhraseModerationService};

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


// Query parameters for admin endpoints
#[derive(serde::Deserialize)]
pub struct AdminPhraseQuery {
    pub include_inactive: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// === NEW ADMIN ENDPOINTS ===

/// Get system statistics (admin only)
pub async fn get_system_stats(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
) -> Result<HttpResponse> {
    match StatsService::get_system_stats(&pool).await {
        Ok(stats) => {
            let response = SystemStatsResponse {
                total_users: stats.total_users,
                active_users: stats.active_users,
                pending_suggestions: stats.pending_suggestions,
                total_phrases: stats.total_phrases,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get system stats: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get system statistics"
            })))
        }
    }
}

/// Get users with search (admin only)
pub async fn get_users(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    query: web::Query<UserSearchQuery>,
) -> Result<HttpResponse> {
    match UserManagementService::get_users(
        &pool,
        query.search.clone(),
        query.limit,
        query.offset,
    ).await {
        Ok(users) => {
            let total = users.len() as i64;
            let response = UserListResponse { users, total };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to get users: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get users"
            })))
        }
    }
}

/// Deactivate user (admin only)
pub async fn deactivate_user(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match UserManagementService::deactivate_user(&pool, user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User deactivated successfully"
        }))),
        Err(e) => {
            log::error!("Failed to deactivate user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to deactivate user"
            })))
        }
    }
}

/// Activate user (admin only)
pub async fn activate_user(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match UserManagementService::activate_user(&pool, user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User activated successfully"
        }))),
        Err(e) => {
            log::error!("Failed to activate user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to activate user"
            })))
        }
    }
}

/// Reset user password (admin only)
pub async fn reset_user_password(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match UserManagementService::reset_user_password(&pool, user_id).await {
        Ok(new_password) => {
            let response = PasswordResetResponse { new_password };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to reset user password: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to reset user password"
            })))
        }
    }
}

/// Promote user to admin (admin only)
pub async fn promote_user_to_admin(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match UserManagementService::promote_to_admin(&pool, user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User promoted to admin successfully"
        }))),
        Err(e) => {
            log::error!("Failed to promote user to admin: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to promote user to admin"
            })))
        }
    }
}

/// Get pending phrase suggestions (admin only)
pub async fn get_pending_suggestions(
    pool: web::Data<PgPool>,
    _req: HttpRequest,
) -> Result<HttpResponse> {
    match PhraseModerationService::get_pending_suggestions(&pool).await {
        Ok(suggestions) => {
            let total = suggestions.len() as i64;
            let response = PendingSuggestionsResponse {
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

/// Approve phrase suggestion (admin only)
pub async fn approve_suggestion(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    request: Option<web::Json<AdminActionRequest>>,
) -> Result<HttpResponse> {
    let admin_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let suggestion_id = path.into_inner();
    let admin_reason = request.and_then(|r| r.admin_reason.clone());

    match PhraseModerationService::approve_suggestion(&pool, suggestion_id, admin_id, admin_reason).await {
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

/// Reject phrase suggestion (admin only)
pub async fn reject_suggestion(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    request: Option<web::Json<AdminActionRequest>>,
) -> Result<HttpResponse> {
    let admin_id = req.extensions().get::<Uuid>().cloned().unwrap();
    let suggestion_id = path.into_inner();
    let admin_reason = request.and_then(|r| r.admin_reason.clone());

    match PhraseModerationService::reject_suggestion(&pool, suggestion_id, admin_id, admin_reason).await {
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
