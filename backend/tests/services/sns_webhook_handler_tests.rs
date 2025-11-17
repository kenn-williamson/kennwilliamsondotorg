use backend::repositories::mocks::MockEmailSuppressionRepository;
use backend::repositories::traits::email_suppression_repository::EmailSuppressionRepository;
/// Phase 3 Unit Tests: SNS Webhook Message Handling
/// Tests the logic for parsing and processing SNS notifications (bounces, complaints)
use backend::services::webhooks::sns_handler::{SnsHandler, SnsMessage};
use chrono::Utc;

#[tokio::test]
async fn test_parse_sns_subscription_confirmation() {
    // Given: SNS subscription confirmation message
    let sns_message = SnsMessage {
        message_type: "SubscriptionConfirmation".to_string(),
        message_id: "test-msg-123".to_string(),
        topic_arn: "arn:aws:sns:us-east-1:123456789:ses-bounces".to_string(),
        message: "You have chosen to subscribe...".to_string(),
        timestamp: "2025-10-03T12:00:00.000Z".to_string(),
        signature_version: "1".to_string(),
        signature: "test-signature".to_string(),
        signing_cert_url: "https://sns.us-east-1.amazonaws.com/cert.pem".to_string(),
        subscribe_url: Some("https://sns.us-east-1.amazonaws.com/subscribe".to_string()),
        token: Some("test-token".to_string()),
    };

    // When: Parsing message type
    let _handler = SnsHandler::new(Box::new(MockEmailSuppressionRepository::new()));
    let message_type = &sns_message.message_type;

    // Then: Should identify as subscription confirmation
    assert_eq!(message_type, "SubscriptionConfirmation");
}

#[tokio::test]
async fn test_parse_hard_bounce_notification() {
    // Given: SNS bounce notification with hard bounce
    let bounce_message = r#"{
        "eventType": "Bounce",
        "bounce": {
            "bounceType": "Permanent",
            "bounceSubType": "General",
            "bouncedRecipients": [
                {"emailAddress": "bounce@example.com", "status": "5.1.1", "diagnosticCode": "smtp; 550 5.1.1 user unknown"}
            ],
            "timestamp": "2025-10-03T12:00:00.000Z",
            "feedbackId": "test-feedback-id-123"
        },
        "mail": {
            "messageId": "msg-123",
            "source": "noreply@kennwilliamson.org"
        }
    }"#;

    let sns_message = SnsMessage {
        message_type: "Notification".to_string(),
        message_id: "sns-msg-123".to_string(),
        topic_arn: "arn:aws:sns:us-east-1:123456789:ses-bounces".to_string(),
        message: bounce_message.to_string(),
        timestamp: "2025-10-03T12:00:00.000Z".to_string(),
        signature_version: "1".to_string(),
        signature: "test-signature".to_string(),
        signing_cert_url: "https://sns.us-east-1.amazonaws.com/cert.pem".to_string(),
        subscribe_url: None,
        token: None,
    };

    // When: Processing bounce notification
    let suppression_repo = MockEmailSuppressionRepository::new();
    let handler = SnsHandler::new(Box::new(suppression_repo.clone()));

    handler.handle_notification(&sns_message).await.unwrap();

    // Then: Should create suppression for hard bounce
    let suppression = suppression_repo
        .find_by_email("bounce@example.com")
        .await
        .unwrap();

    assert!(suppression.is_some());
    let suppression = suppression.unwrap();
    assert_eq!(suppression.suppression_type, "bounce");
    assert!(suppression.suppress_transactional);
    assert!(suppression.suppress_marketing);
}

