use backend::repositories::postgres::postgres_user_preferences_repository::PostgresUserPreferencesRepository;
use backend::repositories::traits::user_preferences_repository::UserPreferencesRepository;
use backend::test_utils::UserBuilder;
use uuid::Uuid;

async fn create_test_user_with_prefs(pool: &sqlx::PgPool) -> backend::models::db::User {
    UserBuilder::new()
        .with_email(&format!("test-{}@example.com", Uuid::new_v4()))
        .persist(pool)
        .await
        .expect("Failed to create test user with preferences")
}

#[tokio::test]
async fn test_find_by_user_id() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user_with_prefs(&test_container.pool).await;

    // Find preferences (created by UserBuilder)
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

    let user = create_test_user_with_prefs(&test_container.pool).await;

    // Update timer_is_public only (real user action)
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

    let user = create_test_user_with_prefs(&test_container.pool).await;

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

    let user = create_test_user_with_prefs(&test_container.pool).await;

    // Enable both settings (real user action)
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
