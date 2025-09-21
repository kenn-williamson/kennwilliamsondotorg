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

/// Helper function to create a test database with proper configuration
async fn create_test_database() -> (PgPool, String) {
    let image = GenericImage::new("ghcr.io/fboulnois/pg_uuidv7", "1.6.0")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"))
        .with_env_var("POSTGRES_DB", "testdb")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres");
    
    println!("🚀 Starting pg_uuidv7 container...");
    let container = image.start().await.expect("Failed to start PostgreSQL container");
    println!("✅ Container started successfully");
    
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{}/testdb?sslmode=disable", port);
    
    println!("🔗 Connection string: {}", connection_string);
    
    // Wait for database to be ready with retry logic
    let pool = wait_for_database_ready(&connection_string).await;
    println!("✅ Database connection established");
    
    // Enable pg_uuidv7 extension
    println!("🔧 Enabling pg_uuidv7 extension...");
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7")
        .execute(&pool)
        .await
        .expect("Failed to enable pg_uuidv7 extension");
    println!("✅ pg_uuidv7 extension enabled");
    
    // Run migrations
    println!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("✅ Migrations completed successfully");
    
    (pool, connection_string)
}

/// Helper function to create a test user using the connection pool
async fn create_test_user_in_db(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<backend::models::db::user::User, sqlx::Error> {
    use backend::models::db::user::User;
    
    // Insert user (let database generate the ID)
    println!("📝 Inserting user into database...");
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
    println!("✅ User inserted successfully with ID: {}", user_id);
    
    // Add user role
    println!("🔐 Adding user role...");
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) 
         SELECT $1, id FROM roles WHERE name = 'user'",
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    println!("✅ User role added successfully");
    
    // Fetch the created user
    println!("🔍 Fetching created user...");
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
    println!("✅ User fetched successfully");
    
    Ok(user)
}

/// Test basic database operations with testcontainers
#[tokio::test]
async fn test_database_operations() {
    println!("🚀 Starting database operations test...");
    
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
    
    // Wait for database to be ready with retry logic
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
    
    // Test creating a user
    let email = format!("test_{}@test.com", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    let password_hash = "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string();
    let display_name = "Test User";
    let slug = format!("test-user-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    assert_eq!(user.email, email);
    assert_eq!(user.display_name, display_name);
    assert_eq!(user.slug, slug);
    
    println!("✅ Database operations test passed!");
}

/// Test parallel execution - multiple tests should not conflict
#[tokio::test]
async fn test_parallel_1() {
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
    
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7").execute(&pool).await.expect("Failed to enable pg_uuidv7 extension");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");
    
    let email = format!("parallel1_{}@test.com", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    let password_hash = "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string();
    let display_name = "Parallel User 1";
    let slug = format!("parallel-user-1-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    assert_eq!(user.email, email);
    println!("✅ Parallel test 1 passed!");
}

/// Test parallel execution - multiple tests should not conflict
#[tokio::test]
async fn test_parallel_2() {
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
    
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pg_uuidv7").execute(&pool).await.expect("Failed to enable pg_uuidv7 extension");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");
    
    let email = format!("parallel2_{}@test.com", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    let password_hash = "$2b$04$6X8Q8Q8Q8Q8Q8Q8Q8Q8Q8O8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q".to_string();
    let display_name = "Parallel User 2";
    let slug = format!("parallel-user-2-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    
    let user = create_test_user_in_db(&pool, &email, &password_hash, display_name, &slug)
        .await
        .expect("Failed to create test user");
    
    assert_eq!(user.email, email);
    println!("✅ Parallel test 2 passed!");
}
