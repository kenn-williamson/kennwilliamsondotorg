use actix_web::{test, web, App};
use sqlx::PgPool;
use std::env;

mod test_helpers;

use test_helpers::*;
use backend::services::incident_timer::IncidentTimerService;
use backend::services::auth::AuthService;
use backend::models::incident_timer::{CreateIncidentTimer, UpdateIncidentTimer};
use backend::{routes, middleware};

#[actix_web::test]
async fn test_get_timer_by_user_slug_public() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let incident_service = IncidentTimerService::new(pool.clone());

    // Clean up any existing test data first
    cleanup_test_db(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(incident_service))
            .service(
                web::scope("/api")
                    .configure(routes::incident_timers::configure_public_routes)
            )
    ).await;

    // Step 1: Create test user directly in database
    let user_slug = unique_test_slug("timer-test");
    let user = create_test_user_in_db(
        &pool,
        &unique_test_email("timer"),
        &test_password_hash(),
        "Timer Test User",
        &user_slug,
    ).await.expect("Failed to create test user");

    // Step 2: Create test timer directly in database
    let _timer = create_test_timer_in_db(
        &pool,
        user.id,
        None, // Use current timestamp
        Some("Test incident timer"),
    ).await.expect("Failed to create test timer");

    // Step 3: Test the public endpoint to get timer by user slug (NO AUTH REQUIRED)
    let public_req = test::TestRequest::get()
        .uri(&format!("/api/incident-timers/{}", user_slug))
        .to_request();

    let public_resp = test::call_service(&app, public_req).await;
    let public_status = public_resp.status().as_u16();
    
    if public_status != 200 {
        let error_body: serde_json::Value = test::read_body_json(public_resp).await;
        panic!("Public timer access failed with status {}: {}", public_status, error_body);
    }

    let timer_body: serde_json::Value = test::read_body_json(public_resp).await;
    assert!(timer_body["id"].is_string(), "Response should contain timer ID");
    assert!(timer_body["created_at"].is_string(), "Response should contain creation time");
    assert!(timer_body["reset_timestamp"].is_string(), "Response should contain reset timestamp");
    assert_eq!(timer_body["notes"], "Test incident timer", "Response should contain timer notes");

    // Cleanup
    cleanup_test_db(&pool).await;
}

#[actix_web::test]
async fn test_create_timer_protected_endpoint() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);
    let incident_service = IncidentTimerService::new(pool.clone());

    // Clean up any existing test data first
    cleanup_test_db(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(incident_service))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("")
                            .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                            .configure(routes::incident_timers::configure_protected_routes)
                    )
            )
    ).await;

    // Step 1: Create test user directly in database
    let user_slug = unique_test_slug("protected-test");
    let user = create_test_user_in_db(
        &pool,
        &unique_test_email("protected"),
        &test_password_hash(),
        "Protected Test User",
        &user_slug,
    ).await.expect("Failed to create test user");

    // Step 2: Create login credentials and use auth service to get token
    let login_data = backend::models::user::LoginRequest {
        email: user.email.clone(),
        password: "TestPassword123!".to_string(),
    };
    
    let auth_response = auth_service.login(login_data).await
        .expect("Failed to login")
        .expect("Login should succeed");
    let token = auth_response.token;

    // Step 3: Test protected endpoint - create timer
    let timer_data = CreateIncidentTimer {
        reset_timestamp: None,
        notes: Some("Protected timer test".to_string()),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/incident-timers")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&timer_data)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_status = create_resp.status().as_u16();
    
    if create_status != 201 {
        let error_body: serde_json::Value = test::read_body_json(create_resp).await;
        panic!("Timer creation failed with status {}: {}", create_status, error_body);
    }

    let timer_response: serde_json::Value = test::read_body_json(create_resp).await;
    assert!(timer_response["id"].is_string(), "Response should contain timer ID");
    assert_eq!(timer_response["notes"], "Protected timer test", "Notes should match");
    assert!(timer_response["reset_timestamp"].is_string(), "Should contain reset timestamp");

    // Cleanup
    cleanup_test_db(&pool).await;
}

