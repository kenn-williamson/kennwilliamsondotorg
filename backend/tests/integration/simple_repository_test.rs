use backend::repositories::traits::UserRepository;
use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
use crate::integration::test_database::create_test_database;
use crate::test_helpers::{test_password_hash, unique_test_email, unique_test_slug};

#[tokio::test]
async fn test_postgres_user_repository_create_and_find() {
    let test_db = create_test_database("user_repo_simple").await.unwrap();
    
    // Create repository with test database
    let user_repo = PostgresUserRepository::new(test_db.pool.clone());
    
    // Test data
    let email = unique_test_email("test_user");
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug("test-user");
    
    // Create user
    let user = user_repo.create_user(
        &email,
        &password_hash,
        &display_name,
        &slug,
    ).await.unwrap();
    
    // Verify user was created
    assert_eq!(user.email, email);
    assert_eq!(user.display_name, display_name);
    assert_eq!(user.slug, slug);
    assert!(user.active);
    
    // Find user by ID
    let found_user = user_repo.find_by_id(user.id).await.unwrap().unwrap();
    assert_eq!(found_user.id, user.id);
    assert_eq!(found_user.email, email);
    
    // Find user by email
    let found_by_email = user_repo.find_by_email(&email).await.unwrap().unwrap();
    assert_eq!(found_by_email.id, user.id);
    
    // Find user by slug
    let found_by_slug = user_repo.find_by_slug(&slug).await.unwrap().unwrap();
    assert_eq!(found_by_slug.id, user.id);
}

#[tokio::test]
async fn test_postgres_user_repository_update_profile() {
    let test_db = create_test_database("user_repo_update").await.unwrap();
    
    let user_repo = PostgresUserRepository::new(test_db.pool.clone());
    
    // Create initial user
    let email = unique_test_email("test_user");
    let password_hash = test_password_hash();
    let display_name = "Original Name";
    let slug = unique_test_slug("original-slug");
    
    let user = user_repo.create_user(
        &email,
        &password_hash,
        &display_name,
        &slug,
    ).await.unwrap();
    
    // Update profile
    let new_display_name = "Updated Name";
    let new_slug = unique_test_slug("updated-slug");
    
    let updated_user = user_repo.update_profile(
        user.id,
        &new_display_name,
        &new_slug,
    ).await.unwrap();
    
    // Verify updates
    assert_eq!(updated_user.display_name, new_display_name);
    assert_eq!(updated_user.slug, new_slug);
    assert_eq!(updated_user.email, email); // Email should not change
    
    // Verify in database
    let found_user = user_repo.find_by_id(user.id).await.unwrap().unwrap();
    assert_eq!(found_user.display_name, new_display_name);
    assert_eq!(found_user.slug, new_slug);
}

#[tokio::test]
async fn test_postgres_user_repository_unique_constraints() {
    let test_db = create_test_database("user_repo_constraints").await.unwrap();
    
    let user_repo = PostgresUserRepository::new(test_db.pool.clone());
    
    // Create first user
    let email1 = unique_test_email("test_user");
    let password_hash = test_password_hash();
    let display_name1 = "Test User 1";
    let slug1 = unique_test_slug("test-user");
    
    let user1 = user_repo.create_user(
        &email1,
        &password_hash,
        &display_name1,
        &slug1,
    ).await.unwrap();
    
    // Try to create user with same email (should fail)
    let result = user_repo.create_user(
        &email1, // Same email
        &password_hash,
        "Different Name",
        &unique_test_slug("different-slug"),
    ).await;
    
    assert!(result.is_err());
    
    // Try to create user with same slug (should fail)
    let result = user_repo.create_user(
        &unique_test_email("different_user"),
        &password_hash,
        "Different Name",
        &slug1, // Same slug
    ).await;
    
    assert!(result.is_err());
}

