use anyhow::Result;
use async_trait::async_trait;

#[cfg(feature = "mocks")]
pub mod mock_email_service;
pub mod ses_email_service;
pub mod suppression_guard;
pub mod templates;

#[cfg(feature = "mocks")]
pub use mock_email_service::MockEmailService;
pub use ses_email_service::SesEmailService;
pub use suppression_guard::SuppressionGuard;
pub use templates::{Email, EmailTemplate};

/// Generic email service trait for sending emails
/// Allows easy swapping between providers (SES, SMTP, SendGrid, etc.)
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send a generic email using the Email struct
    ///
    /// This is the primary method for sending emails. It provides the most
    /// flexibility and should be used for all new email types.
    ///
    /// # Arguments
    /// * `email` - Email struct containing all email details (to, subject, body, etc.)
    ///
    /// # Returns
    /// * `Ok(())` - Email sent successfully (or queued for sending)
    /// * `Err(_)` - Failed to send email (network error, invalid email, etc.)
    async fn send_email(&self, email: Email) -> Result<()>;
}
