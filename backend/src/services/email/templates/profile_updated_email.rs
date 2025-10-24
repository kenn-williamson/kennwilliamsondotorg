use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for profile updated notification
///
/// Sends a security alert to users when their profile is updated
#[derive(Template)]
#[template(path = "emails/profile_updated.html")]
pub struct ProfileUpdatedEmailTemplate {
    /// Recipient's current display name
    pub user_display_name: String,

    /// Previous display name (if changed)
    pub old_display_name: Option<String>,

    /// New display name
    pub new_display_name: String,

    /// Previous slug/username (if changed)
    pub old_slug: Option<String>,

    /// New slug/username
    pub new_slug: String,

    /// Formatted timestamp when profile was updated
    pub updated_at: String,

    /// URL to user's profile page
    pub profile_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl ProfileUpdatedEmailTemplate {
    /// Create a new profile updated email template
    ///
    /// # Arguments
    /// * `user_display_name` - Recipient's current display name
    /// * `old_display_name` - Previous display name (None if no change)
    /// * `new_display_name` - New display name
    /// * `old_slug` - Previous slug (None if no change)
    /// * `new_slug` - New slug
    /// * `updated_at` - Formatted timestamp (e.g., "January 15, 2025 at 3:45 PM UTC")
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    pub fn new(
        user_display_name: impl Into<String>,
        old_display_name: Option<String>,
        new_display_name: impl Into<String>,
        old_slug: Option<String>,
        new_slug: impl Into<String>,
        updated_at: impl Into<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let new_slug_val = new_slug.into();
        let profile_url = format!("{}/user/{}", frontend_base, new_slug_val);

        Self {
            user_display_name: user_display_name.into(),
            old_display_name,
            new_display_name: new_display_name.into(),
            old_slug,
            new_slug: new_slug_val,
            updated_at: updated_at.into(),
            profile_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for ProfileUpdatedEmailTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        let mut changes = String::new();

        if let Some(old_name) = &self.old_display_name {
            changes.push_str(&format!(
                "Display Name: {} → {}\n",
                old_name, self.new_display_name
            ));
        } else {
            changes.push_str(&format!("Display Name: {}\n", self.new_display_name));
        }

        if let Some(old_slug) = &self.old_slug {
            changes.push_str(&format!("Username: {} → {}\n", old_slug, self.new_slug));
        } else {
            changes.push_str(&format!("Username: {}\n", self.new_slug));
        }

        format!(
            r#"Profile Updated Successfully

Hello {},

Your profile on KennWilliamson.org was successfully updated on {}.

CHANGES MADE:
{}

View your updated profile: {}

IF YOU DIDN'T MAKE THIS CHANGE:
Someone may have accessed your account. We recommend changing your password immediately and reviewing your account activity.

This is an automated security notification. For your protection, we send this email whenever your profile is updated.

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.updated_at, changes, self.profile_url
        )
    }

    fn subject(&self) -> String {
        "Your Profile Was Updated - KennWilliamson.org".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_updated_email_renders_html() {
        let template = ProfileUpdatedEmailTemplate::new(
            "John Doe",
            Some("Old Name".to_string()),
            "John Doe",
            Some("old-slug".to_string()),
            "john-doe",
            "January 15, 2025 at 3:45 PM UTC",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("John Doe"));
        assert!(html.contains("Old Name"));
        assert!(html.contains("old-slug"));
        assert!(html.contains("john-doe"));
        assert!(html.contains("January 15, 2025 at 3:45 PM UTC"));
        assert!(html.contains("Profile Updated"));
        assert!(html.contains("https://kennwilliamson.org/user/john-doe"));
    }

    #[test]
    fn test_profile_updated_email_renders_plain_text() {
        let template = ProfileUpdatedEmailTemplate::new(
            "Jane Smith",
            Some("Old Name".to_string()),
            "Jane Smith",
            Some("old-slug".to_string()),
            "jane-smith",
            "January 15, 2025 at 3:45 PM UTC",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Jane Smith"));
        assert!(text.contains("Old Name"));
        assert!(text.contains("old-slug"));
        assert!(text.contains("jane-smith"));
        assert!(text.contains("January 15, 2025 at 3:45 PM UTC"));
        assert!(text.contains("https://kennwilliamson.org/user/jane-smith"));
    }

    #[test]
    fn test_profile_updated_email_subject() {
        let template = ProfileUpdatedEmailTemplate::new(
            "Test User",
            None,
            "Test User",
            None,
            "test-user",
            "Now",
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(subject, "Your Profile Was Updated - KennWilliamson.org");
    }

    #[test]
    fn test_profile_url_construction() {
        let template = ProfileUpdatedEmailTemplate::new(
            "User",
            None,
            "User",
            None,
            "my-slug",
            "Now",
            "https://example.com/",
        );

        // Should trim trailing slash from frontend_url
        assert_eq!(template.profile_url, "https://example.com/user/my-slug");
    }

    #[test]
    fn test_profile_updated_without_old_values() {
        let template = ProfileUpdatedEmailTemplate::new(
            "First User",
            None,
            "First User",
            None,
            "first-user",
            "Now",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("First User"));
        assert!(html.contains("first-user"));
        // Should not show strikethrough arrows if no old values
        assert!(!html.contains("→"));
    }

    #[test]
    fn test_xss_prevention_in_names() {
        let template = ProfileUpdatedEmailTemplate::new(
            "<script>alert('xss')</script>",
            Some("<script>old</script>".to_string()),
            "<script>new</script>",
            Some("<script>old</script>".to_string()),
            "safe-slug",
            "Now",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags
        assert!(!html.contains("<script>"));
        let escaped = html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;");
        assert!(escaped);
    }

    #[test]
    fn test_plain_text_with_no_old_values() {
        let template = ProfileUpdatedEmailTemplate::new(
            "User",
            None,
            "User",
            None,
            "user-slug",
            "Now",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(!text.contains("→"));
        assert!(text.contains("Display Name: User"));
        assert!(text.contains("Username: user-slug"));
    }

    #[test]
    fn test_plain_text_with_old_values() {
        let template = ProfileUpdatedEmailTemplate::new(
            "New Name",
            Some("Old Name".to_string()),
            "New Name",
            Some("old-slug".to_string()),
            "new-slug",
            "Now",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Old Name → New Name"));
        assert!(text.contains("old-slug → new-slug"));
    }
}
