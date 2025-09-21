use backend::repositories::traits::UserRepository;
use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
use backend::models::db::user::User;
use crate::integration::{create_test_app_for_repository, TestDatabase};
use crate::test_helpers::{test_password_hash, unique_test_email, unique_test_slug};

#[tokio::test]
async fn test_user_repository_create_and_find() {
    let (container, test_db) = create_test_app_for_repository("user_repo_create_find").await.unwrap();
    
    // Get the repository from container
    let user_repo = container.user_repository.clone();
    
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
async fn test_user_repository_update_profile() {
    let (container, test_db) = create_test_app_for_repository("user_repo_update_profile").await.unwrap();
    
    let user_repo = container.user_repository.clone();
    
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
async fn test_user_repository_change_password() {
    let (container, test_db) = create_test_app_for_repository("user_repo_change_password").await.unwrap();
    
    let user_repo = container.user_repository.clone();
    
    // Create user
    let email = unique_test_email("test_user");
    let original_password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug("test-user");
    
    let user = user_repo.create_user(
        &email,
        &original_password_hash,
        &display_name,
        &slug,
    ).await.unwrap();
    
    // Change password
    let new_password_hash = bcrypt::hash("NewPassword123!", 4).unwrap();
    user_repo.change_password(user.id, &new_password_hash).await.unwrap();
    
    // Verify password was changed
    let updated_user = user_repo.find_by_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated_user.password_hash, new_password_hash);
    assert_ne!(updated_user.password_hash, original_password_hash);
}

#[tokio::test]
async fn test_user_repository_deactivate_user() {
    let (container, test_db) = create_test_app_for_repository("user_repo_deactivate").await.unwrap();
    
    let user_repo = container.user_repository.clone();
    
    // Create user
    let email = unique_test_email("test_user");
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug("test-user");
    
    let user = user_repo.create_user(
        &email,
        &password_hash,
        &display_name,
        &slug,
    ).await.unwrap();
    
    // Verify user is active
    assert!(user.active);
    
    // Deactivate user
    user_repo.deactivate_user(user.id).await.unwrap();
    
    // Verify user is deactivated
    let deactivated_user = user_repo.find_by_id(user.id).await.unwrap().unwrap();
    assert!(!deactivated_user.active);
}

#[tokio::test]
async fn test_user_repository_activate_user() {
    let (container, test_db) = create_test_app_for_repository("user_repo_activate").await.unwrap();
    
    let user_repo = container.user_repository.clone();
    
    // Create and deactivate user
    let email = unique_test_email("test_user");
    let password_hash = test_password_hash();
    let display_name = "Test User";
    let slug = unique_test_slug("test-user");
    
    let user = user_repo.create_user(
        &email,
        &password_hash,
        &display_name,
        &slug,
    ).await.unwrap();
    
    user_repo.deactivate_user(user.id).await.unwrap();
    
    // Verify user is deactivated
    let deactivated_user = user_repo.find_by_id(user.id).await.unwrap().unwrap();
    assert!(!deactivated_user.active);
    
    // Activate user
    user_repo.activate_user(user.id).await.unwrap();
    
    // Verify user is activated
    let activated_user = user_repo.find_by_id(user.id).await.unwrap().unwrap();
    assert!(activated_user.active);
}

#[tokio::test]
async fn test_user_repository_unique_constraints() {
    let (container, test_db) = create_test_app_for_repository("user_repo_unique_constraints").await.unwrap();
    
    let user_repo = container.user_repository.clone();
    
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

#[tokio::test]
async fn test_user_repository_not_found_cases() {
    let (container, test_db) = create_test_app_for_repository("user_repo_not_found").await.unwrap();
    
    let user_repo = container.user_repository.clone();
    let non_existent_id = uuid::Uuid::new_v4();
    
    // Find by non-existent ID
    let result = user_repo.find_by_id(non_existent_id).await.unwrap();
    assert!(result.is_none());
    
    // Find by non-existent email
    let result = user_repo.find_by_email("nonexistent@example.com").await.unwrap();
    assert!(result.is_none());
    
    // Find by non-existent slug
    let result = user_repo.find_by_slug("nonexistent-slug").await.unwrap();
    assert!(result.is_none());
}

