use serde_json::json;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Wait for database to be ready with retry logic
async fn wait_for_database_ready(connection_string: &str) -> PgPool {
    let mut attempt = 0;
    let max_attempts = 10;
    
    while attempt < max_attempts {
        attempt += 1;
        println!("🔍 Database readiness check attempt {}/{}", attempt, max_attempts);
        
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(10))
            .idle_timeout(std::time::Duration::from_secs(600))
            .connect(connection_string)
            .await
        {
            Ok(pool) => {
                // Test the connection
                match sqlx::query("SELECT 1").fetch_one(&pool).await {
                    Ok(_) => {
                        println!("✅ Database is ready!");
                        return pool;
                    },
                    Err(e) => {
                        println!("⚠️  Connection established but query failed: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("❌ Connection failed: {}", e);
            }
        }
        
        if attempt < max_attempts {
            let delay = std::cmp::min(1 << attempt, 8); // Exponential backoff, max 8 seconds
            println!("⏳ Waiting {}s before retry...", delay);
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
        }
    }
    
    panic!("Database failed to become ready after {} attempts", max_attempts);
}

/// Create a test app with testcontainers database
async fn create_test_app_with_testcontainers() -> (actix_test::TestServer, PgPool) {
    use backend::services::container::ServiceContainer;
    use backend::routes;
    use actix_web::{web, App};
    use actix_test;
    
    // Create container and keep it alive for the entire test
    let image = GenericImage::new("ghcr.io/fboulnois/pg_uuidv7", "1.6.0")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"))
        .with_env_var("POSTGRES_DB", "testdb")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres");

    let _container = image.start().await.expect("Failed to start PostgreSQL container");
    let port = _container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable", port);

    let pool = wait_for_database_ready(&connection_string).await;

    // Enable pg_uuidv7 extension
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7")
        .execute(&pool)
        .await
        .expect("Failed to enable pg_uuidv7 extension");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Create service container
    let jwt_secret = "test-jwt-secret-for-api-tests".to_string();
    let container = ServiceContainer::new_development(pool.clone(), jwt_secret);
    
    // Create test server
    let pool_clone = pool.clone();
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool_clone.clone()))
            .app_data(web::Data::from(container.auth_service.clone()))
            .app_data(web::Data::from(container.incident_timer_service.clone()))
            .app_data(web::Data::from(container.phrase_service.clone()))
            .app_data(web::Data::from(container.admin_service.clone()))
            .app_data(web::Data::from(container.phrase_moderation_service.clone()))
            .app_data(web::Data::from(container.stats_service.clone()))
            .configure(routes::configure_app_routes)
    });
    
    (srv, pool)
}

