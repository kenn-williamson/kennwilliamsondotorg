use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "kennwilliamson-backend",
        "version": "0.1.0"
    })))
}

pub async fn health_db(pool: web::Data<PgPool>) -> Result<HttpResponse> {
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
        }))),
    }
}
