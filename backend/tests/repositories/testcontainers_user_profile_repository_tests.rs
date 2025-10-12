use backend::repositories::postgres::postgres_user_profile_repository::PostgresUserProfileRepository;
use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
use backend::repositories::traits::user_profile_repository::{
    UpdateProfile, UserProfileRepository,
};
use backend::repositories::traits::user_repository::{CreateUserData, UserRepository};
use uuid::Uuid;

async fn create_test_user(pool: &sqlx::PgPool) -> backend::models::db::User {
    let repo = PostgresUserRepository::new(pool.clone());
    let user_data = CreateUserData {
        email: format!("test-{}@example.com", Uuid::new_v4()),
        password_hash: "temp_hash".to_string(),
        display_name: "Test User".to_string(),
        slug: format!("test-{}", Uuid::new_v4()),
    };
    repo.create_user(&user_data).await.unwrap()
}

#[tokio::test]
async fn test_create_profile() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    let profile = repo.create(user.id).await.unwrap();

    assert_eq!(profile.user_id, user.id);
    assert!(profile.real_name.is_none());
    assert!(profile.bio.is_none());
    assert!(profile.avatar_url.is_none());
    assert!(profile.location.is_none());
    assert!(profile.website.is_none());
}

#[tokio::test]
async fn test_find_by_user_id() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create profile
    repo.create(user.id).await.unwrap();

    // Find profile
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().user_id, user.id);
}

#[tokio::test]
async fn test_find_by_user_id_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let non_existent_id = Uuid::new_v4();

    let found = repo.find_by_user_id(non_existent_id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_update_profile_single_field() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create profile
    repo.create(user.id).await.unwrap();

    // Update only real_name
    let update_data = UpdateProfile {
        real_name: Some("John Doe".to_string()),
        bio: None,
        avatar_url: None,
        location: None,
        website: None,
    };

    let updated = repo.update(user.id, update_data).await.unwrap();

    assert_eq!(updated.real_name, Some("John Doe".to_string()));
    assert!(updated.bio.is_none());
}

#[tokio::test]
async fn test_update_profile_multiple_fields() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create profile
    repo.create(user.id).await.unwrap();

    // Update multiple fields
    let update_data = UpdateProfile {
        real_name: Some("Jane Smith".to_string()),
        bio: Some("Software developer".to_string()),
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        location: Some("San Francisco".to_string()),
        website: Some("https://example.com".to_string()),
    };

    let updated = repo.update(user.id, update_data).await.unwrap();

    assert_eq!(updated.real_name, Some("Jane Smith".to_string()));
    assert_eq!(updated.bio, Some("Software developer".to_string()));
    assert_eq!(
        updated.avatar_url,
        Some("https://example.com/avatar.jpg".to_string())
    );
    assert_eq!(updated.location, Some("San Francisco".to_string()));
    assert_eq!(updated.website, Some("https://example.com".to_string()));
}

#[tokio::test]
async fn test_update_profile_partial() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create profile and update with initial data
    repo.create(user.id).await.unwrap();
    let initial_update = UpdateProfile {
        real_name: Some("Initial Name".to_string()),
        bio: Some("Initial Bio".to_string()),
        avatar_url: None,
        location: None,
        website: None,
    };
    repo.update(user.id, initial_update).await.unwrap();

    // Update only location, keeping other fields unchanged
    let partial_update = UpdateProfile {
        real_name: None,
        bio: None,
        avatar_url: None,
        location: Some("New York".to_string()),
        website: None,
    };

    let updated = repo.update(user.id, partial_update).await.unwrap();

    // Verify previous fields are preserved
    assert_eq!(updated.real_name, Some("Initial Name".to_string()));
    assert_eq!(updated.bio, Some("Initial Bio".to_string()));
    assert_eq!(updated.location, Some("New York".to_string()));
}

#[tokio::test]
async fn test_delete_profile() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserProfileRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create profile
    repo.create(user.id).await.unwrap();

    // Delete profile
    repo.delete(user.id).await.unwrap();

    // Verify deletion
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_none());
}
