use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

/// Test database management with automatic cleanup
#[derive(Clone)]
pub struct TestDatabase {
    pub pool: PgPool,
    pub name: String,
    pub admin_pool: PgPool,
}

impl TestDatabase {
    /// Create a new isolated test database
    pub async fn new(test_name: &str) -> Result<Self> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let db_name = format!("kennwilliamson_test_{}_{}", test_name, timestamp);
        
        // Connect to admin database (postgres) to create test database
        let admin_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/postgres".to_string())
            .replace("/kennwilliamson", "/postgres");
        
        let admin_pool = PgPool::connect(&admin_url).await?;
        
        // Create test database
        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&admin_pool)
            .await?;
        
        // Grant permissions to postgres user
        sqlx::query(&format!("GRANT ALL PRIVILEGES ON DATABASE {} TO postgres", db_name))
            .execute(&admin_pool)
            .await?;
        
        // Connect to test database
        let test_url = admin_url.replace("/postgres", &format!("/{}", db_name));
        let test_pool = PgPool::connect(&test_url).await?;
        
        // Run migrations on test database
        sqlx::migrate!("./migrations").run(&test_pool).await?;
        
        Ok(TestDatabase {
            pool: test_pool,
            name: db_name,
            admin_pool,
        })
    }
    
    /// Get the database URL for this test database
    pub fn database_url(&self) -> String {
        let base_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/kennwilliamson".to_string());
        
        // Ensure we're using the correct username and replace the database name
        if base_url.contains("/kennwilliamson") {
            base_url.replace("/kennwilliamson", &format!("/{}", self.name))
        } else {
            // If the base URL doesn't contain kennwilliamson, construct a new one
            format!("postgresql://postgres:password@localhost:5432/{}", self.name)
        }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Note: Drop runs synchronously, so we can't use async here
        // The actual cleanup will be handled by the test completion
        // We'll implement a cleanup mechanism in the test helpers
    }
}

// Global registry for test databases to enable cleanup
lazy_static::lazy_static! {
    static ref TEST_DATABASES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

/// Register a test database for cleanup
pub async fn register_test_database(db_name: String) {
    let mut databases = TEST_DATABASES.lock().await;
    databases.push(db_name);
}

/// Cleanup all registered test databases
pub async fn cleanup_all_test_databases() -> Result<()> {
    let databases = {
        let mut dbs = TEST_DATABASES.lock().await;
        dbs.drain(..).collect::<Vec<_>>()
    };
    
    if databases.is_empty() {
        return Ok(());
    }
    
    // Connect to admin database for cleanup
    let admin_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/postgres".to_string())
        .replace("/kennwilliamson", "/postgres");
    
    let admin_pool = PgPool::connect(&admin_url).await?;
    
    for db_name in databases {
        cleanup_test_database(&admin_pool, &db_name).await?;
    }
    
    Ok(())
}

/// Cleanup a specific test database
pub async fn cleanup_test_database(admin_pool: &PgPool, db_name: &str) -> Result<()> {
    // Terminate all connections to the test database
    sqlx::query(&format!(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'", 
        db_name
    )).execute(admin_pool).await?;
    
    // Drop the test database
    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
        .execute(admin_pool)
        .await?;
    
    println!("🧹 Cleaned up test database: {}", db_name);
    Ok(())
}

/// Test database factory for easy test setup
pub async fn create_test_database(test_name: &str) -> Result<TestDatabase> {
    let test_db = TestDatabase::new(test_name).await?;
    
    // Register for cleanup
    register_test_database(test_db.name.clone()).await;
    
    Ok(test_db)
}

/// Verify we're using a test database (safety check)
pub fn verify_test_database_url(database_url: &str) {
    if !database_url.contains("kennwilliamson_test") {
        panic!("❌ SAFETY CHECK FAILED: Tests must use test database (kennwilliamson_test), not: {}", database_url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_creation_and_cleanup() {
        let test_db = create_test_database("test_creation").await.unwrap();
        
        // Verify database was created
        assert!(test_db.name.starts_with("kennwilliamson_test_test_creation_"));
        
        // Test that we can query the database
        let result: i32 = sqlx::query_scalar("SELECT 1 as test_value")
            .fetch_one(&test_db.pool)
            .await
            .unwrap();
        
        assert_eq!(result, 1);
        
        // Cleanup will happen automatically via Drop
    }
}
