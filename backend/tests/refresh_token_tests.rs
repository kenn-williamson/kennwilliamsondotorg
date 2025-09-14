use actix_web::{test, web, App};
use sqlx::PgPool;
use std::env;
use chrono::{Duration, Utc};

use backend::models::api::{CreateUserRequest, LoginRequest, RefreshTokenRequest, RevokeTokenRequest};
use backend::services::auth::AuthService;
use backend::routes;

async fn setup_test_app(pool: PgPool, auth_service: AuthService) -> actix_web::dev::ServiceResponse {
    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service))
            .configure(routes::configure_app_routes)
    ).await
}

async fn cleanup_db(pool: &PgPool) {
    let _ = sqlx::query!("DELETE FROM refresh_tokens").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM user_roles").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(pool).await;
}

async fn create_test_user_and_login(app: &impl actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>, Error = actix_web::Error>) -> (String, String, String) {
    let user_data = CreateUserRequest {
        email: format!("test-refresh-{}@example.com", Utc::now().timestamp_millis()),
        password: "TestPassword123!".to_string(),
        display_name: "Test Refresh User".to_string(),
    };

    // Register user
    let register_req = test::TestRequest::post()
        .uri("/backend/auth/register")
        .set_json(&user_data)
        .to_request();

    let register_resp = test::call_service(app, register_req).await;
    assert_eq!(register_resp.status().as_u16(), 201, "Registration should succeed");

    let register_body: serde_json::Value = test::read_body_json(register_resp).await;
    let access_token = register_body["token"].as_str().unwrap().to_string();
    let refresh_token = register_body["refresh_token"].as_str().unwrap().to_string();

    (user_data.email, access_token, refresh_token)
}

#[actix_web::test]
async fn test_refresh_token_success() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Cleanup
    cleanup_db(&pool).await;

    let app = setup_test_app(pool.clone(), auth_service).await;

    // Create and login user to get refresh token
    let (_, _access_token, refresh_token) = create_test_user_and_login(&app).await;

    // Test refresh token endpoint
    let refresh_data = RefreshTokenRequest {
        refresh_token: refresh_token.clone(),
    };

    let refresh_req = test::TestRequest::post()
        .uri("/backend/auth/refresh")
        .set_json(&refresh_data)
        .to_request();

    let refresh_resp = test::call_service(&app, refresh_req).await;
    let status = refresh_resp.status().as_u16();

    if status != 200 {
        let error_body: serde_json::Value = test::read_body_json(refresh_resp).await;
        panic!("Refresh token failed with status {}: {}", status, error_body);
    }

    let refresh_body: serde_json::Value = test::read_body_json(refresh_resp).await;
    assert!(refresh_body["token"].is_string(), "Response should contain new JWT token");
    assert!(refresh_body["refresh_token"].is_string(), "Response should contain new refresh token");

    // New refresh token should be different from old one (rolling tokens)
    assert_ne!(
        refresh_body["refresh_token"].as_str().unwrap(),
        refresh_token,
        "New refresh token should be different from old one"
    );

    // Old refresh token should no longer work
    let old_refresh_req = test::TestRequest::post()
        .uri("/backend/auth/refresh")
        .set_json(&refresh_data)
        .to_request();

    let old_refresh_resp = test::call_service(&app, old_refresh_req).await;
    assert_eq!(old_refresh_resp.status().as_u16(), 401, "Old refresh token should be invalid");

    // Cleanup
    cleanup_db(&pool).await;
}

#[actix_web::test]
async fn test_refresh_token_invalid() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Cleanup
    cleanup_db(&pool).await;

    let app = setup_test_app(pool.clone(), auth_service).await;

    // Test with invalid refresh token
    let refresh_data = RefreshTokenRequest {
        refresh_token: "invalid_token".to_string(),
    };

    let refresh_req = test::TestRequest::post()
        .uri("/backend/auth/refresh")
        .set_json(&refresh_data)
        .to_request();

    let refresh_resp = test::call_service(&app, refresh_req).await;
    assert_eq!(refresh_resp.status().as_u16(), 401, "Invalid refresh token should return 401");

    let error_body: serde_json::Value = test::read_body_json(refresh_resp).await;
    assert!(error_body["error"].as_str().unwrap().contains("Invalid or expired"),
           "Error message should indicate invalid token");

    // Cleanup
    cleanup_db(&pool).await;
}

#[actix_web::test]
async fn test_revoke_refresh_token() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Cleanup
    cleanup_db(&pool).await;

    let app = setup_test_app(pool.clone(), auth_service).await;

    // Create and login user to get refresh token
    let (_, _access_token, refresh_token) = create_test_user_and_login(&app).await;

    // Revoke the refresh token
    let revoke_data = RevokeTokenRequest {
        refresh_token: refresh_token.clone(),
    };

    let revoke_req = test::TestRequest::post()
        .uri("/backend/auth/revoke")
        .set_json(&revoke_data)
        .to_request();

    let revoke_resp = test::call_service(&app, revoke_req).await;
    assert_eq!(revoke_resp.status().as_u16(), 200, "Token revocation should succeed");

    // Try to use the revoked token
    let refresh_data = RefreshTokenRequest {
        refresh_token: refresh_token,
    };

    let refresh_req = test::TestRequest::post()
        .uri("/backend/auth/refresh")
        .set_json(&refresh_data)
        .to_request();

    let refresh_resp = test::call_service(&app, refresh_req).await;
    assert_eq!(refresh_resp.status().as_u16(), 401, "Revoked token should be invalid");

    // Cleanup
    cleanup_db(&pool).await;
}

#[actix_web::test]
async fn test_register_and_login_include_refresh_tokens() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Cleanup
    cleanup_db(&pool).await;

    let app = setup_test_app(pool.clone(), auth_service).await;

    // Test registration includes refresh token
    let user_data = CreateUserRequest {
        email: format!("test-register-refresh-{}@example.com", Utc::now().timestamp_millis()),
        password: "TestPassword123!".to_string(),
        display_name: "Test Register Refresh".to_string(),
    };

    let register_req = test::TestRequest::post()
        .uri("/backend/auth/register")
        .set_json(&user_data)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status().as_u16(), 201, "Registration should succeed");

    let register_body: serde_json::Value = test::read_body_json(register_resp).await;
    assert!(register_body["token"].is_string(), "Registration should include JWT token");
    assert!(register_body["refresh_token"].is_string(), "Registration should include refresh token");

    // Test login includes refresh token
    let login_data = LoginRequest {
        email: user_data.email,
        password: user_data.password,
    };

    let login_req = test::TestRequest::post()
        .uri("/backend/auth/login")
        .set_json(&login_data)
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status().as_u16(), 200, "Login should succeed");

    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(login_body["token"].is_string(), "Login should include JWT token");
    assert!(login_body["refresh_token"].is_string(), "Login should include refresh token");

    // Cleanup
    cleanup_db(&pool).await;
}