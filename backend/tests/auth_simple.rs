use actix_web::{test, web, App};
use sqlx::PgPool;
use std::env;
use chrono;

use backend::models::user::{CreateUserRequest, LoginRequest};
use backend::services::auth::AuthService;
use backend::routes;

#[actix_web::test]
async fn test_user_registration_success() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");
    
    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Clean up any existing test data first
    let _ = sqlx::query!("DELETE FROM user_roles").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service))
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure_routes)
            )
    ).await;

    let user_data = CreateUserRequest {
        email: format!("test-{}@example.com", chrono::Utc::now().timestamp_millis()),
        password: "TestPassword123!".to_string(),
        display_name: "Test User".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status().as_u16();
    
    if status != 201 {
        let error_body: serde_json::Value = test::read_body_json(resp).await;
        panic!("Registration failed with status {}: {}", status, error_body);
    }

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["token"].is_string(), "Response should contain JWT token");
    assert_eq!(body["user"]["email"], user_data.email, "Response should contain user email");
    assert_eq!(body["user"]["display_name"], user_data.display_name, "Response should contain display name");
    assert!(body["user"]["roles"].is_array(), "Response should contain roles array");

    // Cleanup
    let _ = sqlx::query!("DELETE FROM user_roles").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&pool).await;
}

#[actix_web::test]
async fn test_user_login_success() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");
    
    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Clean up any existing test data first
    let _ = sqlx::query!("DELETE FROM user_roles").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service))
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure_routes)
            )
    ).await;

    // First, register a user
    let user_data = CreateUserRequest {
        email: format!("test-login-{}@example.com", chrono::Utc::now().timestamp_millis()),
        password: "TestPassword123!".to_string(),
        display_name: "Test Login User".to_string(),
    };

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_data)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status().as_u16(), 201, "Registration should succeed");

    // Now test login
    let login_data = LoginRequest {
        email: user_data.email.clone(),
        password: user_data.password.clone(),
    };

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_data)
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    let login_status = login_resp.status().as_u16();
    
    if login_status != 200 {
        let error_body: serde_json::Value = test::read_body_json(login_resp).await;
        panic!("Login failed with status {}: {}", login_status, error_body);
    }

    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(login_body["token"].is_string(), "Response should contain JWT token");
    assert_eq!(login_body["user"]["email"], user_data.email, "Response should contain user email");
    assert_eq!(login_body["user"]["display_name"], user_data.display_name, "Response should contain display name");
    assert!(login_body["user"]["roles"].is_array(), "Response should contain roles array");

    // Cleanup
    let _ = sqlx::query!("DELETE FROM user_roles").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&pool).await;
}

#[actix_web::test]
async fn test_user_login_invalid_credentials() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");
    
    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // Clean up any existing test data first
    let _ = sqlx::query!("DELETE FROM user_roles").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service))
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure_routes)
            )
    ).await;

    // First, register a user
    let user_data = CreateUserRequest {
        email: format!("test-invalid-{}@example.com", chrono::Utc::now().timestamp_millis()),
        password: "TestPassword123!".to_string(),
        display_name: "Test Invalid User".to_string(),
    };

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_data)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status().as_u16(), 201, "Registration should succeed");

    // Now test login with wrong password
    let invalid_login_data = LoginRequest {
        email: user_data.email.clone(),
        password: "WrongPassword123!".to_string(),
    };

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&invalid_login_data)
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status().as_u16(), 401, "Invalid credentials should return 401 Unauthorized");

    let error_body: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(error_body["error"].as_str().unwrap().contains("Invalid email or password"), 
           "Error message should indicate invalid credentials");

    // Cleanup
    let _ = sqlx::query!("DELETE FROM user_roles").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM incident_timers").execute(&pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&pool).await;
}