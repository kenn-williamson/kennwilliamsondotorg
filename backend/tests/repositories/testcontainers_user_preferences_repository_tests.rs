use backend::repositories::postgres::postgres_user_preferences_repository::PostgresUserPreferencesRepository;
use backend::repositories::postgres::postgres_user_repository::PostgresUserRepository;
use backend::repositories::traits::user_preferences_repository::UserPreferencesRepository;
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
async fn test_create_preferences() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    let preferences = repo.create(user.id).await.unwrap();

    assert_eq!(preferences.user_id, user.id);
    assert_eq!(preferences.timer_is_public, false);
    assert_eq!(preferences.timer_show_in_list, false);
}

#[tokio::test]
async fn test_find_by_user_id() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create preferences
    repo.create(user.id).await.unwrap();

    // Find preferences
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_some());
    let prefs = found.unwrap();
    assert_eq!(prefs.user_id, user.id);
    assert_eq!(prefs.timer_is_public, false);
}

#[tokio::test]
async fn test_find_by_user_id_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let non_existent_id = Uuid::new_v4();

    let found = repo.find_by_user_id(non_existent_id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_update_timer_is_public() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create preferences with defaults
    repo.create(user.id).await.unwrap();

    // Update timer_is_public only
    repo.update_timer_settings(user.id, true, false)
        .await
        .unwrap();

    // Verify update
    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated.timer_is_public, true);
    assert_eq!(updated.timer_show_in_list, false);
}

#[tokio::test]
async fn test_update_timer_show_in_list() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create preferences
    repo.create(user.id).await.unwrap();

    // Update both settings (is_public must be true for show_in_list)
    repo.update_timer_settings(user.id, true, true)
        .await
        .unwrap();

    // Verify update
    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated.timer_is_public, true);
    assert_eq!(updated.timer_show_in_list, true);
}

#[tokio::test]
async fn test_update_timer_settings_both() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create preferences
    repo.create(user.id).await.unwrap();

    // Enable both settings
    repo.update_timer_settings(user.id, true, true)
        .await
        .unwrap();

    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated.timer_is_public, true);
    assert_eq!(updated.timer_show_in_list, true);

    // Disable both settings
    repo.update_timer_settings(user.id, false, false)
        .await
        .unwrap();

    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert_eq!(updated.timer_is_public, false);
    assert_eq!(updated.timer_show_in_list, false);
}

#[tokio::test]
async fn test_delete_preferences() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create preferences
    repo.create(user.id).await.unwrap();

    // Delete preferences
    repo.delete(user.id).await.unwrap();

    // Verify deletion
    let found = repo.find_by_user_id(user.id).await.unwrap();
    assert!(found.is_none());
}
