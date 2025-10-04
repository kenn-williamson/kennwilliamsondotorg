/// Phase 2 TDD Tests: Email Service Suppression Integration
/// These tests verify that the email service checks the suppression list before sending emails

use backend::models::db::EmailType;
use backend::repositories::mocks::MockEmailSuppressionRepository;
use backend::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};
use backend::services::email::ses_email_service::SesEmailService;
use backend::services::email::EmailService;

#[tokio::test]
async fn test_send_verification_email_succeeds_when_not_suppressed() {
    // Given: An email service with suppression repository
    let suppression_repo = MockEmailSuppressionRepository::new();
    let email_service = SesEmailService::with_suppression(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
        Box::new(suppression_repo),
    );

    // When: Sending to a non-suppressed email
    let result = email_service
        .send_verification_email(
            "clean@example.com",
            Some("Test User"),
            "test-token-123",
            "https://localhost",
        )
        .await;

    // Then: Email should be sent successfully
    // Note: This will actually fail in tests because we don't have AWS credentials
    // But the suppression check should pass before reaching AWS SES
    assert!(result.is_err()); // AWS credentials error, not suppression error
    let error_msg = result.unwrap_err().to_string();
    assert!(!error_msg.contains("suppressed"), "Should not be suppressed, got: {}", error_msg);
}

#[tokio::test]
async fn test_send_verification_email_blocked_when_transactional_suppressed() {
    // Given: An email service with suppression repository
    let suppression_repo = MockEmailSuppressionRepository::new();

    // Add hard bounce suppression (blocks transactional)
    suppression_repo
        .create_suppression(&CreateSuppressionData {
            email: "bounced@example.com".to_string(),
            suppression_type: "bounce".to_string(),
            reason: Some("Hard bounce".to_string()),
            suppress_transactional: true,
            suppress_marketing: true,
        })
        .await
        .unwrap();

    let email_service = SesEmailService::with_suppression(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
        Box::new(suppression_repo),
    );

    // When: Sending verification email to suppressed address
    let result = email_service
        .send_verification_email(
            "bounced@example.com",
            Some("Test User"),
            "test-token-123",
            "https://localhost",
        )
        .await;

    // Then: Email should be blocked with suppression error
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("suppressed") || error_msg.contains("blocked"),
        "Expected suppression error, got: {}",
        error_msg
    );
}

#[tokio::test]
async fn test_send_verification_email_allowed_when_only_marketing_suppressed() {
    // Given: An email service with suppression repository
    let suppression_repo = MockEmailSuppressionRepository::new();

    // Add unsubscribe (blocks marketing only, allows transactional)
    suppression_repo
        .create_suppression(&CreateSuppressionData {
            email: "unsubscribed@example.com".to_string(),
            suppression_type: "unsubscribe".to_string(),
            reason: Some("User unsubscribed from marketing".to_string()),
            suppress_transactional: false,
            suppress_marketing: true,
        })
        .await
        .unwrap();

    let email_service = SesEmailService::with_suppression(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
        Box::new(suppression_repo),
    );

    // When: Sending verification email (transactional) to marketing-only suppressed address
    let result = email_service
        .send_verification_email(
            "unsubscribed@example.com",
            Some("Test User"),
            "test-token-123",
            "https://localhost",
        )
        .await;

    // Then: Email should be allowed (transactional emails bypass marketing suppression)
    // Note: Will fail with AWS credentials error, but NOT suppression error
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(!error_msg.contains("suppressed"), "Transactional email should bypass marketing-only suppression, got: {}", error_msg);
}

#[tokio::test]
async fn test_suppression_check_happens_before_ses_call() {
    // Given: An email service with suppression repository
    let suppression_repo = MockEmailSuppressionRepository::new();

    // Add complaint suppression (blocks all emails)
    suppression_repo
        .create_suppression(&CreateSuppressionData {
            email: "complaint@example.com".to_string(),
            suppression_type: "complaint".to_string(),
            reason: Some("User marked as spam".to_string()),
            suppress_transactional: true,
            suppress_marketing: true,
        })
        .await
        .unwrap();

    let email_service = SesEmailService::with_suppression(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
        Box::new(suppression_repo),
    );

    // When: Attempting to send to suppressed email
    let result = email_service
        .send_verification_email(
            "complaint@example.com",
            Some("Test User"),
            "test-token-123",
            "https://localhost",
        )
        .await;

    // Then: Should fail with suppression error, NOT AWS error
    // This proves suppression check happens before SES API call
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("suppressed") || error_msg.contains("blocked"),
        "Expected suppression error (not AWS error), got: {}",
        error_msg
    );
}

#[tokio::test]
async fn test_email_service_without_suppression_repository_still_works() {
    // Given: An email service WITHOUT suppression repository (backwards compatibility)
    let email_service = SesEmailService::new(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
    );

    // When: Sending verification email
    let result = email_service
        .send_verification_email(
            "any@example.com",
            Some("Test User"),
            "test-token-123",
            "https://localhost",
        )
        .await;

    // Then: Should fail with AWS error (not suppression error)
    // This ensures backwards compatibility - old code without suppression still works
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(!error_msg.contains("suppressed"), "Should not check suppression when repo not provided, got: {}", error_msg);
}
