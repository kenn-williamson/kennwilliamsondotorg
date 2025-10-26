use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use aws_sdk_sesv2::{types::{Body, Content, Destination, EmailContent, Message}, Client};

use super::{Email, EmailService};
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

}

#[async_trait]
impl EmailService for SesEmailService {
    async fn send_email(&self, email: Email) -> Result<()> {
        // Check suppression list for all recipients (if repository is available)
        if let Some(ref suppression_repo) = self.suppression_repo {
            for recipient in &email.to {
                let is_suppressed = suppression_repo
                    .is_email_suppressed(recipient, EmailType::Transactional)
                    .await
                    .context("Failed to check email suppression status")?;

                if is_suppressed {
                    log::warn!(
                        "Email blocked by suppression list: {} (transactional)",
                        recipient
                    );
                    return Err(anyhow!(
                        "Email address {} is suppressed and cannot receive emails",
                        recipient
                    ));
                }
            }
        }

        log::info!(
            "SES email service: Sending email '{}' to {} recipient(s)",
            email.subject,
            email.to.len()
        );

        // Create SES client (credentials loaded from environment or EC2 instance role)
        let ses_client = Self::create_ses_client().await;

        // Build destination with all recipients
        let mut destination_builder = Destination::builder();
        for recipient in &email.to {
            destination_builder = destination_builder.to_addresses(recipient);
        }
        let destination = destination_builder.build();

        // Build subject
        let subject_content = Content::builder()
            .data(&email.subject)
            .charset("UTF-8")
            .build()
            .context("Failed to build email subject")?;

        // Build text body (required)
        let text_content = Content::builder()
            .data(&email.text_body)
            .charset("UTF-8")
            .build()
            .context("Failed to build text body")?;

        // Build body with optional HTML
        let mut body_builder = Body::builder().text(text_content);

        if let Some(html_body) = &email.html_body {
            let html_content = Content::builder()
                .data(html_body)
                .charset("UTF-8")
                .build()
                .context("Failed to build HTML body")?;
            body_builder = body_builder.html(html_content);
        }

        let body = body_builder.build();

        // Build message
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

        // Add reply-to if specified in email or service config
        if let Some(reply_to) = &email.reply_to {
            email_request = email_request.reply_to_addresses(reply_to);
        } else if let Some(reply_to) = &self.reply_to_email {
            email_request = email_request.reply_to_addresses(reply_to);
        }

        email_request.send().await
            .context("Failed to send email via AWS SES")?;

        log::info!(
            "Email '{}' sent successfully to {} recipient(s)",
            email.subject,
            email.to.len()
        );

        Ok(())
    }
}

