/// Feed service module
///
/// Generates RSS, Atom, and JSON Feed syndication feeds from published blog posts.
/// Uses pulldown-cmark for markdown-to-HTML conversion.
use anyhow::Result;
use std::sync::Arc;

use crate::models::api::feed::{JsonFeed, JsonFeedAuthor, JsonFeedItem};
use crate::models::db::BlogPost;
use crate::repositories::traits::{BlogPostFilters, BlogRepository};

/// Site metadata for feed generation
#[derive(Debug)]
pub struct FeedConfig {
    pub site_title: String,
    pub site_description: String,
    pub site_url: String,
    pub author_name: String,
    pub language: String,
}

impl Default for FeedConfig {
    fn default() -> Self {
        Self {
            site_title: "KennWilliamson.org".to_string(),
            site_description:
                "Personal website of Kenn Williamson - A man of the margins exploring truth, beauty, and love through technology and tradition."
                    .to_string(),
            site_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "https://kennwilliamson.org".to_string()),
            author_name: "Kenn Williamson".to_string(),
            language: "en-US".to_string(),
        }
    }
}

/// FeedService provides feed generation for blog posts
///
/// Uses dependency injection pattern with Arc-wrapped trait objects
/// for testability and flexibility.
pub struct FeedService {
    repository: Arc<dyn BlogRepository>,
    config: FeedConfig,
}

impl std::fmt::Debug for FeedService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeedService")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

/// Builder for FeedService with validation
pub struct FeedServiceBuilder {
    repository: Option<Box<dyn BlogRepository>>,
    config: Option<FeedConfig>,
}

impl FeedServiceBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            repository: None,
            config: None,
        }
    }

    /// Set blog repository implementation
    pub fn with_repository(mut self, repository: Box<dyn BlogRepository>) -> Self {
        self.repository = Some(repository);
        self
    }

    /// Set feed configuration (optional - uses defaults if not provided)
    pub fn with_config(mut self, config: FeedConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build FeedService with validation
    ///
    /// # Errors
    ///
    /// Returns error if required dependency (repository) is missing
    pub fn build(self) -> Result<FeedService> {
        Ok(FeedService {
            repository: Arc::from(
                self.repository
                    .ok_or_else(|| anyhow::anyhow!("BlogRepository is required"))?,
            ),
            config: self.config.unwrap_or_default(),
        })
    }
}

impl Default for FeedServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedService {
    /// Create new builder instance
    pub fn builder() -> FeedServiceBuilder {
        FeedServiceBuilder::new()
    }

    /// Create FeedService directly (useful for testing)
    pub fn new(repository: Box<dyn BlogRepository>) -> Self {
        Self {
            repository: Arc::from(repository),
            config: FeedConfig::default(),
        }
    }

    /// Maximum number of posts to include in feeds
    const MAX_FEED_ITEMS: i32 = 50;

    /// Fetch published posts for feed generation
    async fn get_published_posts(&self) -> Result<Vec<BlogPost>> {
        let filters = BlogPostFilters {
            status: Some("published".to_string()),
            tag: None,
            page: 1,
            limit: Self::MAX_FEED_ITEMS,
        };

        let result = self.repository.list_posts(filters).await?;
        Ok(result.posts)
    }

