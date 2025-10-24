use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for access request approval notifications
///
/// Notifies users when their access request has been approved
#[derive(Template)]
#[template(path = "emails/access_request_approved.html")]
pub struct AccessRequestApprovedTemplate {
    /// Display name of the user
    pub user_display_name: String,

    /// The role that was granted
    pub granted_role: String,

    /// Optional message from the admin explaining the approval
    pub admin_message: Option<String>,

    /// URL to the user's profile or relevant page
    pub profile_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl AccessRequestApprovedTemplate {
    /// Create a new access request approved email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `granted_role` - Role that was granted
    /// * `admin_message` - Optional message from the admin
    /// * `frontend_url` - Base URL of the frontend
    pub fn new(
        user_display_name: impl Into<String>,
        granted_role: impl Into<String>,
        admin_message: Option<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let profile_url = format!("{}/profile", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            granted_role: granted_role.into(),
            admin_message,
            profile_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for AccessRequestApprovedTemplate {
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
            r#"Access Request Approved!

Congratulations, {}! Your access request has been approved.

GRANT DETAILS:
- Role Granted: {}{}

You now have access to additional features on KennWilliamson.org. You can view your profile and permissions here:
{}

Thank you for being part of our community!

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.granted_role, admin_section, self.profile_url
        )
    }

    fn subject(&self) -> String {
        format!("Access Request Approved - {}", self.granted_role)
    }
}
