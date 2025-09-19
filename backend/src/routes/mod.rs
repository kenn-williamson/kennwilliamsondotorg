pub mod auth;
pub mod incident_timers;
pub mod health;
pub mod phrases;
pub mod admin;

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
                        .service(
                            web::resource("/profile")
                                .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                                .route(web::put().to(auth::update_profile))
                        )
                        .service(
                            web::resource("/change-password")
                                .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                                .route(web::put().to(auth::change_password))
                        )
                )
                // Public incident timer endpoint
                .route("/{user_slug}/incident-timer", web::get().to(incident_timers::get_latest_by_user_slug))
                // Public phrase endpoint (tied to user slug)
                .route("/{user_slug}/phrase", web::get().to(phrases::get_random_phrase_for_user))
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
                // Protected phrase endpoints
                .service(
                    web::scope("/phrases")
                        .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                        .service(
                            web::resource("")
                                .route(web::get().to(phrases::get_user_phrases))
                        )
                        .service(
                            web::resource("/user")
                                .route(web::get().to(phrases::get_user_phrases_with_exclusions))
                        )
                        .service(
                            web::resource("/random")
                                .route(web::get().to(phrases::get_random_phrase_for_auth_user))
                        )
                        .service(
                            web::resource("/exclude/{id}")
                                .route(web::post().to(phrases::exclude_phrase))
                                .route(web::delete().to(phrases::remove_phrase_exclusion))
                        )
                        .service(
                            web::resource("/excluded")
                                .route(web::get().to(phrases::get_excluded_phrases))
                        )
                        .service(
                            web::resource("/suggestions")
                                .route(web::get().to(phrases::get_user_suggestions))
                                .route(web::post().to(phrases::submit_suggestion))
                        )
                )
                // Admin endpoints with admin role middleware
                .service(
                    web::scope("/admin")
                        .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                        .wrap(actix_web::middleware::from_fn(middleware::admin::admin_auth_middleware))
                        // System stats
                        .route("/stats", web::get().to(admin::get_system_stats))
                        // User management
                        .route("/users", web::get().to(admin::get_users))
                        .service(
                            web::resource("/users/{id}/deactivate")
                                .route(web::post().to(admin::deactivate_user))
                        )
                        .service(
                            web::resource("/users/{id}/activate")
                                .route(web::post().to(admin::activate_user))
                        )
                        .service(
                            web::resource("/users/{id}/reset-password")
                                .route(web::post().to(admin::reset_user_password))
                        )
                        .service(
                            web::resource("/users/{id}/promote")
                                .route(web::post().to(admin::promote_user_to_admin))
                        )
                        // Phrase management
                        .service(
                            web::resource("/phrases")
                                .route(web::get().to(admin::get_all_phrases))
                                .route(web::post().to(admin::create_phrase))
                        )
                        .service(
                            web::resource("/phrases/{id}")
                                .route(web::put().to(admin::update_phrase))
                                .route(web::delete().to(admin::deactivate_phrase))
                        )
                        // Phrase suggestions (new endpoints)
                        .route("/suggestions", web::get().to(admin::get_pending_suggestions_new))
                        .service(
                            web::resource("/suggestions/{id}/approve")
                                .route(web::post().to(admin::approve_suggestion_new))
                        )
                        .service(
                            web::resource("/suggestions/{id}/reject")
                                .route(web::post().to(admin::reject_suggestion_new))
                        )
                        // Legacy suggestion endpoints (keep for compatibility)
                        .service(
                            web::resource("/suggestions-legacy")
                                .route(web::get().to(admin::get_pending_suggestions))
                        )
                        .service(
                            web::resource("/suggestions-legacy/{id}/approve")
                                .route(web::post().to(admin::approve_suggestion))
                        )
                        .service(
                            web::resource("/suggestions-legacy/{id}/reject")
                                .route(web::post().to(admin::reject_suggestion))
                        )
                )
        );
}