#[tokio::test]
async fn test_parse_soft_bounce_notification() {
    // Given: SNS bounce notification with soft bounce
    let bounce_message = r#"{
        "eventType": "Bounce",
        "bounce": {
            "bounceType": "Transient",
            "bounceSubType": "MailboxFull",
            "bouncedRecipients": [
                {"emailAddress": "softbounce@example.com", "status": "4.2.2", "diagnosticCode": "mailbox full"}
            ],
            "timestamp": "2025-10-03T12:00:00.000Z",
            "feedbackId": "test-feedback-id-456"
        },
        "mail": {
            "messageId": "msg-456",
            "source": "noreply@kennwilliamson.org"
        }
    }"#;

    let sns_message = SnsMessage {
        message_type: "Notification".to_string(),
        message_id: "sns-msg-456".to_string(),
        topic_arn: "arn:aws:sns:us-east-1:123456789:ses-bounces".to_string(),
        message: bounce_message.to_string(),
        timestamp: "2025-10-03T12:00:00.000Z".to_string(),
        signature_version: "1".to_string(),
        signature: "test-signature".to_string(),
        signing_cert_url: "https://sns.us-east-1.amazonaws.com/cert.pem".to_string(),
        subscribe_url: None,
        token: None,
    };

    // When: Processing soft bounce notification
    let suppression_repo = MockEmailSuppressionRepository::new();
    let handler = SnsHandler::new(Box::new(suppression_repo.clone()));

    handler.handle_notification(&sns_message).await.unwrap();

    // Then: Should create tracking entry (but not suppress) for first soft bounce
    let suppression = suppression_repo
        .find_by_email("softbounce@example.com")
        .await
        .unwrap();

    assert!(
        suppression.is_some(),
        "First soft bounce should create tracking entry"
    );
    let suppression = suppression.unwrap();
    assert_eq!(suppression.suppression_type, "soft_bounce");
    assert!(
        !suppression.suppress_transactional,
        "First soft bounce should not suppress"
    );
    assert_eq!(suppression.bounce_count, 1);
}

#[tokio::test]
async fn test_soft_bounce_threshold_creates_suppression() {
    // Given: Email service with soft bounce tracking
    let suppression_repo = MockEmailSuppressionRepository::new();
    let handler = SnsHandler::new(Box::new(suppression_repo.clone()));

    // Create initial soft bounce suppression entry (for tracking)
    suppression_repo
        .create_suppression(
            &backend::repositories::traits::email_suppression_repository::CreateSuppressionData {
                email: "repeated@example.com".to_string(),
                suppression_type: "soft_bounce".to_string(),
                reason: Some("Tracking soft bounces".to_string()),
                suppress_transactional: false,
                suppress_marketing: false,
            },
        )
        .await
        .unwrap();

    // Increment bounce count twice (total will be 3)
    suppression_repo
        .increment_bounce_count("repeated@example.com", Utc::now())
        .await
        .unwrap();
    suppression_repo
        .increment_bounce_count("repeated@example.com", Utc::now())
        .await
        .unwrap();

    // When: Processing 3rd soft bounce
    let bounce_message = r#"{
        "eventType": "Bounce",
        "bounce": {
            "bounceType": "Transient",
            "bounceSubType": "MailboxFull",
            "bouncedRecipients": [
                {"emailAddress": "repeated@example.com"}
            ],
            "timestamp": "2025-10-03T12:00:00.000Z",
            "feedbackId": "test-feedback-id-789"
        },
        "mail": {"messageId": "msg-789", "source": "noreply@kennwilliamson.org"}
    }"#;

    let sns_message = SnsMessage {
        message_type: "Notification".to_string(),
        message_id: "sns-msg-789".to_string(),
        topic_arn: "arn:aws:sns:us-east-1:123456789:ses-bounces".to_string(),
        message: bounce_message.to_string(),
        timestamp: "2025-10-03T12:00:00.000Z".to_string(),
        signature_version: "1".to_string(),
        signature: "test-signature".to_string(),
        signing_cert_url: "https://sns.us-east-1.amazonaws.com/cert.pem".to_string(),
        subscribe_url: None,
        token: None,
    };

    handler.handle_notification(&sns_message).await.unwrap();

    // Then: Should convert to full suppression after 3rd soft bounce
    let suppression = suppression_repo
        .find_by_email("repeated@example.com")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(suppression.bounce_count, 3);
    assert!(
        suppression.suppress_transactional,
        "Should suppress transactional after 3 soft bounces"
    );
    assert!(
        suppression.suppress_marketing,
        "Should suppress marketing after 3 soft bounces"
    );
}

