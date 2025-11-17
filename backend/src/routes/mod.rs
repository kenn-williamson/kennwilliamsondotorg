pub mod access_request;
pub mod admin;
pub mod auth;
pub mod blog;
pub mod health;
pub mod incident_timers;
pub mod phrases;
pub mod webhooks;

use crate::middleware;
use crate::middleware::rate_limiter::{admin_rate_limit_middleware, rate_limit_middleware};
use actix_web::web;

pub fn configure_app_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Backend API routes with base public/protected grouping
        .service(
            web::scope("/backend")
                // Webhook routes (no auth, no rate limiting - AWS SNS uses signature verification)
                .configure(webhooks::configure_webhook_routes)
                // Public routes (with rate limiting only)
                .service(
                    web::scope("/public")
                        .wrap(actix_web::middleware::from_fn(rate_limit_middleware))
                        .route("/health", web::get().to(health::health))
                        .route("/health/db", web::get().to(health::health_db))
                        .route("/auth/register", web::post().to(auth::register))
                        .route("/auth/login", web::post().to(auth::login))
                        .route("/auth/preview-slug", web::post().to(auth::preview_slug))
                        .route("/auth/refresh", web::post().to(auth::refresh))
                        .route(
                            "/auth/verify-email",
                            web::get().to(auth::verify_email_handler),
                        )
                        .route("/auth/google/url", web::get().to(auth::google_oauth_url))
                        .route(
                            "/auth/google/callback",
                            web::post().to(auth::google_oauth_callback),
                        )
                        .route(
                            "/auth/forgot-password",
                            web::post().to(auth::forgot_password),
                        )
                        .route("/auth/reset-password", web::post().to(auth::reset_password))
                        .route(
                            "/{user_slug}/incident-timer",
                            web::get().to(incident_timers::get_latest_by_user_slug),
                        )
                        .route(
                            "/{user_slug}/phrase",
                            web::get().to(phrases::get_random_phrase_for_user),
                        )
                        .route("/public-timers", web::get().to(auth::get_public_timer_list))
                        // Blog public routes
                        .service(
                            web::scope("/blog")
                                .route("/posts", web::get().to(blog::get_published_posts))
                                .route("/posts/{slug}", web::get().to(blog::get_post_by_slug))
                                .route("/tags", web::get().to(blog::get_all_tags))
                                .route("/search", web::get().to(blog::search_posts)),
                        ),
                )
                // Protected routes (with auth and rate limiting middleware)
                .service(
                    web::scope("/protected")
                        .wrap(actix_web::middleware::from_fn(
                            middleware::auth::jwt_auth_middleware,
                        ))
                        .wrap(actix_web::middleware::from_fn(rate_limit_middleware))
                        .service(
                            web::scope("/auth")
                                .route("/me", web::get().to(auth::get_current_user))
                                .route("/revoke", web::post().to(auth::revoke))
                                .route("/revoke-all", web::post().to(auth::revoke_all))
                                .route("/profile", web::put().to(auth::update_profile))
                                .route("/change-password", web::put().to(auth::change_password))
                                .route("/set-password", web::put().to(auth::set_password))
                                .route("/validate-slug", web::get().to(auth::validate_slug))
                                .route("/delete-account", web::delete().to(auth::delete_account))
                                .route("/export-data", web::get().to(auth::export_data))
                                .route("/preferences", web::put().to(auth::update_preferences))
                                .route(
                                    "/send-verification",
                                    web::post().to(auth::send_verification_email_handler),
                                ),
                        )
                        .service(
                            web::scope("/incident-timers")
                                .route("", web::get().to(incident_timers::get_user_timers))
                                .route("", web::post().to(incident_timers::create_timer))
                                .route("/{id}", web::put().to(incident_timers::update_timer))
                                .route("/{id}", web::delete().to(incident_timers::delete_timer)),
                        )
                        .service(
                            web::scope("/phrases")
                                .route("", web::get().to(phrases::get_user_phrases))
                                .route(
                                    "/user",
                                    web::get().to(phrases::get_user_phrases_with_exclusions),
                                )
                                .route(
                                    "/random",
                                    web::get().to(phrases::get_random_phrase_for_auth_user),
                                )
                                .route("/exclude/{id}", web::post().to(phrases::exclude_phrase))
                                .route(
                                    "/exclude/{id}",
                                    web::delete().to(phrases::remove_phrase_exclusion),
                                )
                                .route("/excluded", web::get().to(phrases::get_excluded_phrases))
                                .route("/suggestions", web::get().to(phrases::get_user_suggestions))
                                .route("/suggestions", web::post().to(phrases::submit_suggestion)),
                        )
                        .service(
                            web::scope("/access-requests")
                                .route("", web::post().to(access_request::create_access_request)),
                        )
                        // Admin routes (with admin middleware - requires JWT first)
                        .service(
                            web::scope("/admin")
                                .wrap(actix_web::middleware::from_fn(
                                    middleware::admin::admin_auth_middleware,
                                ))
                                .wrap(actix_web::middleware::from_fn(admin_rate_limit_middleware))
                                .route("/stats", web::get().to(admin::get_system_stats))
                                .route("/users", web::get().to(admin::get_users))
                                .service(
                                    web::resource("/users/{id}/deactivate")
                                        .route(web::post().to(admin::deactivate_user)),
                                )
                                .service(
                                    web::resource("/users/{id}/activate")
                                        .route(web::post().to(admin::activate_user)),
                                )
                                .service(
                                    web::resource("/users/{id}/reset-password")
                                        .route(web::post().to(admin::reset_user_password)),
                                )
                                .service(
                                    web::resource("/users/{id}/promote")
                                        .route(web::post().to(admin::promote_user_to_admin)),
                                )
                                .service(
                                    web::resource("/users/{id}/roles/{role_name}")
                                        .route(web::post().to(admin::add_user_role))
                                        .route(web::delete().to(admin::remove_user_role)),
                                )
                                .service(
                                    web::resource("/phrases")
                                        .route(web::get().to(admin::get_phrases))
                                        .route(web::post().to(admin::create_phrase)),
                                )
                                .service(
                                    web::resource("/phrases/{id}")
                                        .route(web::put().to(admin::update_phrase))
                                        .route(web::delete().to(admin::deactivate_phrase)),
                                )
                                .route(
                                    "/suggestions",
                                    web::get().to(admin::get_pending_suggestions),
                                )
                                .service(
                                    web::resource("/suggestions/{id}/approve")
                                        .route(web::post().to(admin::approve_suggestion)),
                                )
                                .service(
                                    web::resource("/suggestions/{id}/reject")
                                        .route(web::post().to(admin::reject_suggestion)),
                                )
                                .route(
                                    "/access-requests",
                                    web::get().to(admin::get_pending_access_requests),
                                )
                                .service(
                                    web::resource("/access-requests/{id}/approve")
                                        .route(web::post().to(admin::approve_access_request)),
                                )
                                .service(
                                    web::resource("/access-requests/{id}/reject")
                                        .route(web::post().to(admin::reject_access_request)),
                                )
                                // Blog admin routes
                                .service(
                                    web::scope("/blog")
                                        .route("/posts", web::post().to(blog::create_post))
                                        .route("/posts/{id}", web::put().to(blog::update_post))
                                        .route("/posts/{id}", web::delete().to(blog::delete_post)),
                                ),
                        ),
                ),
        );
}
