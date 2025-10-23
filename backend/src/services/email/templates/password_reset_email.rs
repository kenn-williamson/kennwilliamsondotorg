use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for password reset
///
/// Sends a secure password reset link to users
#[derive(Template)]
#[template(path = "emails/password_reset.html")]
pub struct PasswordResetEmailTemplate {
    /// Recipient's display name
    pub to_name: String,

    /// Full URL for password reset (includes token)
    pub reset_url: String,
}

impl PasswordResetEmailTemplate {
    /// Create a new password reset email template
    ///
    /// # Arguments
    /// * `to_name` - Recipient's display name
    /// * `reset_token` - The password reset token
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    pub fn new(to_name: impl Into<String>, reset_token: &str, frontend_url: &str) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let reset_url = format!("{}/reset-password?token={}", frontend_base, reset_token);

        Self {
            to_name: to_name.into(),
            reset_url,
        }
    }
}

impl EmailTemplate for PasswordResetEmailTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        format!(
            r#"Password Reset Request

Hello {},

We received a request to reset the password for your KennWilliamson.org account. Visit the following link to create a new password:

{}

IMPORTANT SECURITY NOTICE:
- This password reset link will expire in 1 hour
- For security, this link can only be used once
- If you didn't request this reset, please ignore this email

If you didn't request a password reset, your account is still secure. You can safely ignore this email.

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.to_name, self.reset_url
        )
    }

    fn subject(&self) -> String {
        "Reset Your Password - KennWilliamson.org".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_reset_email_renders_html() {
        let template = PasswordResetEmailTemplate::new(
            "John Doe",
            "reset-token-123",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("John Doe"));
        assert!(html.contains("https://kennwilliamson.org/reset-password?token=reset-token-123"));
        assert!(html.contains("Reset Password"));
        assert!(html.contains("1 hour"));
    }

    #[test]
    fn test_password_reset_email_renders_plain_text() {
        let template = PasswordResetEmailTemplate::new(
            "Jane Smith",
            "reset-token-456",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Jane Smith"));
        assert!(text.contains("https://kennwilliamson.org/reset-password?token=reset-token-456"));
        assert!(text.contains("1 hour"));
    }

    #[test]
    fn test_password_reset_email_subject() {
        let template = PasswordResetEmailTemplate::new(
            "Test User",
            "token",
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(subject, "Reset Your Password - KennWilliamson.org");
    }

    #[test]
    fn test_reset_url_construction() {
        let template =
            PasswordResetEmailTemplate::new("User", "my-token", "https://example.com/");

        // Should trim trailing slash from frontend_url
        assert_eq!(
            template.reset_url,
            "https://example.com/reset-password?token=my-token"
        );
    }

    #[test]
    fn test_xss_prevention_in_name() {
        let template = PasswordResetEmailTemplate::new(
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
