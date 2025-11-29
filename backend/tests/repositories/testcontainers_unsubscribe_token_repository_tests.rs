use backend::models::db::unsubscribe_token::email_types;
use backend::repositories::postgres::postgres_unsubscribe_token_repository::PostgresUnsubscribeTokenRepository;
use backend::repositories::traits::unsubscribe_token_repository::UnsubscribeTokenRepository;
use backend::test_utils::UserBuilder;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Hash a raw token to store in the database
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

async fn create_test_user(pool: &sqlx::PgPool) -> backend::models::db::User {
    UserBuilder::new()
        .with_email(format!("test-{}@example.com", Uuid::new_v4()))
        .persist(pool)
        .await
        .expect("Failed to create test user")
}

#[tokio::test]
async fn test_create_and_find_by_token_hash() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUnsubscribeTokenRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create a raw token and hash it
    let raw_token = "a".repeat(64);
    let token_hash = hash_token(&raw_token);

    // Create the unsubscribe token
    repo.create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash)
        .await
        .unwrap();

    // Find it by hash
    let found = repo.find_by_token_hash(&token_hash).await.unwrap();
    assert!(found.is_some());

    let token_info = found.unwrap();
    assert_eq!(token_info.user_id, user.id);
    assert_eq!(token_info.email_type, email_types::BLOG_NOTIFICATIONS);
}

#[tokio::test]
async fn test_find_by_token_hash_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUnsubscribeTokenRepository::new(test_container.pool.clone());

    let non_existent_hash = hash_token(&"nonexistent".repeat(64));

    let found = repo.find_by_token_hash(&non_existent_hash).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_by_user_and_type() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUnsubscribeTokenRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create a token
    let raw_token = "b".repeat(64);
    let token_hash = hash_token(&raw_token);
    repo.create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash)
        .await
        .unwrap();

    // Verify it exists
    let found = repo.find_by_token_hash(&token_hash).await.unwrap();
    assert!(found.is_some());

    // Delete it
    repo.delete_by_user_and_type(user.id, email_types::BLOG_NOTIFICATIONS)
        .await
        .unwrap();

    // Verify it's gone
    let found = repo.find_by_token_hash(&token_hash).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_upsert_replaces_existing_token() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUnsubscribeTokenRepository::new(test_container.pool.clone());

    let user = create_test_user(&test_container.pool).await;

    // Create first token
    let raw_token1 = "c".repeat(64);
    let token_hash1 = hash_token(&raw_token1);
    repo.create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash1)
        .await
        .unwrap();

    // Create second token for same user and email type (should replace)
    let raw_token2 = "d".repeat(64);
    let token_hash2 = hash_token(&raw_token2);
    repo.create_or_replace(user.id, email_types::BLOG_NOTIFICATIONS, &token_hash2)
        .await
        .unwrap();

    // First token should no longer be found
    let found1 = repo.find_by_token_hash(&token_hash1).await.unwrap();
    assert!(found1.is_none());

    // Second token should be found
    let found2 = repo.find_by_token_hash(&token_hash2).await.unwrap();
    assert!(found2.is_some());
    assert_eq!(found2.unwrap().user_id, user.id);
}

#[tokio::test]
async fn test_different_users_can_have_tokens() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUnsubscribeTokenRepository::new(test_container.pool.clone());

    let user1 = create_test_user(&test_container.pool).await;
    let user2 = create_test_user(&test_container.pool).await;

    // Create tokens for both users
    let raw_token1 = "e".repeat(64);
    let token_hash1 = hash_token(&raw_token1);
    repo.create_or_replace(user1.id, email_types::BLOG_NOTIFICATIONS, &token_hash1)
        .await
        .unwrap();

    let raw_token2 = "f".repeat(64);
    let token_hash2 = hash_token(&raw_token2);
    repo.create_or_replace(user2.id, email_types::BLOG_NOTIFICATIONS, &token_hash2)
        .await
        .unwrap();

    // Both tokens should be found
    let found1 = repo.find_by_token_hash(&token_hash1).await.unwrap();
    assert!(found1.is_some());
    assert_eq!(found1.unwrap().user_id, user1.id);

    let found2 = repo.find_by_token_hash(&token_hash2).await.unwrap();
    assert!(found2.is_some());
    assert_eq!(found2.unwrap().user_id, user2.id);
}

#[tokio::test]
async fn test_delete_does_not_affect_other_users() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresUnsubscribeTokenRepository::new(test_container.pool.clone());

    let user1 = create_test_user(&test_container.pool).await;
    let user2 = create_test_user(&test_container.pool).await;

    // Create tokens for both users
    let raw_token1 = "g".repeat(64);
    let token_hash1 = hash_token(&raw_token1);
    repo.create_or_replace(user1.id, email_types::BLOG_NOTIFICATIONS, &token_hash1)
        .await
        .unwrap();

    let raw_token2 = "h".repeat(64);
    let token_hash2 = hash_token(&raw_token2);
    repo.create_or_replace(user2.id, email_types::BLOG_NOTIFICATIONS, &token_hash2)
        .await
        .unwrap();

    // Delete user1's token
    repo.delete_by_user_and_type(user1.id, email_types::BLOG_NOTIFICATIONS)
        .await
        .unwrap();

    // User1's token should be gone
    let found1 = repo.find_by_token_hash(&token_hash1).await.unwrap();
    assert!(found1.is_none());

    // User2's token should still exist
    let found2 = repo.find_by_token_hash(&token_hash2).await.unwrap();
    assert!(found2.is_some());
}
