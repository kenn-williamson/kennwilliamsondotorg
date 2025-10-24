use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for phrase suggestion rejection notifications
///
/// Notifies users when their phrase suggestion has been rejected
#[derive(Template)]
#[template(path = "emails/phrase_suggestion_rejected.html")]
pub struct PhraseSuggestionRejectedTemplate {
    /// Display name of the user
    pub user_display_name: String,

    /// The phrase text that was rejected
    pub phrase_text: String,

    /// Optional message from the admin explaining the rejection
    pub admin_message: Option<String>,

    /// URL to submit another phrase
    pub submit_phrase_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl PhraseSuggestionRejectedTemplate {
    /// Create a new phrase suggestion rejected email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `phrase_text` - The phrase text that was rejected
    /// * `admin_message` - Optional message from the admin
    /// * `frontend_url` - Base URL of the frontend
    pub fn new(
        user_display_name: impl Into<String>,
        phrase_text: impl Into<String>,
        admin_message: Option<String>,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let submit_phrase_url = format!("{}/phrases", frontend_base);

        Self {
            user_display_name: user_display_name.into(),
            phrase_text: phrase_text.into(),
            admin_message,
            submit_phrase_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for PhraseSuggestionRejectedTemplate {
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
            r#"Phrase Suggestion Update

Hello {},

Thank you for submitting a phrase suggestion to KennWilliamson.org.

YOUR SUGGESTION:
"{}"

After review, we're unable to include this phrase in our collection at this time.{}

We appreciate your creativity and encourage you to submit other phrases in the future. Your participation helps make our community more vibrant!

Submit another phrase here:
{}

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name, self.phrase_text, admin_section, self.submit_phrase_url
        )
    }

    fn subject(&self) -> String {
        "Phrase Suggestion Update - KennWilliamson.org".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_suggestion_rejected_renders_html() {
        let template = PhraseSuggestionRejectedTemplate::new(
            "Jane Doe",
            "Too controversial",
            Some("Does not align with our guidelines".to_string()),
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("Jane Doe"));
        assert!(html.contains("Too controversial"));
        assert!(html.contains("Does not align with our guidelines"));
        assert!(html.contains("https://kennwilliamson.org/phrases"));
    }

    #[test]
    fn test_phrase_suggestion_rejected_renders_plain_text() {
        let template = PhraseSuggestionRejectedTemplate::new(
            "John Smith",
            "Not appropriate",
            None,
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("John Smith"));
        assert!(text.contains("Not appropriate"));
        assert!(text.contains("https://kennwilliamson.org/phrases"));
    }

    #[test]
    fn test_phrase_suggestion_rejected_subject() {
        let template = PhraseSuggestionRejectedTemplate::new(
            "Test User",
            "Test phrase",
            None,
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(subject, "Phrase Suggestion Update - KennWilliamson.org");
    }

    #[test]
    fn test_xss_prevention_in_user_display_name() {
        let template = PhraseSuggestionRejectedTemplate::new(
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
        let template = PhraseSuggestionRejectedTemplate::new(
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
