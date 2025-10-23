use anyhow::Result;
use async_trait::async_trait;

#[cfg(feature = "mocks")]
pub mod mock_email_service;
pub mod ses_email_service;
pub mod templates;

#[cfg(feature = "mocks")]
pub use mock_email_service::MockEmailService;
pub use ses_email_service::SesEmailService;
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

    /// Send an email verification email with token
    ///
    /// **DEPRECATED**: Use `send_email()` with `VerificationEmailTemplate` instead.
    /// This method is maintained for backward compatibility but will be removed in a future version.
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

    /// Send a password reset email with token
    ///
    /// **DEPRECATED**: Use `send_email()` with `PasswordResetEmailTemplate` instead.
    /// This method is maintained for backward compatibility but will be removed in a future version.
    ///
    /// # Arguments
    /// * `to_email` - Recipient email address
    /// * `to_name` - Recipient display name (optional, for personalization)
    /// * `reset_token` - The password reset token to include in the link
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    async fn send_password_reset_email(
        &self,
        to_email: &str,
        to_name: Option<&str>,
        reset_token: &str,
        frontend_url: &str,
    ) -> Result<()>;
}
