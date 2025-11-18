use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use backend::routes;
use backend::services;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

async fn request_logging_middleware(
    req: actix_web::dev::ServiceRequest,
    next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
) -> actix_web::Result<
    actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
    actix_web::Error,
> {
    println!("🔍 REQUEST: {} {}", req.method(), req.path());
    next.call(req).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    // Create database connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let container = match env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string())
        .as_str()
    {
        #[cfg(feature = "mocks")]
        "testing" => services::container::ServiceContainer::new_testing(jwt_secret),
        "production" => services::container::ServiceContainer::new_production(
            pool.clone(),
            jwt_secret,
            redis_url.clone(),
        ),
        _ => services::container::ServiceContainer::new_development(
            pool.clone(),
            jwt_secret,
            redis_url.clone(),
        ),
    };

    println!("🚀 Starting server at http://{}:{}", host, port);
    println!("📊 Database connected successfully");
    println!("🚦 Rate limiting service initialized");

    // Spawn background cleanup task (runs every 24 hours by default)
    let cleanup_interval_hours = env::var("CLEANUP_INTERVAL_HOURS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(24);

    let cleanup_service = container.cleanup_service.clone();
    actix_web::rt::spawn(async move {
        let mut interval = actix_web::rt::time::interval(std::time::Duration::from_secs(
            cleanup_interval_hours * 3600,
        ));

        loop {
            interval.tick().await;
            log::info!("Running scheduled token cleanup...");

            match cleanup_service.cleanup_expired_tokens().await {
                Ok(count) => {
                    if count > 0 {
                        log::info!("Cleanup complete: {} expired tokens removed", count);
                    } else {
                        log::debug!("Cleanup complete: no expired tokens found");
                    }
                }
                Err(e) => {
                    log::error!("Token cleanup failed: {}", e);
                }
            }
        }
    });

    println!(
        "🧹 Token cleanup scheduled every {} hours",
        cleanup_interval_hours
    );

    HttpServer::new(move || {
        let cors_origin =
            env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::from_fn(request_logging_middleware))
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(container.auth_service.clone()))
            .app_data(web::Data::from(container.blog_service.clone()))
            .app_data(web::Data::from(container.incident_timer_service.clone()))
            .app_data(web::Data::from(container.phrase_service.clone()))
            .app_data(web::Data::from(container.admin_service.clone()))
            .app_data(web::Data::from(container.phrase_moderation_service.clone()))
            .app_data(web::Data::from(
                container.access_request_moderation_service.clone(),
            ))
            .app_data(web::Data::from(container.stats_service.clone()))
            .app_data(web::Data::from(container.rate_limit_service.clone()))
            .configure(routes::configure_app_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
