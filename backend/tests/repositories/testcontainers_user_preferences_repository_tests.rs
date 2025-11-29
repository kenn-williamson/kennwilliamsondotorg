use backend::repositories::postgres::postgres_user_preferences_repository::PostgresUserPreferencesRepository;
use backend::repositories::traits::user_preferences_repository::UserPreferencesRepository;
use backend::test_utils::UserBuilder;
use uuid::Uuid;

async fn create_test_user_with_prefs(pool: &sqlx::PgPool) -> backend::models::db::User {
    UserBuilder::new()
        .with_email(format!("test-{}@example.com", Uuid::new_v4()))
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
    assert!(!prefs.timer_is_public);
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
    assert!(updated.timer_is_public);
    assert!(!updated.timer_show_in_list);
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
    assert!(updated.timer_is_public);
    assert!(updated.timer_show_in_list);
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
    assert!(updated.timer_is_public);
    assert!(updated.timer_show_in_list);

    // Disable both settings
    repo.update_timer_settings(user.id, false, false)
        .await
        .unwrap();

    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert!(!updated.timer_is_public);
    assert!(!updated.timer_show_in_list);
}

// ============================================================================
// Blog notification preference tests
// ============================================================================

#[tokio::test]
async fn test_update_blog_notifications_enable() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user_with_prefs(&test_container.pool).await;

    // UserBuilder creates preferences with notify_blog_posts = true by default
    // Disable it first
    repo.update_blog_notifications(user.id, false)
        .await
        .unwrap();

    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert!(!updated.notify_blog_posts);

    // Now enable it
    repo.update_blog_notifications(user.id, true).await.unwrap();

    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert!(updated.notify_blog_posts);
}

#[tokio::test]
async fn test_update_blog_notifications_disable() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    let user = create_test_user_with_prefs(&test_container.pool).await;

    // UserBuilder creates preferences with notify_blog_posts = true by default
    repo.update_blog_notifications(user.id, false)
        .await
        .unwrap();

    let updated = repo.find_by_user_id(user.id).await.unwrap().unwrap();
    assert!(!updated.notify_blog_posts);
}

#[tokio::test]
async fn test_find_users_with_blog_notifications_enabled() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    // Create users with different blog notification settings
    let user1 = create_test_user_with_prefs(&test_container.pool).await;
    let user2 = create_test_user_with_prefs(&test_container.pool).await;
    let user3 = create_test_user_with_prefs(&test_container.pool).await;

    // By default all have notify_blog_posts = true, disable one
    repo.update_blog_notifications(user2.id, false)
        .await
        .unwrap();

    // Find all user IDs with blog notifications enabled
    let user_ids_with_notifications = repo.find_users_with_blog_notifications().await.unwrap();

    // Should include user1 and user3, but not user2
    assert!(user_ids_with_notifications.contains(&user1.id));
    assert!(!user_ids_with_notifications.contains(&user2.id));
    assert!(user_ids_with_notifications.contains(&user3.id));
}

#[tokio::test]
async fn test_find_users_with_blog_notifications_empty() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUserPreferencesRepository::new(test_container.pool.clone());

    // Create a user and disable their notifications
    let user = create_test_user_with_prefs(&test_container.pool).await;
    repo.update_blog_notifications(user.id, false)
        .await
        .unwrap();

    // Find all user IDs with blog notifications enabled (should exclude the one we disabled)
    let user_ids_with_notifications = repo.find_users_with_blog_notifications().await.unwrap();

    // The user we created and disabled should not be in the list
    assert!(!user_ids_with_notifications.contains(&user.id));
}
