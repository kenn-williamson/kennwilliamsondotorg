use backend::events::event_bus::InMemoryEventBus;
use backend::events::handlers::{
    AccessRequestEmailNotificationHandler, PhraseSuggestionEmailNotificationHandler,
};
use backend::events::types::{AccessRequestCreatedEvent, PhraseSuggestionCreatedEvent};
use backend::events::{EventBus, EventPublisher};
use backend::repositories::mocks::{MockAdminRepository, MockUserRepository};
use backend::services::email::MockEmailService;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

/// Test that AccessRequestCreatedEvent triggers email notifications
#[tokio::test]
async fn test_access_request_event_sends_emails_to_admins() {
    // Setup mocks
    let mut mock_admin_repo = MockAdminRepository::new();
    let mock_email_service = MockEmailService::new();

    // Configure expectations
    mock_admin_repo
        .expect_get_admin_emails()
        .times(1)
        .returning(|| {
            Ok(vec![
                "admin1@example.com".to_string(),
                "admin2@example.com".to_string(),
            ])
        });

    // Clone for verification
    let email_service_clone = mock_email_service.clone();

    // Create event bus
    let mut event_bus = InMemoryEventBus::new();

    // Register handler
    let handler = AccessRequestEmailNotificationHandler::new(
        Arc::new(mock_admin_repo),
        Arc::new(mock_email_service),
        "https://kennwilliamson.org",
    );
    event_bus
        .register_handler(Box::new(handler))
        .expect("Failed to register handler");

    // Publish event
    let event = AccessRequestCreatedEvent::new(
        Uuid::new_v4(),
        "user@example.com",
        "Test User",
        "I would like access to submit timers",
        "trusted-contact",
    );

    event_bus
        .publish(Box::new(event))
        .await
        .expect("Failed to publish event");

    // Give handler time to execute (fire-and-forget)
    sleep(Duration::from_millis(100)).await;

    // Verify email was sent
    assert_eq!(
        email_service_clone.count(),
        1,
        "Expected exactly one email to be sent"
    );

    let sent_emails = email_service_clone.get_sent_emails();
    assert_eq!(
        sent_emails[0].to,
        vec!["admin1@example.com", "admin2@example.com"],
        "Email should be sent to all admins"
    );
    assert!(
        sent_emails[0].subject.contains("Access Request"),
        "Subject should mention access request"
    );
    assert!(
        sent_emails[0]
            .html_body
            .as_ref()
            .unwrap()
            .contains("Test User"),
        "Email body should contain user's name"
    );
}

/// Test that PhraseSuggestionCreatedEvent triggers email notifications
#[tokio::test]
async fn test_phrase_suggestion_event_sends_emails_to_admins() {
    // Setup mocks
    let mut mock_admin_repo = MockAdminRepository::new();
    let mut mock_user_repo = MockUserRepository::new();
    let mock_email_service = MockEmailService::new();

    // Configure expectations
    mock_admin_repo
        .expect_get_admin_emails()
        .times(1)
        .returning(|| Ok(vec!["admin@example.com".to_string()]));

    // Configure user repository to return user details
    mock_user_repo.expect_find_by_id().times(1).returning(|_| {
        Ok(Some(
            backend::test_utils::UserBuilder::new()
                .with_display_name("Phrase Suggester")
                .build(),
        ))
    });

    // Clone for verification
    let email_service_clone = mock_email_service.clone();

    // Create event bus
    let mut event_bus = InMemoryEventBus::new();

    // Register handler
    let handler = PhraseSuggestionEmailNotificationHandler::new(
        Arc::new(mock_admin_repo),
        Arc::new(mock_user_repo),
        Arc::new(mock_email_service),
        "https://kennwilliamson.org",
    );
    event_bus
        .register_handler(Box::new(handler))
        .expect("Failed to register handler");

    // Publish event
    let event = PhraseSuggestionCreatedEvent::new(
        Uuid::new_v4(),
        "Time is an illusion. Lunchtime doubly so.",
    );

    event_bus
        .publish(Box::new(event))
        .await
        .expect("Failed to publish event");

    // Give handler time to execute
    sleep(Duration::from_millis(100)).await;

    // Verify email was sent
    assert_eq!(email_service_clone.count(), 1);

    let sent_emails = email_service_clone.get_sent_emails();
    assert_eq!(sent_emails[0].to, vec!["admin@example.com"]);
    assert!(sent_emails[0].subject.contains("Phrase Suggestion"));
    assert!(
        sent_emails[0]
            .html_body
            .as_ref()
            .unwrap()
            .contains("Time is an illusion")
    );
}

