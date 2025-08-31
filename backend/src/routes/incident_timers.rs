use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result as ActixResult};
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::models::incident_timer::{
    CreateIncidentTimer, IncidentTimerResponse, UpdateIncidentTimer,
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
        Ok(Some(timer)) => {
            let response: IncidentTimerResponse = timer.into();
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
    let auth_user = req.extensions().get::<AuthUser>().cloned().unwrap();
    match service.get_all_by_user(auth_user.id).await {
        Ok(timers) => {
            let response: Vec<IncidentTimerResponse> = timers.into_iter().map(|t| t.into()).collect();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            log::error!("Failed to get timers for user {}: {}", auth_user.id, err);
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
    let auth_user = req.extensions().get::<AuthUser>().cloned().unwrap();
    match service.create(auth_user.id, data.into_inner()).await {
        Ok(timer) => {
            let response: IncidentTimerResponse = timer.into();
            Ok(HttpResponse::Created().json(response))
        }
        Err(err) => {
            log::error!("Failed to create timer for user {}: {}", auth_user.id, err);
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
    let auth_user = req.extensions().get::<AuthUser>().cloned().unwrap();
    match service.update(path.id, auth_user.id, data.into_inner()).await {
        Ok(Some(timer)) => {
            let response: IncidentTimerResponse = timer.into();
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Timer not found"
        }))),
        Err(err) => {
            log::error!("Failed to update timer {} for user {}: {}", path.id, auth_user.id, err);
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
    let auth_user = req.extensions().get::<AuthUser>().cloned().unwrap();
    match service.delete(path.id, auth_user.id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Timer not found"
        }))),
        Err(err) => {
            log::error!("Failed to delete timer {} for user {}: {}", path.id, auth_user.id, err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}

pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring public incident timer routes");
    cfg.route("/{user_slug}/incident-timers", web::get().to(get_latest_by_user_slug)); // Public
}

pub fn configure_protected_routes(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring protected incident timer routes");
    cfg.service(
        web::resource("/incident-timers")
            .route(web::get().to(get_user_timers))    // GET /api/incident-timers
            .route(web::post().to(create_timer))      // POST /api/incident-timers
    )
    .service(
        web::resource("/incident-timers/{id}")
            .route(web::put().to(update_timer))       // PUT /api/incident-timers/{id}
            .route(web::delete().to(delete_timer))    // DELETE /api/incident-timers/{id}
    );
}