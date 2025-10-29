use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for phrase suggestion notifications
///
/// Notifies administrators when a user suggests a new phrase
#[derive(Template)]
#[template(path = "emails/phrase_suggestion.html")]
pub struct PhraseSuggestionNotificationTemplate {
    /// Display name of the user who suggested the phrase
    pub user_display_name: String,

    /// The suggested phrase text
    pub phrase_text: String,

    /// URL to the admin panel for reviewing the suggestion
    pub admin_panel_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl PhraseSuggestionNotificationTemplate {
    /// Create a new phrase suggestion notification email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `phrase_text` - The suggested phrase
    /// * `frontend_url` - Base URL of the frontend (e.g., "https://kennwilliamson.org")
    pub fn new(
        user_display_name: impl Into<String>,
        phrase_text: impl Into<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let admin_panel_url = format!("{}/admin?tab=suggestions", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            phrase_text: phrase_text.into(),
            admin_panel_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for PhraseSuggestionNotificationTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        format!(
            r#"New Phrase Suggestion

A user has suggested a new phrase for KennWilliamson.org.

SUGGESTION DETAILS:
- Suggested by: {}
- Phrase: "{}"

Please review this suggestion in the admin panel:
{}

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.phrase_text, self.admin_panel_url
        )
    }

    fn subject(&self) -> String {
        format!(
            "New Phrase Suggestion from {} - KennWilliamson.org",
            self.user_display_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_suggestion_notification_renders_html() {
        let template = PhraseSuggestionNotificationTemplate::new(
            "Jane Doe",
            "Time is an illusion. Lunchtime doubly so.",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("Jane Doe"));
        assert!(html.contains("Time is an illusion"));
        assert!(html.contains("https://kennwilliamson.org/admin?tab=suggestions"));
        assert!(html.contains("Review Suggestion"));
    }

    #[test]
    fn test_phrase_suggestion_notification_renders_plain_text() {
        let template = PhraseSuggestionNotificationTemplate::new(
            "John Smith",
            "Don't panic!",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("John Smith"));
        assert!(text.contains("Don't panic!"));
        assert!(text.contains("https://kennwilliamson.org/admin?tab=suggestions"));
    }

    #[test]
    fn test_phrase_suggestion_notification_subject() {
        let template = PhraseSuggestionNotificationTemplate::new(
            "Test User",
            "Test phrase",
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(
            subject,
            "New Phrase Suggestion from Test User - KennWilliamson.org"
        );
    }

    #[test]
    fn test_xss_prevention_in_user_display_name() {
        let template = PhraseSuggestionNotificationTemplate::new(
            "<script>alert('xss')</script>",
            "Normal phrase",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags (uses numeric entities)
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_xss_prevention_in_phrase_text() {
        let template = PhraseSuggestionNotificationTemplate::new(
            "User",
            "<img src=x onerror=alert('xss')>",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the malicious HTML (uses numeric entities)
        assert!(!html.contains("<img src=x"));
        assert!(html.contains("&#60;img") || html.contains("&lt;img"));
    }
}
