use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod middleware;
mod models;
mod routes;
mod services;

async fn request_logging_middleware(
    req: actix_web::dev::ServiceRequest,
    next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
) -> actix_web::Result<actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, actix_web::Error> {
    println!("🔍 REQUEST: {} {}", req.method(), req.path());
    next.call(req).await
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Create services
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    
    let auth_service = services::auth::AuthService::new(pool.clone(), jwt_secret);
    let incident_timer_service = services::incident_timer::IncidentTimerService::new(pool.clone());

    println!("🚀 Starting server at http://{}:{}", host, port);
    println!("📊 Database connected successfully");

    HttpServer::new(move || {
        let cors_origin = env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());
            
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
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(incident_timer_service.clone()))
            .configure(routes::configure_app_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
