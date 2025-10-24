use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for email verification
///
/// Sends a verification link to users to confirm their email address
#[derive(Template)]
#[template(path = "emails/verification.html")]
pub struct VerificationEmailTemplate {
    /// Recipient's display name
    pub to_name: String,

    /// Full URL for email verification (includes token)
    pub verification_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl VerificationEmailTemplate {
    /// Create a new verification email template
    ///
    /// # Arguments
    /// * `to_name` - Recipient's display name
    /// * `verification_token` - The verification token
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    pub fn new(to_name: impl Into<String>, verification_token: &str, frontend_url: &str) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let verification_url = format!("{}/verify-email?token={}", frontend_base, verification_token);

        Self {
            to_name: to_name.into(),
            verification_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for VerificationEmailTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        format!(
            r#"Welcome, {}!

Thank you for creating an account with KennWilliamson.org. To complete your registration and verify your email address, please visit the following link:

{}

IMPORTANT: This verification link will expire in 24 hours for security reasons.

If you didn't create an account with KennWilliamson.org, you can safely ignore this email.

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.to_name, self.verification_url
        )
    }

    fn subject(&self) -> String {
        "Verify Your Email Address - KennWilliamson.org".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_email_renders_html() {
        let template = VerificationEmailTemplate::new(
            "John Doe",
            "test-token-123",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("John Doe"));
        assert!(html.contains("https://kennwilliamson.org/verify-email?token=test-token-123"));
        assert!(html.contains("Verify Email Address"));
        assert!(html.contains("24 hours"));
    }

    #[test]
    fn test_verification_email_renders_plain_text() {
        let template = VerificationEmailTemplate::new(
            "Jane Smith",
            "test-token-456",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Jane Smith"));
        assert!(text.contains("https://kennwilliamson.org/verify-email?token=test-token-456"));
        assert!(text.contains("24 hours"));
    }

    #[test]
    fn test_verification_email_subject() {
        let template = VerificationEmailTemplate::new(
            "Test User",
            "token",
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(subject, "Verify Your Email Address - KennWilliamson.org");
    }

    #[test]
    fn test_verification_url_construction() {
        let template = VerificationEmailTemplate::new(
            "User",
            "my-token",
            "https://example.com/",
        );

        // Should trim trailing slash from frontend_url
        assert_eq!(
            template.verification_url,
            "https://example.com/verify-email?token=my-token"
        );
    }

    #[test]
    fn test_xss_prevention_in_name() {
        let template = VerificationEmailTemplate::new(
            "<script>alert('xss')</script>",
            "token",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags (uses numeric entities &#60; instead of &lt;)
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }
}
