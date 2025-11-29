use crate::events::DomainEvent;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::any::Any;
use uuid::Uuid;

/// Event emitted when a blog post is published
///
/// This event triggers email notifications to users who have opted in to blog notifications.
/// Event handlers fetch user preferences as needed.
#[derive(Clone, Debug, Serialize)]
pub struct BlogPostPublishedEvent {
    /// ID of the published blog post
    pub post_id: Uuid,

    /// URL-friendly slug of the post
    pub slug: String,

    /// Title of the blog post
    pub title: String,

    /// Optional excerpt of the blog post
    pub excerpt: Option<String>,

    /// Optional featured image URL
    pub featured_image_url: Option<String>,

    /// When this event occurred
    pub occurred_at: DateTime<Utc>,

    /// Optional correlation ID for tracing
    pub correlation_id: Option<String>,
}

impl BlogPostPublishedEvent {
    /// Create a new BlogPostPublishedEvent
    ///
    /// # Arguments
    /// * `post_id` - ID of the published post
    /// * `slug` - URL-friendly slug of the post
    /// * `title` - Title of the blog post
    /// * `excerpt` - Optional excerpt of the post
    /// * `featured_image_url` - Optional featured image URL
    pub fn new(
        post_id: Uuid,
        slug: impl Into<String>,
        title: impl Into<String>,
        excerpt: Option<String>,
        featured_image_url: Option<String>,
    ) -> Self {
        Self {
            post_id,
            slug: slug.into(),
            title: title.into(),
            excerpt,
            featured_image_url,
            occurred_at: Utc::now(),
            correlation_id: None,
        }
    }

    /// Create a new event with correlation ID
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

impl DomainEvent for BlogPostPublishedEvent {
    fn event_type(&self) -> &'static str {
        "blog_post.published"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_boxed(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_post_published_event_creation() {
        let post_id = Uuid::new_v4();
        let event = BlogPostPublishedEvent::new(
            post_id,
            "my-first-post",
            "My First Post",
            Some("This is the excerpt".to_string()),
            Some("https://example.com/image.jpg".to_string()),
        );

        assert_eq!(event.post_id, post_id);
        assert_eq!(event.slug, "my-first-post");
        assert_eq!(event.title, "My First Post");
        assert_eq!(event.excerpt, Some("This is the excerpt".to_string()));
        assert_eq!(
            event.featured_image_url,
            Some("https://example.com/image.jpg".to_string())
        );
        assert_eq!(event.event_type(), "blog_post.published");
        assert!(event.correlation_id.is_none());
    }

    #[test]
    fn test_blog_post_published_event_without_excerpt() {
        let post_id = Uuid::new_v4();
        let event =
            BlogPostPublishedEvent::new(post_id, "no-excerpt-post", "No Excerpt Post", None, None);

        assert_eq!(event.post_id, post_id);
        assert_eq!(event.excerpt, None);
        assert_eq!(event.featured_image_url, None);
    }

    #[test]
    fn test_with_correlation_id() {
        let event =
            BlogPostPublishedEvent::new(Uuid::new_v4(), "test-post", "Test Post", None, None)
                .with_correlation_id("test-correlation-id");

        assert_eq!(event.correlation_id(), Some("test-correlation-id"));
    }

    #[test]
    fn test_event_is_cloneable() {
        let event = BlogPostPublishedEvent::new(
            Uuid::new_v4(),
            "test-post",
            "Test Post",
            Some("Excerpt".to_string()),
            None,
        );

        let cloned = event.clone();
        assert_eq!(event.post_id, cloned.post_id);
        assert_eq!(event.slug, cloned.slug);
        assert_eq!(event.title, cloned.title);
        assert_eq!(event.excerpt, cloned.excerpt);
    }

    #[test]
    fn test_event_is_serializable() {
        let event = BlogPostPublishedEvent::new(
            Uuid::new_v4(),
            "serialization-test",
            "Serialization Test",
            Some("Testing serialization".to_string()),
            None,
        );

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("serialization-test"));
        assert!(json.contains("Serialization Test"));
        assert!(json.contains("Testing serialization"));
        assert!(!json.contains("blog_post.published")); // event_type is not in the serialized struct
    }

    #[test]
    fn test_clone_boxed() {
        let event = BlogPostPublishedEvent::new(Uuid::new_v4(), "test", "Test", None, None);

        let boxed = event.clone_boxed();
        assert_eq!(boxed.event_type(), "blog_post.published");
    }
}
