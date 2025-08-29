use actix_web::{test, web, App, HttpResponse, Result};
use sqlx::PgPool;
use std::env;

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

#[actix_web::test]
async fn test_health_endpoint() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .route("/health", web::get().to(health))
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status().as_u16();
    
    assert_eq!(status, 200, "Health check should return 200 OK");
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy", "Health response should indicate healthy status");
}

#[actix_web::test]
async fn test_health_db_endpoint() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .route("/api/health/db", web::get().to(health_db))
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/health/db")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status().as_u16();
    
    assert_eq!(status, 200, "Database health check should return 200 OK");
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy", "Response should indicate healthy status");
    assert_eq!(body["database"], "connected", "Response should indicate database connected");
    assert_eq!(body["service"], "kennwilliamson-backend", "Response should contain service name");
}