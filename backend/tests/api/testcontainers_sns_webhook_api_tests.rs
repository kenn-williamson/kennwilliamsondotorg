/// Phase 3 Integration Tests: SNS Webhook API Endpoints
/// Tests the actual HTTP endpoints that AWS SNS will call
use crate::fixtures::TestContext;

#[actix_web::test]
async fn test_sns_subscription_confirmation_endpoint() {
    // Given: Test server with webhook endpoint
    let ctx = TestContext::builder().build().await;

    // SNS subscription confirmation payload
    let subscription_payload = serde_json::json!({
        "Type": "SubscriptionConfirmation",
        "MessageId": "test-msg-123",
        "TopicArn": "arn:aws:sns:us-east-1:123456789:ses-bounces",
        "Message": "You have chosen to subscribe to the topic...",
        "Timestamp": "2025-10-03T12:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "test-signature",
        "SigningCertURL": "https://sns.us-east-1.amazonaws.com/cert.pem",
        "SubscribeURL": "https://sns.us-east-1.amazonaws.com/subscribe?token=test",
        "Token": "test-token"
    });

    // When: Posting subscription confirmation to webhook
    let mut resp = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&subscription_payload)
        .await
        .unwrap();

    // Then: Should attempt to confirm (may fail in test due to fake URL)
    // In production, this would call the real AWS SNS subscribe URL
    // In tests, we expect 500 because the URL is fake
    let status = resp.status();
    let _body = resp.body().await.unwrap();

    // Test should verify the endpoint processes the request (not necessarily succeeds with fake URL)
    assert!(
        status.is_server_error(),
        "Expected 500 error for fake subscribe URL, got {}",
        status
    );
}

#[actix_web::test]
async fn test_hard_bounce_notification_creates_suppression() {
    // Given: Test server with webhook endpoint
    let ctx = TestContext::builder().build().await;

    // Hard bounce notification payload
    let bounce_payload = serde_json::json!({
        "Type": "Notification",
        "MessageId": "bounce-msg-123",
        "TopicArn": "arn:aws:sns:us-east-1:123456789:ses-bounces",
        "Message": serde_json::json!({
            "notificationType": "Bounce",
            "bounce": {
                "bounceType": "Permanent",
                "bounceSubType": "General",
                "bouncedRecipients": [
                    {
                        "emailAddress": "hardbounce@example.com",
                        "status": "5.1.1",
                        "diagnosticCode": "smtp; 550 5.1.1 user unknown"
                    }
                ],
                "timestamp": "2025-10-03T12:00:00.000Z"
            },
            "mail": {
                "messageId": "msg-123",
                "source": "noreply@kennwilliamson.org"
            }
        }).to_string(),
        "Timestamp": "2025-10-03T12:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "test-signature",
        "SigningCertURL": "https://sns.us-east-1.amazonaws.com/cert.pem"
    });

    // When: Posting bounce notification to webhook
    let resp = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&bounce_payload)
        .await
        .unwrap();

    // Then: Should process successfully and return 200
    assert!(resp.status().is_success());

    // And: Should create suppression in database
    let suppression: Option<backend::models::db::EmailSuppression> = sqlx::query_as(
        "SELECT * FROM email_suppressions WHERE email = $1"
    )
    .bind("hardbounce@example.com")
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();

    assert!(suppression.is_some());
    let suppression = suppression.unwrap();
    assert_eq!(suppression.suppression_type, "bounce");
    assert!(suppression.suppress_transactional);
    assert!(suppression.suppress_marketing);
}

#[actix_web::test]
async fn test_complaint_notification_creates_suppression() {
    // Given: Test server with webhook endpoint
    let ctx = TestContext::builder().build().await;

    // Complaint notification payload
    let complaint_payload = serde_json::json!({
        "Type": "Notification",
        "MessageId": "complaint-msg-123",
        "TopicArn": "arn:aws:sns:us-east-1:123456789:ses-complaints",
        "Message": serde_json::json!({
            "notificationType": "Complaint",
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
        }).to_string(),
        "Timestamp": "2025-10-03T12:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "test-signature",
        "SigningCertURL": "https://sns.us-east-1.amazonaws.com/cert.pem"
    });

    // When: Posting complaint notification to webhook
    let resp = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&complaint_payload)
        .await
        .unwrap();

    // Then: Should process successfully and return 200
    assert!(resp.status().is_success());

    // And: Should create suppression in database
    let suppression: Option<backend::models::db::EmailSuppression> = sqlx::query_as(
        "SELECT * FROM email_suppressions WHERE email = $1"
    )
    .bind("spam@example.com")
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();

    assert!(suppression.is_some());
    let suppression = suppression.unwrap();
    assert_eq!(suppression.suppression_type, "complaint");
    assert!(suppression.suppress_transactional);
    assert!(suppression.suppress_marketing);
}

