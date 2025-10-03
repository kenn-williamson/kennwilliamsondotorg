use anyhow::Result;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use super::EmailService;

/// Mock email service for testing
/// Stores emails in memory instead of sending them
#[derive(Debug, Clone)]
pub struct MockEmailService {
    sent_emails: Arc<Mutex<Vec<SentEmail>>>,
}

#[derive(Debug, Clone)]
pub struct SentEmail {
    pub to_email: String,
    pub to_name: Option<String>,
    pub verification_token: String,
    pub frontend_url: String,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get all sent emails (for testing assertions)
    pub fn get_sent_emails(&self) -> Vec<SentEmail> {
        self.sent_emails.lock().unwrap().clone()
    }

    /// Clear all sent emails (for test cleanup)
    pub fn clear(&self) {
        self.sent_emails.lock().unwrap().clear();
    }

    /// Get count of sent emails
    pub fn count(&self) -> usize {
        self.sent_emails.lock().unwrap().len()
    }
}

impl Default for MockEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for MockEmailService {
    async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: Option<&str>,
        verification_token: &str,
        frontend_url: &str,
    ) -> Result<()> {
        let email = SentEmail {
            to_email: to_email.to_string(),
            to_name: to_name.map(|s| s.to_string()),
            verification_token: verification_token.to_string(),
            frontend_url: frontend_url.to_string(),
        };

        self.sent_emails.lock().unwrap().push(email);

        log::info!("Mock email sent to: {}", to_email);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_email_service() {
        let service = MockEmailService::new();

        service
            .send_verification_email(
                "test@example.com",
                Some("Test User"),
                "test-token-123",
                "https://example.com",
            )
            .await
            .unwrap();

        assert_eq!(service.count(), 1);

        let emails = service.get_sent_emails();
        assert_eq!(emails[0].to_email, "test@example.com");
        assert_eq!(emails[0].to_name, Some("Test User".to_string()));
        assert_eq!(emails[0].verification_token, "test-token-123");

        service.clear();
        assert_eq!(service.count(), 0);
    }
}
