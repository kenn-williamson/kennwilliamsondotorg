use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for password changed notification
///
/// Sends a security alert to users when their password is changed
#[derive(Template)]
#[template(path = "emails/password_changed.html")]
pub struct PasswordChangedEmailTemplate {
    /// Recipient's display name
    pub user_display_name: String,

    /// Formatted timestamp when password was changed
    pub changed_at: String,

    /// URL for password reset (if unauthorized change)
    pub password_reset_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl PasswordChangedEmailTemplate {
    /// Create a new password changed email template
    ///
    /// # Arguments
    /// * `user_display_name` - Recipient's display name
    /// * `changed_at` - Formatted timestamp (e.g., "January 15, 2025 at 3:45 PM UTC")
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    pub fn new(
        user_display_name: impl Into<String>,
        changed_at: impl Into<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let password_reset_url = format!("{}/forgot-password", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            changed_at: changed_at.into(),
            password_reset_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for PasswordChangedEmailTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        format!(
            r#"Security Alert: Password Changed

Hello {},

Your password for KennWilliamson.org was successfully changed on {}.

✓ YOUR ACCOUNT IS SECURE
This is a confirmation that your password change was successful.

IF YOU DIDN'T MAKE THIS CHANGE:
- Your account may have been compromised
- Reset your password immediately: {}
- Contact support if you need assistance

SECURITY TIPS:
- Use a unique password for each account
- Enable additional security features when available
- Never share your password with anyone

This is an automated security notification. For your protection, we send this email whenever your password is changed.

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.changed_at, self.password_reset_url
        )
    }

    fn subject(&self) -> String {
        "Security Alert: Your Password Was Changed - KennWilliamson.org".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_changed_email_renders_html() {
        let template = PasswordChangedEmailTemplate::new(
            "John Doe",
            "January 15, 2025 at 3:45 PM UTC",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("John Doe"));
        assert!(html.contains("January 15, 2025 at 3:45 PM UTC"));
        assert!(html.contains("Password Changed"));
        assert!(html.contains("https://kennwilliamson.org/forgot-password"));
    }

    #[test]
    fn test_password_changed_email_renders_plain_text() {
        let template = PasswordChangedEmailTemplate::new(
            "Jane Smith",
            "January 15, 2025 at 3:45 PM UTC",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Jane Smith"));
        assert!(text.contains("January 15, 2025 at 3:45 PM UTC"));
        assert!(text.contains("Security Alert"));
        assert!(text.contains("https://kennwilliamson.org/forgot-password"));
    }

    #[test]
    fn test_password_changed_email_subject() {
        let template =
            PasswordChangedEmailTemplate::new("Test User", "Now", "https://kennwilliamson.org");

        let subject = template.subject();

        assert_eq!(
            subject,
            "Security Alert: Your Password Was Changed - KennWilliamson.org"
        );
    }

    #[test]
    fn test_password_reset_url_construction() {
        let template = PasswordChangedEmailTemplate::new("User", "Now", "https://example.com/");

        // Should trim trailing slash from frontend_url
        assert_eq!(
            template.password_reset_url,
            "https://example.com/forgot-password"
        );
    }

    #[test]
    fn test_xss_prevention_in_name() {
        let template = PasswordChangedEmailTemplate::new(
            "<script>alert('xss')</script>",
            "Now",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_xss_prevention_in_timestamp() {
        let template = PasswordChangedEmailTemplate::new(
            "User",
            "<script>alert('xss')</script>",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }
}