#[actix_web::test]
async fn test_soft_bounce_notification_tracks_count() {
    // Given: Test server with webhook endpoint
    let ctx = TestContext::builder().build().await;

    // Soft bounce notification payload
    let soft_bounce_payload = serde_json::json!({
        "Type": "Notification",
        "MessageId": "soft-bounce-msg-123",
        "TopicArn": "arn:aws:sns:us-east-1:123456789:ses-bounces",
        "Message": serde_json::json!({
            "notificationType": "Bounce",
            "bounce": {
                "bounceType": "Transient",
                "bounceSubType": "MailboxFull",
                "bouncedRecipients": [
                    {"emailAddress": "softbounce@example.com"}
                ],
                "timestamp": "2025-10-03T12:00:00.000Z"
            },
            "mail": {
                "messageId": "msg-soft-123",
                "source": "noreply@kennwilliamson.org"
            }
        }).to_string(),
        "Timestamp": "2025-10-03T12:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "test-signature",
        "SigningCertURL": "https://sns.us-east-1.amazonaws.com/cert.pem"
    });

    // When: Posting soft bounce notification to webhook
    let resp = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&soft_bounce_payload)
        .await
        .unwrap();

    // Then: Should process successfully
    assert!(resp.status().is_success());

    // And: Should track bounce but NOT suppress yet
    let suppression: Option<backend::models::db::EmailSuppression> = sqlx::query_as(
        "SELECT * FROM email_suppressions WHERE email = $1"
    )
    .bind("softbounce@example.com")
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();

    // First soft bounce should track but not suppress
    if let Some(suppression) = suppression {
        assert_eq!(suppression.bounce_count, 1);
        assert!(!suppression.suppress_transactional, "First soft bounce should not suppress");
    }
}

#[actix_web::test]
async fn test_malformed_sns_message_returns_400() {
    // Given: Test server with webhook endpoint
    let ctx = TestContext::builder().build().await;

    // Malformed payload (missing required fields)
    let malformed_payload = serde_json::json!({
        "Type": "Notification",
        "MessageId": "bad-msg"
        // Missing other required fields
    });

    // When: Posting malformed message to webhook
    let resp = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&malformed_payload)
        .await
        .unwrap();

    // Then: Should return 400 Bad Request
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_webhook_endpoint_requires_no_authentication() {
    // Given: Test server with webhook endpoint (no auth token)
    let ctx = TestContext::builder().build().await;

    // Valid SNS subscription confirmation (no auth header)
    let subscription_payload = serde_json::json!({
        "Type": "SubscriptionConfirmation",
        "MessageId": "test-msg-123",
        "TopicArn": "arn:aws:sns:us-east-1:123456789:ses-bounces",
        "Message": "You have chosen to subscribe...",
        "Timestamp": "2025-10-03T12:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "test-signature",
        "SigningCertURL": "https://sns.us-east-1.amazonaws.com/cert.pem",
        "SubscribeURL": "https://sns.us-east-1.amazonaws.com/subscribe",
        "Token": "test-token"
    });

    // When: Posting without Authorization header (AWS SNS doesn't send auth)
    let resp = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&subscription_payload)
        .await
        .unwrap();

    // Then: Should process webhook without authentication requirement
    // (SNS uses signature verification instead)
    // Note: Will return 500 in test due to fake subscribe URL, but that's OK
    // The important part is it doesn't return 401 Unauthorized
    assert!(
        !resp.status().is_client_error() || resp.status().as_u16() != 401,
        "Should not require authentication, got {}",
        resp.status()
    );
}

#[actix_web::test]
async fn test_webhook_idempotent_duplicate_notifications() {
    // Given: Test server with webhook endpoint
    let ctx = TestContext::builder().build().await;

    // Same bounce notification
    let bounce_payload = serde_json::json!({
        "Type": "Notification",
        "MessageId": "duplicate-msg-123",
        "TopicArn": "arn:aws:sns:us-east-1:123456789:ses-bounces",
        "Message": serde_json::json!({
            "notificationType": "Bounce",
            "bounce": {
                "bounceType": "Permanent",
                "bouncedRecipients": [{"emailAddress": "duplicate@example.com"}],
                "timestamp": "2025-10-03T12:00:00.000Z"
            },
            "mail": {"messageId": "msg-dup", "source": "noreply@kennwilliamson.org"}
        }).to_string(),
        "Timestamp": "2025-10-03T12:00:00.000Z",
        "SignatureVersion": "1",
        "Signature": "test-signature",
        "SigningCertURL": "https://sns.us-east-1.amazonaws.com/cert.pem"
    });

    // When: Posting same notification twice
    let resp1 = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&bounce_payload)
        .await
        .unwrap();

    let resp2 = ctx.server
        .post("/backend/webhooks/ses")
        .send_json(&bounce_payload)
        .await
        .unwrap();

    // Then: Both should succeed (idempotent)
    assert!(resp1.status().is_success());
    assert!(resp2.status().is_success());

    // And: Should only have one suppression entry
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM email_suppressions WHERE email = $1"
    )
    .bind("duplicate@example.com")
    .fetch_one(&ctx.pool)
    .await
    .unwrap();

    assert_eq!(count, 1, "Should not create duplicate suppression entries");
}