    /// Convert markdown content to HTML
    fn markdown_to_html(markdown: &str) -> String {
        use pulldown_cmark::{html, Options, Parser};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    /// Build post URL from slug
    fn post_url(&self, slug: &str) -> String {
        format!("{}/blog/{}", self.config.site_url, slug)
    }

    /// Generate RSS 2.0 feed
    pub async fn generate_rss(&self) -> Result<String> {
        let posts = self.get_published_posts().await?;

        let mut channel = rss::ChannelBuilder::default()
            .title(&self.config.site_title)
            .link(&self.config.site_url)
            .description(&self.config.site_description)
            .language(Some(self.config.language.clone()))
            .generator(Some("KennWilliamson.org Feed Generator".to_string()))
            .build();

        let items: Vec<rss::Item> = posts
            .into_iter()
            .map(|post| {
                let mut item = rss::ItemBuilder::default()
                    .title(Some(post.title.clone()))
                    .link(Some(self.post_url(&post.slug)))
                    .description(post.excerpt.clone())
                    .content(Some(Self::markdown_to_html(&post.content)))
                    .author(Some(self.config.author_name.clone()))
                    .guid(Some(
                        rss::GuidBuilder::default()
                            .value(self.post_url(&post.slug))
                            .permalink(true)
                            .build(),
                    ))
                    .build();

                // Set publication date if available
                if let Some(published_at) = post.published_at {
                    item.set_pub_date(Some(published_at.to_rfc2822()));
                }

                // Add categories from tags
                let categories: Vec<rss::Category> = post
                    .tags
                    .iter()
                    .map(|tag| {
                        rss::CategoryBuilder::default()
                            .name(tag.clone())
                            .build()
                    })
                    .collect();
                item.set_categories(categories);

                // Add featured image as enclosure if present
                if let Some(ref image_url) = post.featured_image_url {
                    item.set_enclosure(Some(
                        rss::EnclosureBuilder::default()
                            .url(image_url.clone())
                            .mime_type("image/jpeg".to_string())
                            .length("0".to_string())
                            .build(),
                    ));
                }

                item
            })
            .collect();

        channel.set_items(items);

        Ok(channel.to_string())
    }

    /// Generate Atom feed
    pub async fn generate_atom(&self) -> Result<String> {
        use atom_syndication::{
            ContentBuilder, EntryBuilder, FeedBuilder, GeneratorBuilder, LinkBuilder,
            PersonBuilder, TextBuilder,
        };

        let posts = self.get_published_posts().await?;

        // Find the most recent update time for feed updated field
        let last_updated = posts
            .iter()
            .map(|p| p.updated_at)
            .max()
            .unwrap_or_else(chrono::Utc::now);

        let entries: Vec<atom_syndication::Entry> = posts
            .into_iter()
            .map(|post| {
                let mut entry_builder = EntryBuilder::default();

                entry_builder
                    .id(self.post_url(&post.slug))
                    .title(TextBuilder::default().value(post.title.clone()).build())
                    .updated(post.updated_at)
                    .links(vec![LinkBuilder::default()
                        .href(self.post_url(&post.slug))
                        .rel("alternate".to_string())
                        .mime_type(Some("text/html".to_string()))
                        .build()])
                    .authors(vec![PersonBuilder::default()
                        .name(self.config.author_name.clone())
                        .uri(Some(self.config.site_url.clone()))
                        .build()])
                    .content(Some(
                        ContentBuilder::default()
                            .content_type(Some("html".to_string()))
                            .value(Some(Self::markdown_to_html(&post.content)))
                            .build(),
                    ));

                // Add summary if available
                if let Some(ref excerpt) = post.excerpt {
                    entry_builder.summary(Some(
                        TextBuilder::default()
                            .value(excerpt.clone())
                            .build(),
                    ));
                }

                // Add published date if available
                if let Some(published_at) = post.published_at {
                    entry_builder.published(Some(published_at.into()));
                }

                // Add categories from tags
                let categories: Vec<atom_syndication::Category> = post
                    .tags
                    .iter()
                    .map(|tag| {
                        atom_syndication::CategoryBuilder::default()
                            .term(tag.clone())
                            .build()
                    })
                    .collect();
                entry_builder.categories(categories);

                entry_builder.build()
            })
            .collect();

        let feed = FeedBuilder::default()
            .id(&self.config.site_url)
            .title(TextBuilder::default().value(self.config.site_title.clone()).build())
            .subtitle(Some(
                TextBuilder::default()
                    .value(self.config.site_description.clone())
                    .build(),
            ))
            .updated(last_updated)
            .links(vec![
                LinkBuilder::default()
                    .href(self.config.site_url.clone())
                    .rel("alternate".to_string())
                    .mime_type(Some("text/html".to_string()))
                    .build(),
                LinkBuilder::default()
                    .href(format!("{}/feed/atom", self.config.site_url))
                    .rel("self".to_string())
                    .mime_type(Some("application/atom+xml".to_string()))
                    .build(),
            ])
            .authors(vec![PersonBuilder::default()
                .name(self.config.author_name.clone())
                .uri(Some(self.config.site_url.clone()))
                .build()])
            .generator(Some(
                GeneratorBuilder::default()
                    .value("KennWilliamson.org Feed Generator".to_string())
                    .build(),
            ))
            .lang(Some(self.config.language.clone()))
            .entries(entries)
            .build();

        Ok(feed.to_string())
    }

    /// Generate JSON Feed 1.1
    pub async fn generate_json(&self) -> Result<String> {
        let posts = self.get_published_posts().await?;

        let items: Vec<JsonFeedItem> = posts
            .into_iter()
            .map(|post| JsonFeedItem {
                id: self.post_url(&post.slug),
                url: Some(self.post_url(&post.slug)),
                title: Some(post.title),
                content_html: Some(Self::markdown_to_html(&post.content)),
                content_text: None,
                summary: post.excerpt,
                image: post.featured_image_url,
                date_published: post.published_at,
                date_modified: Some(post.updated_at),
                tags: if post.tags.is_empty() {
                    None
                } else {
                    Some(post.tags)
                },
                language: Some(self.config.language.clone()),
            })
            .collect();

        let feed = JsonFeed {
            version: JsonFeed::VERSION.to_string(),
            title: self.config.site_title.clone(),
            home_page_url: self.config.site_url.clone(),
            feed_url: format!("{}/feed/json", self.config.site_url),
            description: Some(self.config.site_description.clone()),
            icon: Some(format!("{}/favicon-large.png", self.config.site_url)),
            favicon: Some(format!("{}/favicon-small.png", self.config.site_url)),
            language: Some(self.config.language.clone()),
            authors: Some(vec![JsonFeedAuthor {
                name: Some(self.config.author_name.clone()),
                url: Some(self.config.site_url.clone()),
                avatar: None,
            }]),
            items,
        };

        serde_json::to_string_pretty(&feed).map_err(|e| anyhow::anyhow!("JSON serialization failed: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::traits::BlogPostList;
    use crate::test_utils::BlogPostBuilder;
    use mockall::predicate::*;

    // Import MockBlogRepository
    use crate::repositories::mocks::MockBlogRepository;

    fn create_test_config() -> FeedConfig {
        FeedConfig {
            site_title: "Test Blog".to_string(),
            site_description: "A test blog for unit tests".to_string(),
            site_url: "https://test.example.com".to_string(),
            author_name: "Test Author".to_string(),
            language: "en-US".to_string(),
        }
    }

    fn create_published_post(slug: &str, title: &str) -> BlogPost {
        BlogPostBuilder::new()
            .with_slug(slug)
            .with_title(title)
            .with_content("# Hello\n\nThis is **test** content.")
            .with_excerpt("This is a test excerpt")
            .with_tags(vec!["rust", "testing"])
            .published()
            .build()
    }

    #[tokio::test]
    async fn test_generate_rss_with_posts() {
        // Given: Mock repository with published posts
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_list_posts()
            .withf(|filters: &BlogPostFilters| {
                filters.status == Some("published".to_string()) && filters.page == 1
            })
            .times(1)
            .returning(|_| {
                Ok(BlogPostList {
                    posts: vec![
                        create_published_post("first-post", "First Post"),
                        create_published_post("second-post", "Second Post"),
                    ],
                    total: 2,
                    page: 1,
                    total_pages: 1,
                })
            });

        let service = FeedService::builder()
            .with_repository(Box::new(mock_repo))
            .with_config(create_test_config())
            .build()
            .expect("Failed to build FeedService");

        // When: Generate RSS feed
        let result = service.generate_rss().await;

        // Then: RSS feed is valid and contains posts
        assert!(result.is_ok());
        let rss = result.unwrap();
        assert!(rss.contains("<title>Test Blog</title>"));
        assert!(rss.contains("<title>First Post</title>"));
        assert!(rss.contains("<title>Second Post</title>"));
        assert!(rss.contains("https://test.example.com/blog/first-post"));
        assert!(rss.contains("<category>rust</category>"));
    }

    #[tokio::test]
    async fn test_generate_rss_empty_feed() {
        // Given: Mock repository with no posts
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_list_posts()
            .times(1)
            .returning(|_| {
                Ok(BlogPostList {
                    posts: vec![],
                    total: 0,
                    page: 1,
                    total_pages: 0,
                })
            });

        let service = FeedService::builder()
            .with_repository(Box::new(mock_repo))
            .with_config(create_test_config())
            .build()
            .expect("Failed to build FeedService");

        // When: Generate RSS feed
        let result = service.generate_rss().await;

        // Then: RSS feed is valid but empty
        assert!(result.is_ok());
        let rss = result.unwrap();
        assert!(rss.contains("<title>Test Blog</title>"));
        assert!(!rss.contains("<item>"));
    }

    #[tokio::test]
    async fn test_generate_atom_with_posts() {
        // Given: Mock repository with published posts
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_list_posts()
            .times(1)
            .returning(|_| {
                Ok(BlogPostList {
                    posts: vec![create_published_post("test-post", "Test Post")],
                    total: 1,
                    page: 1,
                    total_pages: 1,
                })
            });

        let service = FeedService::builder()
            .with_repository(Box::new(mock_repo))
            .with_config(create_test_config())
            .build()
            .expect("Failed to build FeedService");

        // When: Generate Atom feed
        let result = service.generate_atom().await;

        // Then: Atom feed is valid
        assert!(result.is_ok());
        let atom = result.unwrap();
        assert!(atom.contains("<title>Test Blog</title>"));
        assert!(atom.contains("<title>Test Post</title>"));
        assert!(atom.contains("https://test.example.com/blog/test-post"));
    }

    #[tokio::test]
    async fn test_generate_json_with_posts() {
        // Given: Mock repository with published posts
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_list_posts()
            .times(1)
            .returning(|_| {
                Ok(BlogPostList {
                    posts: vec![create_published_post("json-test", "JSON Test Post")],
                    total: 1,
                    page: 1,
                    total_pages: 1,
                })
            });

        let service = FeedService::builder()
            .with_repository(Box::new(mock_repo))
            .with_config(create_test_config())
            .build()
            .expect("Failed to build FeedService");

        // When: Generate JSON feed
        let result = service.generate_json().await;

        // Then: JSON feed is valid
        assert!(result.is_ok());
        let json = result.unwrap();

        // Parse and verify structure
        let feed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");
        assert_eq!(feed["version"], "https://jsonfeed.org/version/1.1");
        assert_eq!(feed["title"], "Test Blog");
        assert!(feed["items"].is_array());
        assert_eq!(feed["items"][0]["title"], "JSON Test Post");
    }

    #[tokio::test]
    async fn test_markdown_to_html_conversion() {
        // Test basic markdown conversion
        let markdown = "# Hello\n\nThis is **bold** and *italic*.";
        let html = FeedService::markdown_to_html(markdown);

        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[tokio::test]
    async fn test_markdown_to_html_with_code() {
        let markdown = "Here is `inline code` and:\n\n```rust\nfn main() {}\n```";
        let html = FeedService::markdown_to_html(markdown);

        assert!(html.contains("<code>inline code</code>"));
        assert!(html.contains("<pre>"));
    }

    #[tokio::test]
    async fn test_builder_requires_repository() {
        // Given: Builder without repository
        let result = FeedService::builder().build();

        // Then: Build fails
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("BlogRepository is required"));
    }

    #[tokio::test]
    async fn test_post_url_generation() {
        let mut mock_repo = MockBlogRepository::new();
        mock_repo.expect_list_posts().returning(|_| {
            Ok(BlogPostList {
                posts: vec![],
                total: 0,
                page: 1,
                total_pages: 0,
            })
        });

        let service = FeedService::builder()
            .with_repository(Box::new(mock_repo))
            .with_config(create_test_config())
            .build()
            .expect("Failed to build FeedService");

        assert_eq!(
            service.post_url("my-post"),
            "https://test.example.com/blog/my-post"
        );
    }
}
