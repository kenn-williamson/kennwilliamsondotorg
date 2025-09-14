use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result as ActixResult};
use uuid::Uuid;

use crate::models::api::{
    CreateIncidentTimer, IncidentTimerResponse, PublicIncidentTimerResponse, UpdateIncidentTimer,
};
use crate::services::incident_timer::IncidentTimerService;

#[derive(serde::Deserialize)]
pub struct UserSlugPath {
    user_slug: String,
}

#[derive(serde::Deserialize)]
pub struct TimerIdPath {
    id: Uuid,
}

// Public endpoint - get latest timer for user by slug
pub async fn get_latest_by_user_slug(
    path: web::Path<UserSlugPath>,
    service: web::Data<IncidentTimerService>,
) -> ActixResult<HttpResponse> {
    match service.get_latest_by_user_slug(&path.user_slug).await {
        Ok(Some((timer, user_display_name))) => {
            let response = PublicIncidentTimerResponse {
                id: timer.id,
                reset_timestamp: timer.reset_timestamp,
                notes: timer.notes,
                created_at: timer.created_at,
                updated_at: timer.updated_at,
                user_display_name,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "No timer found for this user"
        }))),
        Err(err) => {
            log::error!("Failed to get timer for user slug {}: {}", path.user_slug, err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// Protected endpoint - get all timers for current user
pub async fn get_user_timers(
    req: HttpRequest,
    service: web::Data<IncidentTimerService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match service.get_all_by_user(user_id).await {
        Ok(timers) => {
            let response: Vec<IncidentTimerResponse> = timers.into_iter().map(|t| t.into()).collect();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to get timers for user {}: {}", user_id, err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// Protected endpoint - create new timer
pub async fn create_timer(
    req: HttpRequest,
    data: web::Json<CreateIncidentTimer>,
    service: web::Data<IncidentTimerService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match service.create(user_id, data.into_inner()).await {
        Ok(timer) => {
            let response: IncidentTimerResponse = timer.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(err) => {
            log::error!("Failed to create timer for user {}: {}", user_id, err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// Protected endpoint - update timer
pub async fn update_timer(
    path: web::Path<TimerIdPath>,
    req: HttpRequest,
    data: web::Json<UpdateIncidentTimer>,
    service: web::Data<IncidentTimerService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match service.update(path.id, user_id, data.into_inner()).await {
        Ok(Some(timer)) => {
            let response: IncidentTimerResponse = timer.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Timer not found"
        }))),
        Err(err) => {
            log::error!("Failed to update timer {} for user {}: {}", path.id, user_id, err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

// Protected endpoint - delete timer
pub async fn delete_timer(
    path: web::Path<TimerIdPath>,
    req: HttpRequest,
    service: web::Data<IncidentTimerService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match service.delete(path.id, user_id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Timer not found"
        }))),
        Err(err) => {
            log::error!("Failed to delete timer {} for user {}: {}", path.id, user_id, err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