/// Create a test user in the database
async fn create_test_user_in_db(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<backend::models::db::user::User, sqlx::Error> {
    use backend::models::db::user::User;

    // Insert user (let database generate the ID)
    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, display_name, slug, active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, true, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .bind(slug)
    .fetch_one(pool)
    .await?;

    let user_id: Uuid = result.get("id");

    // Add user role
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) 
         SELECT $1, id FROM roles WHERE name = 'user'",
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    // Fetch the created user
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT u.id, u.email, u.password_hash, u.display_name, u.slug, u.active, u.created_at, u.updated_at,
               COALESCE(ARRAY_AGG(r.name) FILTER (WHERE r.name IS NOT NULL), ARRAY[]::text[]) as roles
        FROM users u
        LEFT JOIN user_roles ur ON u.id = ur.user_id
        LEFT JOIN roles r ON ur.role_id = r.id
        WHERE u.id = $1
        GROUP BY u.id, u.email, u.password_hash, u.display_name, u.slug, u.active, u.created_at, u.updated_at
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Create a JWT token for testing
async fn create_test_jwt_token(user: &backend::models::db::user::User) -> Result<String, anyhow::Error> {
    use backend::services::auth::jwt::JwtService;
    
    let jwt_secret = "test-jwt-secret-for-api-tests".to_string();
    let jwt_service = JwtService::new(jwt_secret);
    
    jwt_service.generate_token(user).map_err(|e| e.into())
}

/// Generates a unique test email
fn unique_test_email() -> String {
    format!("test_{}@test.com", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

/// Generates a unique test slug
fn unique_test_slug() -> String {
    format!("test-user-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

/// Test password hash for testing
fn test_password_hash() -> String {
    "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string()
}

#[actix_web::test]
async fn test_register_success() {
    let (srv, _pool) = create_test_app_with_testcontainers().await;
    
    let request_body = json!({
        "email": unique_test_email(),
        "password": "TestPassword123!",
        "display_name": "Test User"
    });
    
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    
    println!("Registration response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Registration error response: {:?}", body);
    }
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert!(body.get("user").is_some());
    
    let user = body.get("user").unwrap();
    assert_eq!(user.get("email").unwrap(), request_body.get("email").unwrap());
    assert_eq!(user.get("display_name").unwrap(), request_body.get("display_name").unwrap());
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    let (srv, _pool) = create_test_app_with_testcontainers().await;
    
    let email = unique_test_email();
    let request_body = json!({
        "email": email,
        "password": "TestPassword123!",
        "display_name": "Test User"
    });
    
    // First registration should succeed
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    // Second registration with same email should fail
    let mut resp = srv.post("/backend/public/auth/register")
        .send_json(&request_body)
        .await
        .unwrap();
    assert_eq!(resp.status(), 409); // Conflict
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("error").is_some());
    assert!(body.get("error").unwrap().as_str().unwrap().contains("Email already exists"));
}

#[actix_web::test]
async fn test_login_success() {
    let (srv, pool) = create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = unique_test_email();
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug();
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let request_body = json!({
        "email": user.email,
        "password": "TestPassword123!"
    });
    
    let mut resp = srv.post("/backend/public/auth/login")
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert!(body.get("user").is_some());
    
    let returned_user = body.get("user").unwrap();
    assert_eq!(returned_user.get("id").unwrap().as_str().unwrap(), user.id.to_string());
    assert_eq!(returned_user.get("email").unwrap().as_str().unwrap(), user.email);
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    let (srv, _pool) = create_test_app_with_testcontainers().await;
    
    let request_body = json!({
        "email": "nonexistent@example.com",
        "password": "WrongPassword123!"
    });
    
    let mut resp = srv.post("/backend/public/auth/login")
        .send_json(&request_body)
        .await
        .unwrap();
    assert_eq!(resp.status(), 401); // Unauthorized
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("error").is_some());
    assert_eq!(body.get("error").unwrap(), "Invalid email or password");
}

#[actix_web::test]
async fn test_get_current_user_success() {
    let (srv, pool) = create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = unique_test_email();
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug();
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    // Create JWT token for the user
    let token = create_test_jwt_token(&user).await.unwrap();
    
    let mut resp = srv.get("/backend/protected/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("id").unwrap().as_str().unwrap(), user.id.to_string());
    assert_eq!(body.get("email").unwrap().as_str().unwrap(), user.email);
    assert_eq!(body.get("display_name").unwrap().as_str().unwrap(), user.display_name);
}

#[actix_web::test]
async fn test_get_current_user_unauthorized() {
    let (srv, _pool) = create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/auth/me")
        .send()
        .await
        .unwrap();
    println!("Unauthorized response status: {}", resp.status());
    if resp.status() != 401 {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Unexpected response: {:?}", body);
    }
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_get_current_user_invalid_token() {
    let (srv, _pool) = create_test_app_with_testcontainers().await;
    
    let mut resp = srv.get("/backend/protected/auth/me")
        .insert_header(("Authorization", "Bearer invalid_token"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_update_profile_success() {
    let (srv, pool) = create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = unique_test_email();
    let password_hash = test_password_hash();
    let display_name = "Original Name";
    let slug = unique_test_slug();
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let token = create_test_jwt_token(&user).await.unwrap();
    
    let request_body = json!({
        "display_name": "Updated Name",
        "slug": "updated-slug"
    });
    
    let mut resp = srv.put("/backend/protected/auth/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("display_name").unwrap(), "Updated Name");
    assert_eq!(body.get("slug").unwrap(), "updated-slug");
}

#[actix_web::test]
async fn test_change_password_success() {
    let (srv, pool) = create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = unique_test_email();
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug();
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let token = create_test_jwt_token(&user).await.unwrap();
    
    let request_body = json!({
        "current_password": "TestPassword123!",
        "new_password": "NewPassword456!"
    });
    
    let mut resp = srv.put("/backend/protected/auth/change-password")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("message").unwrap(), "Password changed successfully");
}

#[actix_web::test]
async fn test_change_password_wrong_current() {
    let (srv, pool) = create_test_app_with_testcontainers().await;
    
    // Create a test user
    let email = unique_test_email();
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug();
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    let token = create_test_jwt_token(&user).await.unwrap();
    
    let request_body = json!({
        "current_password": "WrongPassword123!",
        "new_password": "NewPassword456!"
    });
    
    let mut resp = srv.put("/backend/protected/auth/change-password")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&request_body)
        .await
        .unwrap();
    assert_eq!(resp.status(), 400); // Bad Request
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body.get("error").unwrap(), "Current password is incorrect");
}
