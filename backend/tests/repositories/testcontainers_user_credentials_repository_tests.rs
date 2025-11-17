use backend::repositories::postgres::postgres_user_credentials_repository::PostgresUserCredentialsRepository;
use backend::repositories::traits::user_credentials_repository::UserCredentialsRepository;
use backend::test_utils::UserBuilder;
use uuid::Uuid;

async fn create_test_user_with_credentials(pool: &sqlx::PgPool) -> backend::models::db::User {
    UserBuilder::new()
        .with_email(format!("test-{}@example.com", Uuid::new_v4()))
        .with_slug(format!("test-{}", Uuid::new_v4()))
        .with_password("test_password")
        .persist(pool)
        .await
        .expect("Failed to create test user")
}

#[tokio::test]
async fn test_find_by_user_id() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    let user = create_test_user_with_credentials(&test_container.pool).await;

    // Find credentials (created by UserBuilder)
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_some());
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

    let user = create_test_user_with_credentials(&test_container.pool).await;

    // Update password (real user action)
    repo.update_password(user.id, "new_hash".to_string())
        .await
        .unwrap();

    // Verify update
    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated.password_hash, "new_hash");
}

#[tokio::test]
async fn test_has_password() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserCredentialsRepository::new(test_container.pool.clone());

    // OAuth-only user (no password)
    let oauth_user = UserBuilder::new()
        .oauth("google_123", "OAuth User")
        .persist(&test_container.pool)
        .await
        .unwrap();

    let has_password = repo.has_password(oauth_user.id).await.unwrap();
    assert!(!has_password);

    // User with password
    let pwd_user = create_test_user_with_credentials(&test_container.pool).await;
    let has_password = repo.has_password(pwd_user.id).await.unwrap();
    assert!(has_password);
}