/// Test that multiple handlers can be registered for different event types
#[tokio::test]
async fn test_multiple_event_types_with_different_handlers() {
    // Setup mocks for access request handler
    let mut mock_admin_repo1 = MockAdminRepository::new();
    let mock_email_service1 = MockEmailService::new();

    mock_admin_repo1
        .expect_get_admin_emails()
        .times(1)
        .returning(|| Ok(vec!["admin@example.com".to_string()]));

    let email_service_clone1 = mock_email_service1.clone();

    // Setup mocks for phrase suggestion handler
    let mut mock_admin_repo2 = MockAdminRepository::new();
    let mut mock_user_repo2 = MockUserRepository::new();
    let mock_email_service2 = MockEmailService::new();

    mock_admin_repo2
        .expect_get_admin_emails()
        .times(1)
        .returning(|| Ok(vec!["admin@example.com".to_string()]));

    mock_user_repo2.expect_find_by_id().times(1).returning(|_| {
        Ok(Some(
            backend::test_utils::UserBuilder::new()
                .with_display_name("User")
                .build(),
        ))
    });

    let email_service_clone2 = mock_email_service2.clone();

    // Create event bus
    let mut event_bus = InMemoryEventBus::new();

    // Register both handlers
    let access_request_handler = AccessRequestEmailNotificationHandler::new(
        Arc::new(mock_admin_repo1),
        Arc::new(mock_email_service1),
        "https://kennwilliamson.org",
    );
    event_bus
        .register_handler(Box::new(access_request_handler))
        .expect("Failed to register access request handler");

    let phrase_suggestion_handler = PhraseSuggestionEmailNotificationHandler::new(
        Arc::new(mock_admin_repo2),
        Arc::new(mock_user_repo2),
        Arc::new(mock_email_service2),
        "https://kennwilliamson.org",
    );
    event_bus
        .register_handler(Box::new(phrase_suggestion_handler))
        .expect("Failed to register phrase suggestion handler");

    // Publish access request event
    let access_event = AccessRequestCreatedEvent::new(
        Uuid::new_v4(),
        "user@example.com",
        "User",
        "Message",
        "role",
    );
    event_bus
        .publish(Box::new(access_event))
        .await
        .expect("Failed to publish access request event");

    // Publish phrase suggestion event
    let phrase_event = PhraseSuggestionCreatedEvent::new(Uuid::new_v4(), "Phrase");
    event_bus
        .publish(Box::new(phrase_event))
        .await
        .expect("Failed to publish phrase suggestion event");

    // Give handlers time to execute
    sleep(Duration::from_millis(100)).await;

    // Verify both handlers executed independently
    assert_eq!(
        email_service_clone1.count(),
        1,
        "Access request handler should have sent email"
    );
    assert_eq!(
        email_service_clone2.count(),
        1,
        "Phrase suggestion handler should have sent email"
    );
}

/// Test that handler failures don't prevent other events from being processed
#[tokio::test]
async fn test_handler_error_isolation() {
    // Setup mock that will fail on first call
    let mut mock_admin_repo = MockAdminRepository::new();

    mock_admin_repo
        .expect_get_admin_emails()
        .times(2)
        .returning(|| Err(anyhow::anyhow!("Database error")));

    // Create event bus
    let mut event_bus = InMemoryEventBus::new();

    // Register handler
    let handler = AccessRequestEmailNotificationHandler::new(
        Arc::new(mock_admin_repo),
        Arc::new(MockEmailService::new()),
        "https://kennwilliamson.org",
    );
    event_bus
        .register_handler(Box::new(handler))
        .expect("Failed to register handler");

    // Publish two events
    let event1 = AccessRequestCreatedEvent::new(
        Uuid::new_v4(),
        "user1@example.com",
        "User 1",
        "Message",
        "role",
    );
    let event2 = AccessRequestCreatedEvent::new(
        Uuid::new_v4(),
        "user2@example.com",
        "User 2",
        "Message",
        "role",
    );

    // Both publishes should succeed even though handlers will fail
    event_bus
        .publish(Box::new(event1))
        .await
        .expect("Publish should succeed even if handler fails");
    event_bus
        .publish(Box::new(event2))
        .await
        .expect("Publish should succeed even if handler fails");

    // Give handlers time to fail
    sleep(Duration::from_millis(100)).await;

    // Test passes if we get here without panicking
}

/// Test that EventBus works correctly when no handlers are registered
#[tokio::test]
async fn test_publish_with_no_handlers_registered() {
    let event_bus = InMemoryEventBus::new();

    // Publish event with no handlers
    let event = AccessRequestCreatedEvent::new(
        Uuid::new_v4(),
        "user@example.com",
        "User",
        "Message",
        "role",
    );

    // Should not error
    let result = event_bus.publish(Box::new(event)).await;
    assert!(result.is_ok(), "Publishing with no handlers should succeed");
}
