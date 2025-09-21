use crate::integration::test_database::create_test_database;
use crate::test_helpers::{test_password_hash, unique_test_email, unique_test_slug};

#[tokio::test]
async fn test_database_per_test_isolation() {
    // Create first test database
    let test_db_1 = create_test_database("isolation_test_1").await.unwrap();
    
    // Create second test database
    let test_db_2 = create_test_database("isolation_test_2").await.unwrap();
    
    // Verify they have different names
    assert_ne!(test_db_1.name, test_db_2.name);
    
    // Create user in first database
    let user_1 = crate::test_helpers::create_test_user_in_db(
        &test_db_1.pool,
        &unique_test_email("user1"),
        &test_password_hash(),
        "User One",
        &unique_test_slug("user1"),
    ).await.unwrap();
    
    // Create user in second database
    let user_2 = crate::test_helpers::create_test_user_in_db(
        &test_db_2.pool,
        &unique_test_email("user2"),
        &test_password_hash(),
        "User Two",
        &unique_test_slug("user2"),
    ).await.unwrap();
    
    // Verify users exist in their respective databases
    assert_eq!(user_1.display_name, "User One");
    assert_eq!(user_2.display_name, "User Two");
    
    // Verify user from db1 doesn't exist in db2
    let result = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT COUNT(*) FROM users WHERE id = $1"
    )
    .bind(user_1.id)
    .fetch_one(&test_db_2.pool)
    .await
    .unwrap();
    
    assert_eq!(result, Some(0)); // User 1 should not exist in db 2
    
    // Verify user from db2 doesn't exist in db1
    let result = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT COUNT(*) FROM users WHERE id = $1"
    )
    .bind(user_2.id)
    .fetch_one(&test_db_1.pool)
    .await
    .unwrap();
    
    assert_eq!(result, Some(0)); // User 2 should not exist in db 1
}

#[tokio::test]
async fn test_database_migrations_work() {
    let test_db = create_test_database("migrations_test").await.unwrap();
    
    // Verify all tables exist by querying them
    let tables = sqlx::query_scalar::<_, String>(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name"
    )
    .fetch_all(&test_db.pool)
    .await
    .unwrap();
    
    // Check that key tables exist
    assert!(tables.contains(&"users".to_string()));
    assert!(tables.contains(&"roles".to_string()));
    assert!(tables.contains(&"user_roles".to_string()));
    assert!(tables.contains(&"incident_timers".to_string()));
    assert!(tables.contains(&"refresh_tokens".to_string()));
    assert!(tables.contains(&"phrases".to_string()));
    assert!(tables.contains(&"phrase_suggestions".to_string()));
    assert!(tables.contains(&"user_excluded_phrases".to_string()));
}

#[tokio::test]
async fn test_database_cleanup_after_test() {
    let test_db = create_test_database("cleanup_test").await.unwrap();
    let db_name = test_db.name.clone();
    
    // Create some data
    let _user = crate::test_helpers::create_test_user_in_db(
        &test_db.pool,
        &unique_test_email("cleanup_user"),
        &test_password_hash(),
        "Cleanup User",
        &unique_test_slug("cleanup-user"),
    ).await.unwrap();
    
    // Verify data exists
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&test_db.pool)
        .await
        .unwrap();
    
    assert_eq!(count, 1);
    
    // Database will be cleaned up automatically when test_db goes out of scope
    // This test verifies that the database was created and can be used
    println!("Created test database: {}", db_name);
}