#[actix_web::test]
async fn test_get_user_timers_protected() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);
    let incident_service = IncidentTimerService::new(pool.clone());

    cleanup_test_db(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(incident_service))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("")
                            .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                            .configure(routes::incident_timers::configure_protected_routes)
                    )
            )
    ).await;

    // Create test user and get auth token
    let user_slug = unique_test_slug("get-timers");
    let user = create_test_user_in_db(
        &pool,
        &unique_test_email("get-timers"),
        &test_password_hash(),
        "Get Timers User",
        &user_slug,
    ).await.expect("Failed to create test user");

    let login_data = backend::models::user::LoginRequest {
        email: user.email.clone(),
        password: "TestPassword123!".to_string(),
    };
    
    let auth_response = auth_service.login(login_data).await
        .expect("Failed to login")
        .expect("Login should succeed");
    let token = auth_response.token;

    // Create a couple of test timers for the user
    let _timer1 = create_test_timer_in_db(
        &pool,
        user.id,
        None,
        Some("First timer"),
    ).await.expect("Failed to create first timer");

    let _timer2 = create_test_timer_in_db(
        &pool,
        user.id,
        None,
        Some("Second timer"),
    ).await.expect("Failed to create second timer");

    // Test GET /api/incident-timers (get user's timers)
    let get_req = test::TestRequest::get()
        .uri("/api/incident-timers")
        .append_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    let get_status = get_resp.status().as_u16();
    
    if get_status != 200 {
        let error_body: serde_json::Value = test::read_body_json(get_resp).await;
        panic!("Get timers failed with status {}: {}", get_status, error_body);
    }

    let timers: serde_json::Value = test::read_body_json(get_resp).await;
    assert!(timers.is_array(), "Response should be an array");
    assert_eq!(timers.as_array().unwrap().len(), 2, "Should return 2 timers");
    
    // Verify timer data
    let timer_array = timers.as_array().unwrap();
    assert!(timer_array[0]["id"].is_string(), "Timer should have ID");
    assert!(timer_array[0]["notes"].is_string(), "Timer should have notes");
    assert!(timer_array[1]["id"].is_string(), "Timer should have ID");
    assert!(timer_array[1]["notes"].is_string(), "Timer should have notes");

    cleanup_test_db(&pool).await;
}

#[actix_web::test]
async fn test_update_timer_protected() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);
    let incident_service = IncidentTimerService::new(pool.clone());

    cleanup_test_db(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(incident_service))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("")
                            .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                            .configure(routes::incident_timers::configure_protected_routes)
                    )
            )
    ).await;

    // Create test user and get auth token
    let user_slug = unique_test_slug("update-timer");
    let user = create_test_user_in_db(
        &pool,
        &unique_test_email("update-timer"),
        &test_password_hash(),
        "Update Timer User",
        &user_slug,
    ).await.expect("Failed to create test user");

    let login_data = backend::models::user::LoginRequest {
        email: user.email.clone(),
        password: "TestPassword123!".to_string(),
    };
    
    let auth_response = auth_service.login(login_data).await
        .expect("Failed to login")
        .expect("Login should succeed");
    let token = auth_response.token;

    // Create a test timer
    let timer = create_test_timer_in_db(
        &pool,
        user.id,
        None,
        Some("Original notes"),
    ).await.expect("Failed to create timer");

    // Test PUT /api/incident-timers/{id} (update timer)
    let update_data = UpdateIncidentTimer {
        reset_timestamp: None, // Keep existing
        notes: Some("Updated notes".to_string()),
    };

    let update_req = test::TestRequest::put()
        .uri(&format!("/api/incident-timers/{}", timer.id))
        .append_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_data)
        .to_request();

    let update_resp = test::call_service(&app, update_req).await;
    let update_status = update_resp.status().as_u16();
    
    if update_status != 200 {
        let error_body: serde_json::Value = test::read_body_json(update_resp).await;
        panic!("Update timer failed with status {}: {}", update_status, error_body);
    }

    let updated_timer: serde_json::Value = test::read_body_json(update_resp).await;
    assert_eq!(updated_timer["id"], timer.id.to_string(), "ID should match");
    assert_eq!(updated_timer["notes"], "Updated notes", "Notes should be updated");

    cleanup_test_db(&pool).await;
}

#[actix_web::test]
async fn test_delete_timer_protected() {
    dotenv::from_filename(".env.test").ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env.test");

    let auth_service = AuthService::new(pool.clone(), jwt_secret);
    let incident_service = IncidentTimerService::new(pool.clone());

    cleanup_test_db(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(incident_service))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("")
                            .wrap(actix_web::middleware::from_fn(middleware::auth::jwt_auth_middleware))
                            .configure(routes::incident_timers::configure_protected_routes)
                    )
            )
    ).await;

    // Create test user and get auth token
    let user_slug = unique_test_slug("delete-timer");
    let user = create_test_user_in_db(
        &pool,
        &unique_test_email("delete-timer"),
        &test_password_hash(),
        "Delete Timer User",
        &user_slug,
    ).await.expect("Failed to create test user");

    let login_data = backend::models::user::LoginRequest {
        email: user.email.clone(),
        password: "TestPassword123!".to_string(),
    };
    
    let auth_response = auth_service.login(login_data).await
        .expect("Failed to login")
        .expect("Login should succeed");
    let token = auth_response.token;

    // Create a test timer
    let timer = create_test_timer_in_db(
        &pool,
        user.id,
        None,
        Some("Timer to delete"),
    ).await.expect("Failed to create timer");

    // Test DELETE /api/incident-timers/{id}
    let delete_req = test::TestRequest::delete()
        .uri(&format!("/api/incident-timers/{}", timer.id))
        .append_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let delete_resp = test::call_service(&app, delete_req).await;
    let delete_status = delete_resp.status().as_u16();
    
    if delete_status != 204 {
        let error_body: serde_json::Value = test::read_body_json(delete_resp).await;
        panic!("Delete timer failed with status {}: {}", delete_status, error_body);
    }

    // DELETE should return 204 No Content with empty body
    assert_eq!(delete_status, 204, "Delete should return 204 No Content");

    cleanup_test_db(&pool).await;
}