use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for access request rejection notifications
///
/// Notifies users when their access request has been rejected
#[derive(Template)]
#[template(path = "emails/access_request_rejected.html")]
pub struct AccessRequestRejectedTemplate {
    /// Display name of the user
    pub user_display_name: String,

    /// Optional message from the admin explaining the rejection
    pub admin_message: Option<String>,

    /// URL to request access again or learn more
    pub home_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl AccessRequestRejectedTemplate {
    /// Create a new access request rejected email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `admin_message` - Optional message from the admin
    /// * `frontend_url` - Base URL of the frontend
    pub fn new(
        user_display_name: impl Into<String>,
        admin_message: Option<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let home_url = format!("{}/", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            admin_message,
            home_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for AccessRequestRejectedTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        let admin_section = if let Some(msg) = &self.admin_message {
            format!("\n\nMessage from Admin:\n\"{}\"\n", msg)
        } else {
            String::new()
        };

        format!(
            r#"Access Request Update

Hello {},

Thank you for your interest in elevated access to KennWilliamson.org.

After review, we're unable to approve your access request at this time.{}

You're welcome to continue using the public features of our site, and we appreciate your understanding.

If you have questions or would like to submit a new request in the future, please don't hesitate to reach out.

Visit our site:
{}

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, admin_section, self.home_url
        )
    }

    fn subject(&self) -> String {
        "Access Request Update - KennWilliamson.org".to_string()
    }
}
