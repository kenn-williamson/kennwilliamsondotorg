use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use aws_sdk_sesv2::{types::{Body, Content, Destination, EmailContent, Message}, Client};

use super::EmailService;
use crate::models::db::EmailType;
use crate::repositories::traits::email_suppression_repository::EmailSuppressionRepository;

/// AWS SES email service implementation
pub struct SesEmailService {
    from_email: String,
    reply_to_email: Option<String>,
    suppression_repo: Option<Box<dyn EmailSuppressionRepository>>,
}

impl SesEmailService {

    /// Create a new SES email service with suppression checking
    pub fn with_suppression(
        from_email: String,
        reply_to_email: Option<String>,
        suppression_repo: Box<dyn EmailSuppressionRepository>,
    ) -> Self {
        Self {
            from_email,
            reply_to_email,
            suppression_repo: Some(suppression_repo),
        }
    }

    /// Create AWS SES client from environment
    async fn create_ses_client() -> Client {
        let config = aws_config::load_from_env().await;
        Client::new(&config)
    }

    /// Create email body content for verification email
    fn create_verification_email_body(to_name: Option<&str>, verification_url: &str) -> String {
        let greeting = if let Some(name) = to_name {
            format!("Hello {},", name)
        } else {
            "Hello,".to_string()
        };

        format!(
            r#"{greeting}

Thank you for registering at KennWilliamson.org!

Please verify your email address by clicking the link below:

{verification_url}

This link will expire in 24 hours.

If you did not create an account, please ignore this email.

---
KennWilliamson.org
"#,
            greeting = greeting,
            verification_url = verification_url
        )
    }

    /// Create email body content for password reset email
    fn create_password_reset_email_body(to_name: Option<&str>, reset_url: &str) -> String {
        let greeting = if let Some(name) = to_name {
            format!("Hello {},", name)
        } else {
            "Hello,".to_string()
        };

        format!(
            r#"{greeting}

We received a request to reset your password for your KennWilliamson.org account.

Click the link below to reset your password:

{reset_url}

This link will expire in 1 hour for security reasons.

If you did not request a password reset, please ignore this email and your password will remain unchanged.

---
KennWilliamson.org
"#,
            greeting = greeting,
            reset_url = reset_url
        )
    }
}

#[async_trait]
impl EmailService for SesEmailService {
    async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: Option<&str>,
        verification_token: &str,
        frontend_url: &str,
    ) -> Result<()> {
        // Check suppression list before sending (if repository is available)
        if let Some(ref suppression_repo) = self.suppression_repo {
            let is_suppressed = suppression_repo
                .is_email_suppressed(to_email, EmailType::Transactional)
                .await
                .context("Failed to check email suppression status")?;

            if is_suppressed {
                log::warn!(
                    "Email blocked by suppression list: {} (transactional)",
                    to_email
                );
                return Err(anyhow!(
                    "Email address is suppressed and cannot receive emails"
                ));
            }
        }

        let verification_url =
            format!("{}/verify-email?token={}", frontend_url, verification_token);
        let email_body = Self::create_verification_email_body(to_name, &verification_url);

        log::info!(
            "SES email service: Sending verification email to {}",
            to_email
        );

        // Create SES client (credentials loaded from environment or EC2 instance role)
        let ses_client = Self::create_ses_client().await;

        // Build SES email message
        let destination = Destination::builder()
            .to_addresses(to_email)
            .build();

        let subject_content = Content::builder()
            .data("Verify your email address")
            .charset("UTF-8")
            .build()
            .context("Failed to build email subject")?;

        let body_content = Content::builder()
            .data(email_body)
            .charset("UTF-8")
            .build()
            .context("Failed to build email body")?;

        let body = Body::builder().text(body_content).build();

        let message = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder()
            .simple(message)
            .build();

        // Build and send email request
        let mut email_request = ses_client
            .send_email()
            .from_email_address(&self.from_email)
            .destination(destination)
            .content(email_content);

        if let Some(reply_to) = &self.reply_to_email {
            email_request = email_request.reply_to_addresses(reply_to);
        }

        email_request.send().await
            .context("Failed to send email via AWS SES")?;

        log::info!("Verification email sent successfully to {}", to_email);

        Ok(())
    }

    async fn send_password_reset_email(
        &self,
        to_email: &str,
        to_name: Option<&str>,
        reset_token: &str,
        frontend_url: &str,
    ) -> Result<()> {
        // Check suppression list before sending (if repository is available)
        if let Some(ref suppression_repo) = self.suppression_repo {
            let is_suppressed = suppression_repo
                .is_email_suppressed(to_email, EmailType::Transactional)
                .await
                .context("Failed to check email suppression status")?;

            if is_suppressed {
                log::warn!(
                    "Email blocked by suppression list: {} (transactional)",
                    to_email
                );
                return Err(anyhow!(
                    "Email address is suppressed and cannot receive emails"
                ));
            }
        }

        let reset_url =
            format!("{}/reset-password?token={}", frontend_url, reset_token);
        let email_body = Self::create_password_reset_email_body(to_name, &reset_url);

        log::info!(
            "SES email service: Sending password reset email to {}",
            to_email
        );

        // Create SES client (credentials loaded from environment or EC2 instance role)
        let ses_client = Self::create_ses_client().await;

        // Build SES email message
        let destination = Destination::builder()
            .to_addresses(to_email)
            .build();

        let subject_content = Content::builder()
            .data("Reset your password")
            .charset("UTF-8")
            .build()
            .context("Failed to build email subject")?;

        let body_content = Content::builder()
            .data(email_body)
            .charset("UTF-8")
            .build()
            .context("Failed to build email body")?;

        let body = Body::builder().text(body_content).build();

        let message = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder()
            .simple(message)
            .build();

        // Build and send email request
        let mut email_request = ses_client
            .send_email()
            .from_email_address(&self.from_email)
            .destination(destination)
            .content(email_content);

        if let Some(reply_to) = &self.reply_to_email {
            email_request = email_request.reply_to_addresses(reply_to);
        }

        email_request.send().await
            .context("Failed to send email via AWS SES")?;

        log::info!("Password reset email sent successfully to {}", to_email);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_verification_email_body() {
        let body = SesEmailService::create_verification_email_body(
            Some("John Doe"),
            "https://example.com/verify?token=abc123",
        );

        assert!(body.contains("Hello John Doe,"));
        assert!(body.contains("https://example.com/verify?token=abc123"));
        assert!(body.contains("24 hours"));
    }

    #[test]
    fn test_create_verification_email_body_no_name() {
        let body = SesEmailService::create_verification_email_body(
            None,
            "https://example.com/verify?token=abc123",
        );

        assert!(body.contains("Hello,"));
        assert!(!body.contains("Hello ,"));
    }
}
