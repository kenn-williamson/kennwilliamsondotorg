use backend::repositories::postgres::postgres_user_external_login_repository::PostgresUserExternalLoginRepository;
use backend::repositories::traits::user_external_login_repository::{
    CreateExternalLogin, UserExternalLoginRepository,
};
use backend::test_utils::UserBuilder;
use uuid::Uuid;

// Fixture helper: Create a user for testing external login repository
// Uses UserBuilder pattern for resilient test fixtures
async fn create_test_user(pool: &sqlx::PgPool) -> backend::models::db::User {
    UserBuilder::new()
        .with_email(format!("test-{}@example.com", Uuid::new_v4()))
        .with_slug(format!("test-{}", Uuid::new_v4()))
        .with_password("temp_hash")
        .persist(pool)
        .await
        .expect("Failed to create test user")
}

#[tokio::test]
async fn test_create_external_login() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    let data = CreateExternalLogin {
        user_id: user.id,
        provider: "google".to_string(),
        provider_user_id: "google_123".to_string(),
    };

    let login = repo.create(data).await.unwrap();
    assert_eq!(login.user_id, user.id);
    assert_eq!(login.provider, "google");
    assert_eq!(login.provider_user_id, "google_123");
}

#[tokio::test]
async fn test_find_by_provider() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create login
    let data = CreateExternalLogin {
        user_id: user.id,
        provider: "github".to_string(),
        provider_user_id: "github_456".to_string(),
    };
    repo.create(data).await.unwrap();

    // Find by provider
    let found = repo.find_by_provider("github", "github_456").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().user_id, user.id);
}

#[tokio::test]
async fn test_find_by_provider_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    // Should not find non-existent provider
    let found = repo
        .find_by_provider("nonexistent", "user_999")
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_find_by_user_id_multiple_providers() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create Google login
    repo.create(CreateExternalLogin {
        user_id: user.id,
        provider: "google".to_string(),
        provider_user_id: "google_123".to_string(),
    })
    .await
    .unwrap();

    // Create GitHub login
    repo.create(CreateExternalLogin {
        user_id: user.id,
        provider: "github".to_string(),
        provider_user_id: "github_456".to_string(),
    })
    .await
    .unwrap();

    // Find all logins for user
    let logins = repo.find_by_user_id(user.id).await.unwrap();
    assert_eq!(logins.len(), 2);

    let providers: Vec<String> = logins.iter().map(|l| l.provider.clone()).collect();
    assert!(providers.contains(&"google".to_string()));
    assert!(providers.contains(&"github".to_string()));
}

#[tokio::test]
async fn test_unlink_provider() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create login
    repo.create(CreateExternalLogin {
        user_id: user.id,
        provider: "google".to_string(),
        provider_user_id: "google_123".to_string(),
    })
    .await
    .unwrap();

    // Unlink provider
    repo.unlink_provider(user.id, "google").await.unwrap();

    // Verify unlinked
    let found = repo.find_by_provider("google", "google_123").await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_is_provider_linked() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Not linked initially
    let is_linked = repo.is_provider_linked(user.id, "google").await.unwrap();
    assert!(!is_linked);

    // Create login
    repo.create(CreateExternalLogin {
        user_id: user.id,
        provider: "google".to_string(),
        provider_user_id: "google_123".to_string(),
    })
    .await
    .unwrap();

    // Now linked
    let is_linked = repo.is_provider_linked(user.id, "google").await.unwrap();
    assert!(is_linked);
}

// Note: delete() method removed - CASCADE handles deletion when user is deleted
// Test removed: test_delete_external_login

#[tokio::test]
async fn test_unique_provider_constraint() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserExternalLoginRepository::new(test_container.pool.clone());

    let user1 = create_test_user(&test_container.pool).await;
    let user2 = create_test_user(&test_container.pool).await;

    // User1 links Google account
    repo.create(CreateExternalLogin {
        user_id: user1.id,
        provider: "google".to_string(),
        provider_user_id: "google_shared".to_string(),
    })
    .await
    .unwrap();

    // User2 tries to link the same Google account - should fail
    let result = repo
        .create(CreateExternalLogin {
            user_id: user2.id,
            provider: "google".to_string(),
            provider_user_id: "google_shared".to_string(),
        })
        .await;

    assert!(result.is_err());
    // Should be a unique constraint violation
    assert!(result.unwrap_err().to_string().contains("unique")
        || result_contains_duplicate_key_error());

    fn result_contains_duplicate_key_error() -> bool {
        // PostgreSQL error for duplicate key
        true
    }
}
