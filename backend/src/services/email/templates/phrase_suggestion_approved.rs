use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for phrase suggestion approval notifications
///
/// Notifies users when their phrase suggestion has been approved
#[derive(Template)]
#[template(path = "emails/phrase_suggestion_approved.html")]
pub struct PhraseSuggestionApprovedTemplate {
    /// Display name of the user
    pub user_display_name: String,

    /// The phrase text that was approved
    pub phrase_text: String,

    /// Optional message from the admin explaining the approval
    pub admin_message: Option<String>,

    /// URL to the phrases page
    pub phrases_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl PhraseSuggestionApprovedTemplate {
    /// Create a new phrase suggestion approved email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `phrase_text` - The phrase text that was approved
    /// * `admin_message` - Optional message from the admin
    /// * `frontend_url` - Base URL of the frontend
    pub fn new(
        user_display_name: impl Into<String>,
        phrase_text: impl Into<String>,
        admin_message: Option<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let phrases_url = format!("{}/phrases", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            phrase_text: phrase_text.into(),
            admin_message,
            phrases_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for PhraseSuggestionApprovedTemplate {
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
            r#"Phrase Suggestion Approved!

Congratulations, {}! Your phrase suggestion has been approved and added to KennWilliamson.org.

YOUR PHRASE:
"{}"{}

Your phrase is now active and will appear for users throughout the site. Thank you for contributing to our collection!

View all phrases here:
{}

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.phrase_text, admin_section, self.phrases_url
        )
    }

    fn subject(&self) -> String {
        "Your Phrase Suggestion Was Approved! - KennWilliamson.org".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_suggestion_approved_renders_html() {
        let template = PhraseSuggestionApprovedTemplate::new(
            "Jane Doe",
            "Time is an illusion",
            Some("Love this phrase!".to_string()),
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("Jane Doe"));
        assert!(html.contains("Time is an illusion"));
        assert!(html.contains("Love this phrase!"));
        assert!(html.contains("https://kennwilliamson.org/phrases"));
    }

    #[test]
    fn test_phrase_suggestion_approved_renders_plain_text() {
        let template = PhraseSuggestionApprovedTemplate::new(
            "John Smith",
            "Don't panic!",
            None,
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("John Smith"));
        assert!(text.contains("Don't panic!"));
        assert!(text.contains("https://kennwilliamson.org/phrases"));
    }

    #[test]
    fn test_phrase_suggestion_approved_subject() {
        let template = PhraseSuggestionApprovedTemplate::new(
            "Test User",
            "Test phrase",
            None,
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(
            subject,
            "Your Phrase Suggestion Was Approved! - KennWilliamson.org"
        );
    }

    #[test]
    fn test_xss_prevention_in_user_display_name() {
        let template = PhraseSuggestionApprovedTemplate::new(
            "<script>alert('xss')</script>",
            "Normal phrase",
            None,
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_xss_prevention_in_phrase_text() {
        let template = PhraseSuggestionApprovedTemplate::new(
            "User",
            "<img src=x onerror=alert('xss')>",
            None,
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the malicious HTML
        assert!(!html.contains("<img src=x"));
        assert!(html.contains("&#60;img") || html.contains("&lt;img"));
    }
}
