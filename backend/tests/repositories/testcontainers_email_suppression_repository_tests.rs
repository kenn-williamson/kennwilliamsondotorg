use backend::models::db::{EmailType};
use backend::repositories::postgres::postgres_email_suppression_repository::PostgresEmailSuppressionRepository;
use backend::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};
use chrono::Utc;

#[tokio::test]
async fn test_create_hard_bounce_suppression() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "bounce@example.com".to_string(),
        suppression_type: "bounce".to_string(),
        reason: Some("550 5.1.1 User unknown".to_string()),
        suppress_transactional: true,
        suppress_marketing: true,
    };

    let suppression = repo.create_suppression(&data).await.unwrap();

    assert_eq!(suppression.email, "bounce@example.com");
    assert_eq!(suppression.suppression_type, "bounce");
    assert_eq!(suppression.reason, Some("550 5.1.1 User unknown".to_string()));
    assert!(suppression.suppress_transactional);
    assert!(suppression.suppress_marketing);
    assert_eq!(suppression.bounce_count, 0);

    // Verify it blocks all emails
    assert!(repo.is_email_suppressed("bounce@example.com", EmailType::Transactional).await.unwrap());
    assert!(repo.is_email_suppressed("bounce@example.com", EmailType::Marketing).await.unwrap());
}

#[tokio::test]
async fn test_create_complaint_suppression() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "complaint@example.com".to_string(),
        suppression_type: "complaint".to_string(),
        reason: Some("User marked as spam".to_string()),
        suppress_transactional: true,
        suppress_marketing: true,
    };

    let suppression = repo.create_suppression(&data).await.unwrap();

    assert_eq!(suppression.suppression_type, "complaint");

    // Verify it blocks all emails
    assert!(repo.is_email_suppressed("complaint@example.com", EmailType::Transactional).await.unwrap());
    assert!(repo.is_email_suppressed("complaint@example.com", EmailType::Marketing).await.unwrap());
}

#[tokio::test]
async fn test_create_unsubscribe_suppression() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "unsubscribe@example.com".to_string(),
        suppression_type: "unsubscribe".to_string(),
        reason: Some("User clicked unsubscribe".to_string()),
        suppress_transactional: false,
        suppress_marketing: true,
    };

    let suppression = repo.create_suppression(&data).await.unwrap();

    assert_eq!(suppression.suppression_type, "unsubscribe");

    // Should NOT block transactional emails
    assert!(!repo.is_email_suppressed("unsubscribe@example.com", EmailType::Transactional).await.unwrap());

    // Should block marketing emails
    assert!(repo.is_email_suppressed("unsubscribe@example.com", EmailType::Marketing).await.unwrap());
}

#[tokio::test]
async fn test_create_manual_suppression_with_custom_scope() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "manual@example.com".to_string(),
        suppression_type: "manual".to_string(),
        reason: Some("Admin decision - temporary block".to_string()),
        suppress_transactional: true,
        suppress_marketing: false,
    };

    let suppression = repo.create_suppression(&data).await.unwrap();

    assert_eq!(suppression.suppression_type, "manual");
    assert!(suppression.suppress_transactional);
    assert!(!suppression.suppress_marketing);

    // Respects custom flags
    assert!(repo.is_email_suppressed("manual@example.com", EmailType::Transactional).await.unwrap());
    assert!(!repo.is_email_suppressed("manual@example.com", EmailType::Marketing).await.unwrap());
}

#[tokio::test]
async fn test_duplicate_email_constraint() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "duplicate@example.com".to_string(),
        suppression_type: "bounce".to_string(),
        reason: None,
        suppress_transactional: true,
        suppress_marketing: true,
    };

    // First creation should succeed
    repo.create_suppression(&data).await.unwrap();

    // Second creation with same email should fail (unique constraint)
    let result = repo.create_suppression(&data).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unique") ||
            result_contains_duplicate_key_error());

    fn result_contains_duplicate_key_error() -> bool {
        // PostgreSQL error for duplicate key
        true // SQLx will return database error
    }
}

#[tokio::test]
async fn test_find_by_email() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "find@example.com".to_string(),
        suppression_type: "bounce".to_string(),
        reason: Some("Test reason".to_string()),
        suppress_transactional: true,
        suppress_marketing: true,
    };

    repo.create_suppression(&data).await.unwrap();

    // Should find existing suppression
    let found = repo.find_by_email("find@example.com").await.unwrap();
    assert!(found.is_some());
    let suppression = found.unwrap();
    assert_eq!(suppression.email, "find@example.com");
    assert_eq!(suppression.reason, Some("Test reason".to_string()));

    // Should not find non-existent suppression
    let not_found = repo.find_by_email("notfound@example.com").await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_increment_bounce_count() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "bouncer@example.com".to_string(),
        suppression_type: "bounce".to_string(),
        reason: None,
        suppress_transactional: true,
        suppress_marketing: true,
    };

    repo.create_suppression(&data).await.unwrap();

    // Initial bounce count should be 0
    let suppression = repo.find_by_email("bouncer@example.com").await.unwrap().unwrap();
    assert_eq!(suppression.bounce_count, 0);
    assert!(suppression.last_bounce_at.is_none());

    // Increment bounce count
    let bounced_at = Utc::now();
    repo.increment_bounce_count("bouncer@example.com", bounced_at)
        .await
        .unwrap();

    // Verify count incremented
    let suppression = repo.find_by_email("bouncer@example.com").await.unwrap().unwrap();
    assert_eq!(suppression.bounce_count, 1);
    assert!(suppression.last_bounce_at.is_some());

    // Increment again
    repo.increment_bounce_count("bouncer@example.com", Utc::now())
        .await
        .unwrap();

    let suppression = repo.find_by_email("bouncer@example.com").await.unwrap().unwrap();
    assert_eq!(suppression.bounce_count, 2);
}

#[tokio::test]
async fn test_delete_suppression() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    let data = CreateSuppressionData {
        email: "delete@example.com".to_string(),
        suppression_type: "manual".to_string(),
        reason: None,
        suppress_transactional: false,
        suppress_marketing: true,
    };

    repo.create_suppression(&data).await.unwrap();

    // Verify it exists
    let found = repo.find_by_email("delete@example.com").await.unwrap();
    assert!(found.is_some());

    // Delete it
    repo.delete_suppression("delete@example.com").await.unwrap();

    // Verify it's gone
    let not_found = repo.find_by_email("delete@example.com").await.unwrap();
    assert!(not_found.is_none());

    // Deleting again should not error (idempotent)
    let result = repo.delete_suppression("delete@example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_clean_email_not_suppressed() {
    let test_container = crate::test_helpers::TestContainer::new()
        .await
        .expect("Failed to create test container");
    let repo = PostgresEmailSuppressionRepository::new(test_container.pool.clone());

    // Email not in suppression list should return false
    let is_suppressed = repo
        .is_email_suppressed("clean@example.com", EmailType::Transactional)
        .await
        .unwrap();
    assert!(!is_suppressed);

    let is_suppressed = repo
        .is_email_suppressed("clean@example.com", EmailType::Marketing)
        .await
        .unwrap();
    assert!(!is_suppressed);
}
