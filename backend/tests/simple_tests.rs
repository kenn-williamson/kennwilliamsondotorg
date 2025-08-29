use actix_web::{test, web, App, HttpResponse, Result};
use serde_json::json;

// Simple health endpoint for testing
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "test-backend"
    })))
}

#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(
        App::new().route("/health", web::get().to(health))
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_web::test]
async fn test_not_found() {
    let app = test::init_service(
        App::new().route("/health", web::get().to(health))
    ).await;

    let req = test::TestRequest::get()
        .uri("/nonexistent")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}