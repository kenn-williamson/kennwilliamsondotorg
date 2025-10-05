use anyhow::Result;
use async_trait::async_trait;

#[cfg(feature = "mocks")]
pub mod mock_email_service;
pub mod ses_email_service;

#[cfg(feature = "mocks")]
pub use mock_email_service::MockEmailService;
pub use ses_email_service::SesEmailService;

/// Generic email service trait for sending emails
/// Allows easy swapping between providers (SES, SMTP, SendGrid, etc.)
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send an email verification email with token
    ///
    /// # Arguments
    /// * `to_email` - Recipient email address
    /// * `to_name` - Recipient display name (optional, for personalization)
    /// * `verification_token` - The verification token to include in the link
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: Option<&str>,
        verification_token: &str,
        frontend_url: &str,
    ) -> Result<()>;
}
