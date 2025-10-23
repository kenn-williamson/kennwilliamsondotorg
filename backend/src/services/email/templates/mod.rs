use anyhow::Result;

pub mod access_request_notification;
pub mod password_reset_email;
pub mod phrase_suggestion;
pub mod verification_email;

pub use access_request_notification::AccessRequestNotificationTemplate;
pub use password_reset_email::PasswordResetEmailTemplate;
pub use phrase_suggestion::PhraseSuggestionNotificationTemplate;
pub use verification_email::VerificationEmailTemplate;

/// Trait for email templates that can render to both HTML and plain text
///
/// This trait provides a consistent interface for all email types, enabling:
/// - Type-safe template rendering with compile-time validation (via Askama)
/// - Automatic HTML escaping for XSS prevention
/// - Clean separation between content generation and email delivery
pub trait EmailTemplate: Send + Sync {
    /// Render the email template to HTML
    ///
    /// Returns a Result because template rendering can fail if the template
    /// has syntax errors or is missing required files
    fn render_html(&self) -> Result<String>;

    /// Render the email template to plain text
    ///
    /// Plain text version is used as a fallback for email clients that
    /// don't support HTML, and for improved deliverability
    fn render_plain_text(&self) -> String;

    /// Get the email subject line
    fn subject(&self) -> String;
}

/// Transport-agnostic email representation
///
/// This struct decouples email content from the delivery mechanism,
/// allowing the same Email struct to be used with different email
/// service providers (AWS SES, SMTP, SendGrid, etc.)
#[derive(Debug, Clone)]
pub struct Email {
    /// Recipient email addresses
    pub to: Vec<String>,

    /// Email subject line
    pub subject: String,

    /// HTML body (optional, but recommended for modern email clients)
    pub html_body: Option<String>,

    /// Plain text body (fallback for clients that don't support HTML)
    pub text_body: String,

    /// Optional reply-to address
    pub reply_to: Option<String>,
}

impl Email {
    /// Create a builder for constructing an Email
    pub fn builder() -> EmailBuilder {
        EmailBuilder::default()
    }
}

/// Builder for Email struct
///
/// Provides a flexible API for constructing emails with optional fields
#[derive(Default)]
pub struct EmailBuilder {
    to: Vec<String>,
    subject: Option<String>,
    html_body: Option<String>,
    text_body: Option<String>,
    reply_to: Option<String>,
}

impl EmailBuilder {
    /// Add a recipient email address
    pub fn to(mut self, email: impl Into<String>) -> Self {
        self.to.push(email.into());
        self
    }

    /// Add multiple recipient email addresses
    pub fn with_recipients(mut self, emails: Vec<String>) -> Self {
        self.to = emails;
        self
    }

    /// Set the email subject line
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set the HTML body
    pub fn html_body(mut self, html: impl Into<String>) -> Self {
        self.html_body = Some(html.into());
        self
    }

    /// Set the plain text body
    pub fn text_body(mut self, text: impl Into<String>) -> Self {
        self.text_body = Some(text.into());
        self
    }

    /// Set the reply-to address
    pub fn reply_to(mut self, email: impl Into<String>) -> Self {
        self.reply_to = Some(email.into());
        self
    }

    /// Build the Email struct
    ///
    /// # Errors
    /// Returns an error if required fields are missing
    pub fn build(self) -> Result<Email> {
        let subject = self
            .subject
            .ok_or_else(|| anyhow::anyhow!("Email subject is required"))?;

        let text_body = self
            .text_body
            .ok_or_else(|| anyhow::anyhow!("Email text body is required"))?;

        if self.to.is_empty() {
            return Err(anyhow::anyhow!("At least one recipient is required"));
        }

        Ok(Email {
            to: self.to,
            subject,
            html_body: self.html_body,
            text_body,
            reply_to: self.reply_to,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_builder_success() {
        let email = Email::builder()
            .to("user@example.com")
            .subject("Test Subject")
            .text_body("Plain text body")
            .html_body("<p>HTML body</p>")
            .reply_to("noreply@example.com")
            .build()
            .expect("Failed to build email");

        assert_eq!(email.to, vec!["user@example.com"]);
        assert_eq!(email.subject, "Test Subject");
        assert_eq!(email.text_body, "Plain text body");
        assert_eq!(email.html_body, Some("<p>HTML body</p>".to_string()));
        assert_eq!(email.reply_to, Some("noreply@example.com".to_string()));
    }

    #[test]
    fn test_email_builder_multiple_recipients() {
        let email = Email::builder()
            .with_recipients(vec![
                "user1@example.com".to_string(),
                "user2@example.com".to_string(),
            ])
            .subject("Test Subject")
            .text_body("Plain text body")
            .build()
            .expect("Failed to build email");

        assert_eq!(email.to.len(), 2);
        assert_eq!(email.to[0], "user1@example.com");
        assert_eq!(email.to[1], "user2@example.com");
    }

    #[test]
    fn test_email_builder_missing_subject() {
        let result = Email::builder()
            .to("user@example.com")
            .text_body("Plain text body")
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("subject is required"));
    }

    #[test]
    fn test_email_builder_missing_text_body() {
        let result = Email::builder()
            .to("user@example.com")
            .subject("Test Subject")
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("text body is required"));
    }

    #[test]
    fn test_email_builder_no_recipients() {
        let result = Email::builder()
            .subject("Test Subject")
            .text_body("Plain text body")
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("At least one recipient is required"));
    }

    #[test]
    fn test_email_builder_without_optional_fields() {
        let email = Email::builder()
            .to("user@example.com")
            .subject("Test Subject")
            .text_body("Plain text body")
            .build()
            .expect("Failed to build email");

        assert_eq!(email.html_body, None);
        assert_eq!(email.reply_to, None);
    }
}
