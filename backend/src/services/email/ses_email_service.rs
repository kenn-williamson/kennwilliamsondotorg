use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_sdk_sesv2::{types::{Body, Content, Destination, EmailContent, Message}, Client};

use super::{Email, EmailService};

/// AWS SES email service implementation
///
/// This service handles email delivery via AWS Simple Email Service (SES).
/// It does NOT check suppression lists - wrap with SuppressionGuard for that functionality.
///
/// # Example
///
/// ```rust,no_run
/// use backend::services::email::{SesEmailService, SuppressionGuard};
/// use backend::repositories::postgres::postgres_email_suppression_repository::PostgresEmailSuppressionRepository;
///
/// // Production: Wrap with SuppressionGuard
/// # fn example() {
/// # let from_email = "test@example.com".to_string();
/// # let reply_to = None;
/// # let config_set = None;
/// # let pool = todo!();
/// let ses = SesEmailService::new(from_email, reply_to, config_set);
/// let suppression_repo = Box::new(PostgresEmailSuppressionRepository::new(pool));
/// let guarded = SuppressionGuard::new(Box::new(ses), suppression_repo);
/// # }
/// ```
pub struct SesEmailService {
    from_email: String,
    reply_to_email: Option<String>,
    configuration_set_name: Option<String>,
}

impl SesEmailService {
    /// Create a new SES email service
    ///
    /// # Arguments
    ///
    /// * `from_email` - The "From" email address for all emails
    /// * `reply_to_email` - Optional "Reply-To" email address
    /// * `configuration_set_name` - Optional SES configuration set for bounce/complaint tracking
    pub fn new(
        from_email: String,
        reply_to_email: Option<String>,
        configuration_set_name: Option<String>,
    ) -> Self {
        Self {
            from_email,
            reply_to_email,
            configuration_set_name,
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
        log::info!(
            "Sending email via AWS SES: '{}' to {} recipient(s)",
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

        // Add configuration set if specified (for bounce/complaint tracking)
        if let Some(config_set) = &self.configuration_set_name {
            email_request = email_request.configuration_set_name(config_set);
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

