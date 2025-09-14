pub mod auth;
pub mod incident_timers;
pub mod health;

use actix_web::web;
use crate::middleware;

pub fn configure_app_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Backend API routes with selective middleware application
        .service(
            web::scope("/backend")
                // Public health endpoints
                .route("/health", web::get().to(health::health))
                .route("/health/db", web::get().to(health::health_db))
                // Public auth endpoints
                .service(
                    web::scope("/auth")
                        .route("/register", web::post().to(auth::register))
                        .route("/login", web::post().to(auth::login))
                        .route("/preview-slug", web::post().to(auth::preview_slug))
                        .route("/refresh", web::post().to(auth::refresh))
                        .route("/revoke", web::post().to(auth::revoke))
                        // Protected auth endpoints with middleware
                        .service(
                            web::resource("/me")
                                .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                                .route(web::get().to(auth::get_current_user))
                        )
                        .service(
                            web::resource("/revoke-all")
                                .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                                .route(web::post().to(auth::revoke_all))
                        )
                )
                // Public incident timer endpoint
                .route("/{user_slug}/incident-timer", web::get().to(incident_timers::get_latest_by_user_slug))
                // Protected incident timer endpoints with shared middleware
                .service(
                    web::scope("/incident-timers")
                        .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                        .service(
                            web::resource("")
                                .route(web::get().to(incident_timers::get_user_timers))
                                .route(web::post().to(incident_timers::create_timer))
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::put().to(incident_timers::update_timer))
                                .route(web::delete().to(incident_timers::delete_timer))
                        )
                )
        );
}