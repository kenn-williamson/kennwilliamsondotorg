use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for access request notifications
///
/// Notifies administrators when a user requests elevated access
#[derive(Template)]
#[template(path = "emails/access_request.html")]
pub struct AccessRequestNotificationTemplate {
    /// Display name of the user requesting access
    pub user_display_name: String,

    /// User's message explaining why they need access (optional, HTML-escaped by Askama)
    pub request_message: Option<String>,

    /// The role being requested (e.g., "admin", "moderator")
    pub requested_role: String,

    /// URL to the admin panel for reviewing the request
    pub admin_panel_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl AccessRequestNotificationTemplate {
    /// Create a new access request notification email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `request_message` - Optional message from the user
    /// * `requested_role` - Role being requested
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    pub fn new(
        user_display_name: impl Into<String>,
        request_message: Option<String>,
        requested_role: impl Into<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let admin_panel_url = format!("{}/admin/access-requests", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            request_message,
            requested_role: requested_role.into(),
            admin_panel_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for AccessRequestNotificationTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        let message_section = if let Some(msg) = &self.request_message {
            format!("\n\nUser's Message:\n\"{}\"\n", msg)
        } else {
            String::new()
        };

        format!(
            r#"New Access Request

A user has requested elevated access to KennWilliamson.org.

REQUEST DETAILS:
- User: {}
- Requested Role: {}{}

Please review this request in the admin panel:
{}

NOTE: You can approve or deny this request from the admin panel. The user will be notified of your decision.

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.requested_role, message_section, self.admin_panel_url
        )
    }

    fn subject(&self) -> String {
        format!(
            "New Access Request from {} - KennWilliamson.org",
            self.user_display_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_request_notification_renders_html() {
        let template = AccessRequestNotificationTemplate::new(
            "John Doe",
            Some("I need access to moderate content.".to_string()),
            "moderator",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("John Doe"));
        assert!(html.contains("moderator"));
        assert!(html.contains("I need access to moderate content."));
        assert!(html.contains("https://kennwilliamson.org/admin/access-requests"));
    }

    #[test]
    fn test_access_request_notification_renders_plain_text() {
        let template = AccessRequestNotificationTemplate::new(
            "Jane Smith",
            Some("Need admin access for testing.".to_string()),
            "admin",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Jane Smith"));
        assert!(text.contains("admin"));
        assert!(text.contains("Need admin access for testing."));
        assert!(text.contains("https://kennwilliamson.org/admin/access-requests"));
    }

    #[test]
    fn test_access_request_notification_without_message() {
        let template = AccessRequestNotificationTemplate::new(
            "Test User",
            None,
            "moderator",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");
        let text = template.render_plain_text();

        // Should render without errors even without a message
        assert!(html.contains("Test User"));
        assert!(text.contains("Test User"));
    }

    #[test]
    fn test_access_request_notification_subject() {
        let template = AccessRequestNotificationTemplate::new(
            "Alice Johnson",
            None,
            "admin",
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(
            subject,
            "New Access Request from Alice Johnson - KennWilliamson.org"
        );
    }

    #[test]
    fn test_admin_panel_url_construction() {
        let template = AccessRequestNotificationTemplate::new(
            "User",
            None,
            "admin",
            "https://example.com/",
        );

        // Should trim trailing slash from frontend_url
        assert_eq!(
            template.admin_panel_url,
            "https://example.com/admin/access-requests"
        );
    }

    #[test]
    fn test_xss_prevention_in_user_display_name() {
        let template = AccessRequestNotificationTemplate::new(
            "<script>alert('xss')</script>",
            None,
            "admin",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags (uses numeric entities)
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_xss_prevention_in_request_message() {
        let template = AccessRequestNotificationTemplate::new(
            "User",
            Some("<img src=x onerror=alert('xss')>".to_string()),
            "admin",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the malicious HTML (uses numeric entities)
        assert!(!html.contains("<img src=x"));
        assert!(html.contains("&#60;img") || html.contains("&lt;img"));
    }

    #[test]
    fn test_xss_prevention_in_requested_role() {
        let template = AccessRequestNotificationTemplate::new(
            "User",
            None,
            "<script>alert('xss')</script>",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags (uses numeric entities)
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }
}
