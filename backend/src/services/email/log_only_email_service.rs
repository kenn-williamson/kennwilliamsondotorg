use anyhow::Result;
use async_trait::async_trait;

use super::{Email, EmailService};

/// Log-only email service for development
/// Logs email details to console instead of sending them
///
/// This is useful for development when you want to verify email functionality
/// without actually sending emails through AWS SES.
#[derive(Debug, Clone)]
pub struct LogOnlyEmailService;

impl LogOnlyEmailService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LogOnlyEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for LogOnlyEmailService {
    async fn send_email(&self, email: Email) -> Result<()> {
        log::info!(
            "📧 [DEV MODE] Email NOT sent (log-only mode enabled):\n  To: {}\n  Subject: {}\n  Body preview: {}",
            email.to.join(", "),
            email.subject,
            email.text_body.chars().take(100).collect::<String>()
        );

        Ok(())
    }
}
