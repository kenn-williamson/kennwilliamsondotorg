use actix_web::{test, web, App};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;
use chrono::{Duration, Utc};

use backend::services::container::ServiceContainer;
use backend::routes;
use backend::models::api::{CreateUserRequest, LoginRequest, RefreshTokenRequest};

mod test_helpers;

/// Test the complete refresh token flow to ensure refactor preserved functionality
#[actix_web::test]
async fn test_refresh_token_complete_flow() {
    dotenv::from_filename(".env.test").ok();
    test_helpers::verify_test_database_url();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Clean up any existing test data
    test_helpers::cleanup_test_db(&pool).await;

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");
    
    // Create service container with real repositories
    let container = ServiceContainer::new_development(pool.clone(), jwt_secret);
    
    // Test app setup
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::from(container.auth_service.clone()))
            .app_data(web::Data::from(container.incident_timer_service.clone()))
            .app_data(web::Data::from(container.phrase_service.clone()))
            .configure(routes::configure_app_routes)
    ).await;

    // Step 1: Register a new user
    let test_email = test_helpers::unique_test_email("refresh_test");
    let register_request = CreateUserRequest {
        email: test_email.clone(),
        password: "password123".to_string(),
        display_name: "Refresh Test User".to_string(),
    };

    let register_req = test::TestRequest::post()
        .uri("/backend/public/auth/register")
        .set_json(&register_request)
        .to_request();
    
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status().as_u16(), 201);
    
    let register_data: serde_json::Value = test::read_body_json(register_resp).await;
    let refresh_token = register_data["refresh_token"].as_str().unwrap();
    let jwt_token = register_data["token"].as_str().unwrap();
    
    // Verify we got both tokens
    assert!(!refresh_token.is_empty());
    assert!(!jwt_token.is_empty());
    assert_ne!(refresh_token, jwt_token);

    // Step 2: Use refresh token to get new JWT
    // Add delay to ensure different timestamps (JWT uses second precision)
    tokio::time::sleep(tokio::time::Duration::from_millis(1100)).await;
    
    let refresh_request = RefreshTokenRequest {
        refresh_token: refresh_token.to_string(),
    };

    let refresh_req = test::TestRequest::post()
        .uri("/backend/public/auth/refresh")
        .set_json(&refresh_request)
        .to_request();
    
    let refresh_resp = test::call_service(&app, refresh_req).await;
    assert_eq!(refresh_resp.status().as_u16(), 200);
    
    let refresh_data: serde_json::Value = test::read_body_json(refresh_resp).await;
    let new_jwt_token = refresh_data["token"].as_str().unwrap();
    let new_refresh_token = refresh_data["refresh_token"].as_str().unwrap();
    
    // Verify we got new tokens
    assert!(!new_jwt_token.is_empty());
    assert!(!new_refresh_token.is_empty());
    assert_ne!(new_jwt_token, jwt_token); // Should be different JWT
    assert_ne!(new_refresh_token, refresh_token); // Should be different refresh token

    // Step 3: Verify new JWT works for authenticated request
    let me_req = test::TestRequest::get()
        .uri("/backend/protected/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", new_jwt_token)))
        .to_request();
    
    let me_resp = test::call_service(&app, me_req).await;
    assert_eq!(me_resp.status().as_u16(), 200);
    
    let me_data: serde_json::Value = test::read_body_json(me_resp).await;
    assert_eq!(me_data["email"], test_email);
    assert_eq!(me_data["display_name"], "Refresh Test User");

    // Step 4: Verify old refresh token is invalidated
    let old_refresh_req = test::TestRequest::post()
        .uri("/backend/public/auth/refresh")
        .set_json(&RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
        })
        .to_request();
    
    let old_refresh_resp = test::call_service(&app, old_refresh_req).await;
    assert_eq!(old_refresh_resp.status().as_u16(), 401); // Should be unauthorized

    // Step 5: Test login flow still works
    let login_request = LoginRequest {
        email: register_request.email.clone(),
        password: "password123".to_string(),
    };

    let login_req = test::TestRequest::post()
        .uri("/backend/public/auth/login")
        .set_json(&login_request)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status().as_u16(), 200);
    
    let login_data: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(login_data["refresh_token"].as_str().unwrap().len() > 0);
    assert!(login_data["token"].as_str().unwrap().len() > 0);
}

/// Test refresh token expiration handling
#[actix_web::test]
async fn test_refresh_token_expiration() {
    dotenv::from_filename(".env.test").ok();
    test_helpers::verify_test_database_url();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Clean up any existing test data
    test_helpers::cleanup_test_db(&pool).await;

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");
    
    let container = ServiceContainer::new_development(pool.clone(), jwt_secret);
    
    // Create a user
    let user = test_helpers::create_test_user_in_db(
        &pool,
        &test_helpers::unique_test_email("expiry_test"),
        "$2b$04$test_hash",
        "Expiry Test User",
        &test_helpers::unique_test_slug("expiry-test-user")
    ).await
    .expect("Failed to create test user");

    // Create an expired refresh token directly in the database
    let expired_time = Utc::now() - Duration::days(8); // 8 days ago (expired)
    let token_hash = "expired_token_hash";
    
    test_helpers::create_test_refresh_token_in_db(
        &pool,
        user.id,
        token_hash,
        expired_time,
        None, // No device info
    ).await
    .expect("Failed to insert expired token");

    // Try to use expired token
    let refresh_request = RefreshTokenRequest {
        refresh_token: "expired_token_hash".to_string(),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::from(container.auth_service.clone()))
            .configure(routes::configure_app_routes)
    ).await;

    let refresh_req = test::TestRequest::post()
        .uri("/backend/public/auth/refresh")
        .set_json(&refresh_request)
        .to_request();
    
    let refresh_resp = test::call_service(&app, refresh_req).await;
    assert_eq!(refresh_resp.status().as_u16(), 401); // Should be unauthorized
}

/// Test refresh token with invalid token
#[actix_web::test]
async fn test_refresh_token_invalid() {
    dotenv::from_filename(".env.test").ok();
    test_helpers::verify_test_database_url();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Clean up any existing test data
    test_helpers::cleanup_test_db(&pool).await;

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");
    
    let container = ServiceContainer::new_development(pool.clone(), jwt_secret);
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::from(container.auth_service.clone()))
            .configure(routes::configure_app_routes)
    ).await;

    // Try to use non-existent token
    let refresh_request = RefreshTokenRequest {
        refresh_token: "nonexistent_token".to_string(),
    };

    let refresh_req = test::TestRequest::post()
        .uri("/backend/public/auth/refresh")
        .set_json(&refresh_request)
        .to_request();
    
    let refresh_resp = test::call_service(&app, refresh_req).await;
    assert_eq!(refresh_resp.status().as_u16(), 401); // Should be unauthorized
}
