use anyhow::Result;
use async_trait::async_trait;

use super::EmailService;

/// AWS SES email service implementation
pub struct SesEmailService {
    from_email: String,
    reply_to_email: Option<String>,
    // AWS SES client will be added when we add aws-sdk-sesv2 dependency
    // ses_client: aws_sdk_sesv2::Client,
}

impl SesEmailService {
    pub fn new(from_email: String, reply_to_email: Option<String>) -> Self {
        Self {
            from_email,
            reply_to_email,
        }
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
        let verification_url =
            format!("{}/verify-email?token={}", frontend_url, verification_token);
        let email_body = Self::create_verification_email_body(to_name, &verification_url);

        log::info!(
            "SES email service: Sending verification email to {}",
            to_email
        );

        // TODO: Implement actual AWS SES sending when aws-sdk-sesv2 is added
        // For now, log the email content for development
        log::debug!("Email body:\n{}", email_body);
        log::warn!("AWS SES not yet implemented - email logged but not sent");

        // Temporary: Return Ok for development
        // This will be replaced with actual SES API call:
        /*
        let destination = Destination::builder()
            .to_addresses(to_email)
            .build();

        let subject = Content::builder()
            .data("Verify your email address")
            .charset("UTF-8")
            .build()?;

        let body_content = Content::builder()
            .data(email_body)
            .charset("UTF-8")
            .build()?;

        let body = Body::builder().text(body_content).build();

        let message = Message::builder()
            .subject(subject)
            .body(body)
            .build();

        let mut email_request = self.ses_client
            .send_email()
            .source(&self.from_email)
            .destination(destination)
            .message(message);

        if let Some(reply_to) = &self.reply_to_email {
            email_request = email_request.reply_to_addresses(reply_to);
        }

        email_request.send().await
            .context("Failed to send email via AWS SES")?;
        */

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
