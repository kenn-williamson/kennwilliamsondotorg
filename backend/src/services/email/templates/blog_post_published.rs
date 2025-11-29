use super::EmailTemplate;
use anyhow::Result;
use askama::Template;

/// Email template for blog post notification emails
///
/// Notifies users when a new blog post is published
#[derive(Template)]
#[template(path = "emails/blog_post_published.html")]
pub struct BlogPostPublishedTemplate {
    /// Display name of the user
    pub user_display_name: String,

    /// Title of the blog post
    pub title: String,

    /// Optional excerpt of the blog post
    pub excerpt: Option<String>,

    /// Optional featured image URL
    pub featured_image_url: Option<String>,

    /// Full URL to the blog post
    pub blog_post_url: String,

    /// URL to unsubscribe from blog notifications
    pub unsubscribe_url: String,

    /// Base URL of the frontend (for dynamic logo and other header content)
    pub frontend_url: String,
}

impl BlogPostPublishedTemplate {
    /// Create a new blog post published email template
    ///
    /// # Arguments
    /// * `user_display_name` - Display name of the user
    /// * `title` - Title of the blog post
    /// * `slug` - URL slug of the blog post
    /// * `excerpt` - Optional excerpt of the post
    /// * `featured_image_url` - Optional featured image URL
    /// * `unsubscribe_token` - Token for one-click unsubscribe
    /// * `frontend_url` - Base URL of the frontend
    pub fn new(
        user_display_name: impl Into<String>,
        title: impl Into<String>,
        slug: &str,
        excerpt: Option<String>,
        featured_image_url: Option<String>,
        unsubscribe_token: &str,
        frontend_url: &str,
    ) -> Self {
        let frontend_base = frontend_url.trim_end_matches('/');
        let blog_post_url = format!("{}/blog/{}", frontend_base, slug);
        let unsubscribe_url = format!("{}/unsubscribe/{}", frontend_base, unsubscribe_token);

        Self {
            user_display_name: user_display_name.into(),
            title: title.into(),
            excerpt,
            featured_image_url,
            blog_post_url,
            unsubscribe_url,
            frontend_url: frontend_url.into(),
        }
    }
}

impl EmailTemplate for BlogPostPublishedTemplate {
    fn render_html(&self) -> Result<String> {
        Ok(self.render()?)
    }

    fn render_plain_text(&self) -> String {
        let excerpt_section = if let Some(text) = &self.excerpt {
            format!("\n\n{}\n", text)
        } else {
            String::new()
        };

        format!(
            r#"New Blog Post Published

Hi {}! A new blog post has been published on KennWilliamson.org.

{}{}

Read the full post here:
{}

---
You received this email because you're subscribed to blog notifications.
To unsubscribe, visit: {}

---
KennWilliamson.org
Building the Future with Timeless Craft
"#,
            self.user_display_name,
            self.title,
            excerpt_section,
            self.blog_post_url,
            self.unsubscribe_url
        )
    }

    fn subject(&self) -> String {
        format!("New Blog Post: {} - KennWilliamson.org", self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_post_published_renders_html() {
        let template = BlogPostPublishedTemplate::new(
            "Jane Doe",
            "My First Post",
            "my-first-post",
            Some("This is the excerpt of my first post.".to_string()),
            Some("https://example.com/image.jpg".to_string()),
            "abc123token",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("Jane Doe"));
        assert!(html.contains("My First Post"));
        assert!(html.contains("This is the excerpt of my first post."));
        assert!(html.contains("https://kennwilliamson.org/blog/my-first-post"));
        assert!(html.contains("https://kennwilliamson.org/unsubscribe/abc123token"));
        assert!(html.contains("https://example.com/image.jpg"));
    }

    #[test]
    fn test_blog_post_published_without_excerpt() {
        let template = BlogPostPublishedTemplate::new(
            "John Smith",
            "Post Without Excerpt",
            "no-excerpt",
            None,
            None,
            "token456",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        assert!(html.contains("John Smith"));
        assert!(html.contains("Post Without Excerpt"));
        assert!(html.contains("https://kennwilliamson.org/blog/no-excerpt"));
    }

    #[test]
    fn test_blog_post_published_renders_plain_text() {
        let template = BlogPostPublishedTemplate::new(
            "Test User",
            "Test Post Title",
            "test-post",
            Some("Test excerpt content".to_string()),
            None,
            "testtoken",
            "https://kennwilliamson.org",
        );

        let text = template.render_plain_text();

        assert!(text.contains("Test User"));
        assert!(text.contains("Test Post Title"));
        assert!(text.contains("Test excerpt content"));
        assert!(text.contains("https://kennwilliamson.org/blog/test-post"));
        assert!(text.contains("https://kennwilliamson.org/unsubscribe/testtoken"));
    }

    #[test]
    fn test_blog_post_published_subject() {
        let template = BlogPostPublishedTemplate::new(
            "Test User",
            "Awesome Article",
            "awesome-article",
            None,
            None,
            "token",
            "https://kennwilliamson.org",
        );

        let subject = template.subject();

        assert_eq!(
            subject,
            "New Blog Post: Awesome Article - KennWilliamson.org"
        );
    }

    #[test]
    fn test_xss_prevention_in_user_display_name() {
        let template = BlogPostPublishedTemplate::new(
            "<script>alert('xss')</script>",
            "Normal Post",
            "normal-post",
            None,
            None,
            "token",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the script tags
        assert!(!html.contains("<script>"));
        assert!(html.contains("&#60;script&#62;") || html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_xss_prevention_in_title() {
        let template = BlogPostPublishedTemplate::new(
            "User",
            "<img src=x onerror=alert('xss')>",
            "test",
            None,
            None,
            "token",
            "https://kennwilliamson.org",
        );

        let html = template.render_html().expect("Failed to render HTML");

        // Askama should escape the malicious HTML
        assert!(!html.contains("<img src=x"));
        assert!(html.contains("&#60;img") || html.contains("&lt;img"));
    }

    #[test]
    fn test_trailing_slash_handling() {
        let template = BlogPostPublishedTemplate::new(
            "User",
            "Title",
            "slug",
            None,
            None,
            "token",
            "https://kennwilliamson.org/",
        );

        // Should not have double slashes
        assert!(template.blog_post_url.contains("/blog/slug"));
        assert!(!template.blog_post_url.contains("//blog"));
    }
}
