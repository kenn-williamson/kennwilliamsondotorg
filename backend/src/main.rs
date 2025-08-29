use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod middleware;
mod models;
mod routes;
mod services;

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "kennwilliamson-backend",
        "version": "0.1.0"
    })))
}

async fn health_db(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "database": "connected",
            "service": "kennwilliamson-backend",
            "version": "0.1.0"
        }))),
        Err(e) => Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy",
            "database": "disconnected",
            "error": e.to_string()
        })))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

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
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(incident_timer_service.clone()))
            .route("/health", web::get().to(health))
            .route("/api/health", web::get().to(health))
            .route("/api/health/db", web::get().to(health_db))
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure_routes)
                    .configure(routes::incident_timers::configure_public_routes)
                    .service(
                        web::scope("")
                            .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                            .configure(routes::incident_timers::configure_protected_routes)
                    )
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
