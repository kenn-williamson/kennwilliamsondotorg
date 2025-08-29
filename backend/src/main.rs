use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

mod models;

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

    println!("🚀 Starting server at http://{}:{}", host, port);
    println!("📊 Database connected successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
            .route("/api/health", web::get().to(health))
            .route("/api/health/db", web::get().to(health_db))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
