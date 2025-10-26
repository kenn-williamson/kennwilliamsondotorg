use anyhow::Result;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use super::{Email, EmailService};

/// Mock email service for testing
/// Stores emails in memory instead of sending them
#[derive(Debug, Clone)]
pub struct MockEmailService {
    sent_emails: Arc<Mutex<Vec<Email>>>,
}

// Keep legacy SentEmail for backward compatibility with existing tests
#[derive(Debug, Clone)]
#[allow(dead_code)] // Deprecated struct for backward compatibility
pub struct SentEmail {
    #[allow(dead_code)] // Used for test assertions
    pub to_email: String,
    #[allow(dead_code)] // Used for test assertions
    pub to_name: Option<String>,
    #[allow(dead_code)] // Used for test assertions
    pub verification_token: String,
    #[allow(dead_code)] // Used for test assertions
    pub frontend_url: String,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get all sent emails (for testing assertions)
    #[allow(dead_code)] // Testing infrastructure API
    pub fn get_sent_emails(&self) -> Vec<Email> {
        self.sent_emails.lock().unwrap().clone()
    }

    /// Clear all sent emails (for test cleanup)
    #[allow(dead_code)] // Testing infrastructure API
    pub fn clear(&self) {
        self.sent_emails.lock().unwrap().clear();
    }

    /// Get count of sent emails
    #[allow(dead_code)] // Testing infrastructure API
    pub fn count(&self) -> usize {
        self.sent_emails.lock().unwrap().len()
    }

    /// Get the last sent email (for testing)
    #[allow(dead_code)] // Testing infrastructure API
    pub fn last_sent_email(&self) -> Option<Email> {
        self.sent_emails.lock().unwrap().last().cloned()
    }

    /// Extract verification token from email body (for testing)
    ///
    /// Parses the verification URL from the email text body and extracts the token parameter
    #[allow(dead_code)] // Testing infrastructure API
    pub fn extract_verification_token(&self, email: &Email) -> Option<String> {
        Self::extract_token_from_url(&email.text_body, "verify-email?token=")
    }

    /// Extract password reset token from email body (for testing)
    ///
    /// Parses the password reset URL from the email text body and extracts the token parameter
    #[allow(dead_code)] // Testing infrastructure API
    pub fn extract_password_reset_token(&self, email: &Email) -> Option<String> {
        Self::extract_token_from_url(&email.text_body, "reset-password?token=")
    }

    /// Helper method to extract a token from a URL pattern in text
    fn extract_token_from_url(text: &str, pattern: &str) -> Option<String> {
        // Find the pattern in the text
        let start_idx = text.find(pattern)?;
        let token_start = start_idx + pattern.len();

        // Extract token until whitespace or end of line
        let remaining = &text[token_start..];
        let token_end = remaining
            .find(|c: char| c.is_whitespace() || c == '\n' || c == '\r')
            .unwrap_or(remaining.len());

        Some(remaining[..token_end].to_string())
    }
}

impl Default for MockEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for MockEmailService {
    async fn send_email(&self, email: Email) -> Result<()> {
        log::info!(
            "Mock email service: Capturing email '{}' to {} recipient(s)",
            email.subject,
            email.to.len()
        );

        self.sent_emails.lock().unwrap().push(email);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_email_service_with_template() {
        use crate::services::email::templates::{VerificationEmailTemplate, EmailTemplate};

        let service = MockEmailService::new();

        // Use template pattern directly (matches new approach)
        let template = VerificationEmailTemplate::new(
            "Test User",
            "test-token-123",
            "https://example.com",
        );

        let html_body = template.render_html().unwrap();
        let text_body = template.render_plain_text();
        let subject = template.subject();

        let email = Email::builder()
            .to("test@example.com")
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()
            .unwrap();

        service.send_email(email).await.unwrap();

        assert_eq!(service.count(), 1);

        let emails = service.get_sent_emails();
        assert_eq!(emails[0].to, vec!["test@example.com"]);
        assert!(emails[0].subject.contains("Verify"));
        assert!(emails[0].subject.contains("Email Address"));
        assert!(emails[0].text_body.contains("test-token-123"));
        assert!(emails[0].text_body.contains("Test User"));
        assert!(emails[0].html_body.is_some());

        service.clear();
        assert_eq!(service.count(), 0);
    }

    #[tokio::test]
    async fn test_mock_email_service_send_email() {
        let service = MockEmailService::new();

        let email = Email::builder()
            .to("user@example.com")
            .subject("Test Subject")
            .text_body("Test plain text")
            .html_body("<p>Test HTML</p>")
            .build()
            .unwrap();

        service.send_email(email).await.unwrap();

        assert_eq!(service.count(), 1);

        let last_email = service.last_sent_email().unwrap();
        assert_eq!(last_email.to, vec!["user@example.com"]);
        assert_eq!(last_email.subject, "Test Subject");
        assert_eq!(last_email.text_body, "Test plain text");
        assert_eq!(last_email.html_body, Some("<p>Test HTML</p>".to_string()));
    }
}
