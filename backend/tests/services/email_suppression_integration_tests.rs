/// Phase 2 TDD Tests: Email Service Suppression Integration
/// These tests verify that the SuppressionGuard checks the suppression list before sending emails

use backend::repositories::mocks::MockEmailSuppressionRepository;
use backend::repositories::traits::email_suppression_repository::{
    CreateSuppressionData, EmailSuppressionRepository,
};
use backend::services::email::{EmailService, MockEmailService, SuppressionGuard};
use backend::services::email::templates::{Email, EmailTemplate, VerificationEmailTemplate};

#[tokio::test]
async fn test_send_verification_email_succeeds_when_not_suppressed() {
    // Given: A suppression guard wrapping mock email service
    let mock_email_service = MockEmailService::new();
    let suppression_repo = MockEmailSuppressionRepository::new();
    let email_service = SuppressionGuard::new(
        Box::new(mock_email_service.clone()),
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

    // Then: Email should be sent successfully (no AWS calls, just stored in mock)
    assert!(result.is_ok(), "Email should be sent successfully");
    assert_eq!(mock_email_service.count(), 1, "Email should be stored in mock service");
}

#[tokio::test]
async fn test_send_verification_email_blocked_when_transactional_suppressed() {
    // Given: A suppression guard with a suppressed email
    let mock_email_service = MockEmailService::new();
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

    let email_service = SuppressionGuard::new(
        Box::new(mock_email_service.clone()),
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
    assert!(result.is_err(), "Email should be blocked");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("suppressed"),
        "Expected suppression error, got: {}",
        error_msg
    );
    assert_eq!(mock_email_service.count(), 0, "No email should reach mock service");
}

#[tokio::test]
async fn test_send_verification_email_allowed_when_only_marketing_suppressed() {
    // Given: A suppression guard with marketing-only suppression
    let mock_email_service = MockEmailService::new();
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

    let email_service = SuppressionGuard::new(
        Box::new(mock_email_service.clone()),
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
    assert!(result.is_ok(), "Transactional email should bypass marketing-only suppression");
    assert_eq!(mock_email_service.count(), 1, "Email should reach mock service");
}

#[tokio::test]
async fn test_suppression_check_happens_before_email_service_call() {
    // Given: A suppression guard with a complaint suppression
    let mock_email_service = MockEmailService::new();
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

    let email_service = SuppressionGuard::new(
        Box::new(mock_email_service.clone()),
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

    // Then: Should fail with suppression error before reaching inner service
    // This proves suppression check happens before the wrapped email service is called
    assert!(result.is_err(), "Email should be blocked");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("suppressed"),
        "Expected suppression error, got: {}",
        error_msg
    );
    assert_eq!(mock_email_service.count(), 0, "Email should never reach mock service");
}

#[tokio::test]
async fn test_email_service_can_work_without_suppression_guard() {
    // Given: A bare mock email service (no suppression guard wrapper)
    let mock_email_service = MockEmailService::new();

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

    let result = mock_email_service.send_email(email).await;

    // Then: Email should be sent successfully (SuppressionGuard is optional)
    // This demonstrates that email services can work independently of suppression checking
    assert!(result.is_ok(), "Email should be sent without suppression guard");
    assert_eq!(mock_email_service.count(), 1, "Email should be stored in mock service");
}
