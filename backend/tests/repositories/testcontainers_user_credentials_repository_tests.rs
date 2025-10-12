use backend::repositories::postgres::postgres_user_credentials_repository::PostgresUserCredentialsRepository;
use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
use backend::repositories::traits::user_credentials_repository::UserCredentialsRepository;
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
async fn test_create_credentials() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    // Create a test user first
    let user = create_test_user(&test_container.pool).await;

    let credentials = repo.create(user.id, "$2b$12$test_hash".to_string()).await.unwrap();

    assert_eq!(credentials.user_id, user.id);
    assert_eq!(credentials.password_hash, "$2b$12$test_hash");
}

#[tokio::test]
async fn test_find_by_user_id() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create credentials
    repo.create(user.id, "hash123".to_string()).await.unwrap();

    // Find credentials
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().password_hash, "hash123");
}

#[tokio::test]
async fn test_find_by_user_id_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    let non_existent_id = Uuid::new_v4();

    let found = repo.find_by_user_id(non_existent_id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_update_password() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create credentials
    repo.create(user.id, "old_hash".to_string()).await.unwrap();

    // Update password
    repo.update_password(user.id, "new_hash".to_string()).await.unwrap();

    // Verify update
    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated.password_hash, "new_hash");
}

#[tokio::test]
async fn test_delete_credentials() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create credentials
    repo.create(user.id, "hash".to_string()).await.unwrap();

    // Delete credentials
    repo.delete(user.id).await.unwrap();

    // Verify deletion
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_has_password() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // User without credentials
    let has_password = repo.has_password(user.id).await.unwrap();
    assert_eq!(has_password, false);

    // Create credentials
    repo.create(user.id, "hash".to_string()).await.unwrap();

    // User with credentials
    let has_password = repo.has_password(user.id).await.unwrap();
    assert_eq!(has_password, true);
}