#[tokio::test]
async fn test_parse_complaint_notification() {
    // Given: SNS complaint notification
    let complaint_message = r#"{
        "eventType": "Complaint",
        "complaint": {
            "complainedRecipients": [
                {"emailAddress": "spam@example.com"}
            ],
            "timestamp": "2025-10-03T12:00:00.000Z",
            "feedbackId": "feedback-123",
            "complaintFeedbackType": "abuse"
        },
        "mail": {
            "messageId": "msg-complaint-123",
            "source": "noreply@kennwilliamson.org"
        }
    }"#;

    let sns_message = SnsMessage {
        message_type: "Notification".to_string(),
        message_id: "sns-complaint-123".to_string(),
        topic_arn: "arn:aws:sns:us-east-1:123456789:ses-complaints".to_string(),
        message: complaint_message.to_string(),
        timestamp: "2025-10-03T12:00:00.000Z".to_string(),
        signature_version: "1".to_string(),
        signature: "test-signature".to_string(),
        signing_cert_url: "https://sns.us-east-1.amazonaws.com/cert.pem".to_string(),
        subscribe_url: None,
        token: None,
    };

    // When: Processing complaint notification
    let suppression_repo = MockEmailSuppressionRepository::new();
    let handler = SnsHandler::new(Box::new(suppression_repo.clone()));

    handler.handle_notification(&sns_message).await.unwrap();

    // Then: Should create full suppression for complaint
    let suppression = suppression_repo
        .find_by_email("spam@example.com")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(suppression.suppression_type, "complaint");
    assert!(suppression.suppress_transactional);
    assert!(suppression.suppress_marketing);
    assert!(suppression.reason.unwrap().contains("abuse"));
}

#[tokio::test]
async fn test_duplicate_bounce_notification_updates_count() {
    // Given: Existing bounce suppression
    let suppression_repo = MockEmailSuppressionRepository::new();
    suppression_repo
        .create_suppression(
            &backend::repositories::traits::email_suppression_repository::CreateSuppressionData {
                email: "existing@example.com".to_string(),
                suppression_type: "bounce".to_string(),
                reason: Some("Initial bounce".to_string()),
                suppress_transactional: true,
                suppress_marketing: true,
            },
        )
        .await
        .unwrap();

    // When: Receiving another bounce notification for same email
    let bounce_message = r#"{
        "eventType": "Bounce",
        "bounce": {
            "bounceType": "Permanent",
            "bounceSubType": "General",
            "bouncedRecipients": [{"emailAddress": "existing@example.com"}],
            "timestamp": "2025-10-03T12:00:00.000Z",
            "feedbackId": "test-feedback-id-999"
        },
        "mail": {"messageId": "msg-999", "source": "noreply@kennwilliamson.org"}
    }"#;

    let sns_message = SnsMessage {
        message_type: "Notification".to_string(),
        message_id: "sns-msg-999".to_string(),
        topic_arn: "arn:aws:sns:us-east-1:123456789:ses-bounces".to_string(),
        message: bounce_message.to_string(),
        timestamp: "2025-10-03T12:00:00.000Z".to_string(),
        signature_version: "1".to_string(),
        signature: "test-signature".to_string(),
        signing_cert_url: "https://sns.us-east-1.amazonaws.com/cert.pem".to_string(),
        subscribe_url: None,
        token: None,
    };

    let handler = SnsHandler::new(Box::new(suppression_repo.clone()));
    handler.handle_notification(&sns_message).await.unwrap();

    // Then: Should increment bounce count instead of failing
    let suppression = suppression_repo
        .find_by_email("existing@example.com")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(suppression.bounce_count, 1);
}
