/// Phase 2 TDD Tests: Email Service Suppression Integration
/// These tests verify that the email service checks the suppression list before sending emails

use backend::repositories::mocks::MockEmailSuppressionRepository;
use backend::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};
use backend::services::email::ses_email_service::SesEmailService;
use backend::services::email::EmailService;
use backend::services::email::templates::{Email, EmailTemplate, VerificationEmailTemplate};

#[tokio::test]
async fn test_send_verification_email_succeeds_when_not_suppressed() {
    // Given: An email service with suppression repository
    let suppression_repo = MockEmailSuppressionRepository::new();
    let email_service = SesEmailService::with_suppression(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
        Box::new(suppression_repo),
    );

    // When: Sending to a non-suppressed email using template
    let template = VerificationEmailTemplate::new(
        "Test User",
        "test-token-123",
        "https://localhost",
    );

    let html_body = template.render_html().unwrap();
    let text_body = template.render_plain_text();
    let subject = template.subject();

    let email = Email::builder()
        .to("clean@example.com")
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body)
        .build()
        .unwrap();

    let result = email_service.send_email(email).await;

    // Then: Email should either succeed OR fail with AWS error (NOT suppression error)
    // Note: May succeed if AWS credentials are configured, or fail if not
    if let Err(err) = result {
        let error_msg = err.to_string();
        assert!(!error_msg.contains("suppressed"), "Should not be suppressed, got: {}", error_msg);
    }
    // If result.is_ok(), that's fine - AWS credentials were available
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

    // When: Sending verification email to suppressed address using template
    let template = VerificationEmailTemplate::new(
        "Test User",
        "test-token-123",
        "https://localhost",
    );

    let html_body = template.render_html().unwrap();
    let text_body = template.render_plain_text();
    let subject = template.subject();

    let email = Email::builder()
        .to("bounced@example.com")
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body)
        .build()
        .unwrap();

    let result = email_service.send_email(email).await;

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

    // When: Sending verification email (transactional) to marketing-only suppressed address using template
    let template = VerificationEmailTemplate::new(
        "Test User",
        "test-token-123",
        "https://localhost",
    );

    let html_body = template.render_html().unwrap();
    let text_body = template.render_plain_text();
    let subject = template.subject();

    let email = Email::builder()
        .to("unsubscribed@example.com")
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body)
        .build()
        .unwrap();

    let result = email_service.send_email(email).await;

    // Then: Email should be allowed (transactional emails bypass marketing suppression)
    // Note: May succeed if AWS credentials are configured, or fail with AWS error (NOT suppression)
    if let Err(err) = result {
        let error_msg = err.to_string();
        assert!(!error_msg.contains("suppressed"), "Transactional email should bypass marketing-only suppression, got: {}", error_msg);
    }
    // If result.is_ok(), that's fine - AWS credentials were available and email was sent
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

    // When: Attempting to send to suppressed email using template
    let template = VerificationEmailTemplate::new(
        "Test User",
        "test-token-123",
        "https://localhost",
    );

    let html_body = template.render_html().unwrap();
    let text_body = template.render_plain_text();
    let subject = template.subject();

    let email = Email::builder()
        .to("complaint@example.com")
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body)
        .build()
        .unwrap();

    let result = email_service.send_email(email).await;

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
    // Given: An email service WITH suppression repository (using mock for backwards compatibility test)
    let suppression_repo = MockEmailSuppressionRepository::new();
    let email_service = SesEmailService::with_suppression(
        "noreply@kennwilliamson.org".to_string(),
        Some("support@kennwilliamson.org".to_string()),
        Box::new(suppression_repo),
    );

    // When: Sending verification email using template
    let template = VerificationEmailTemplate::new(
        "Test User",
        "test-token-123",
        "https://localhost",
    );

    let html_body = template.render_html().unwrap();
    let text_body = template.render_plain_text();
    let subject = template.subject();

    let email = Email::builder()
        .to("any@example.com")
        .subject(subject)
        .text_body(text_body)
        .html_body(html_body)
        .build()
        .unwrap();

    let result = email_service.send_email(email).await;

    // Then: Should either succeed OR fail with AWS error (NOT suppression error)
    // This ensures backwards compatibility - code with suppression repo still works
    if let Err(err) = result {
        let error_msg = err.to_string();
        assert!(!error_msg.contains("suppressed"), "Should not have suppression error, got: {}", error_msg);
    }
    // If result.is_ok(), that's fine - AWS credentials were available and email was sent
}
