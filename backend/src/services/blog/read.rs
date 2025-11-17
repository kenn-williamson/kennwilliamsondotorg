use anyhow::Result;
use uuid::Uuid;

use crate::models::db::BlogPost;
use crate::repositories::traits::{BlogPostFilters, BlogPostList, TagCount};

use super::BlogService;

/// Get blog post by ID
pub async fn get_post_by_id(service: &BlogService, id: Uuid) -> Result<Option<BlogPost>> {
    service.repository.get_post_by_id(id).await
}

/// Get blog post by slug
pub async fn get_post_by_slug(service: &BlogService, slug: &str) -> Result<Option<BlogPost>> {
    service.repository.get_post_by_slug(slug).await
}

/// List blog posts with filters and pagination
pub async fn list_posts(service: &BlogService, filters: BlogPostFilters) -> Result<BlogPostList> {
    service.repository.list_posts(filters).await
}

/// Search blog posts using full-text search
pub async fn search_posts(
    service: &BlogService,
    query: &str,
    page: i32,
    limit: i32,
) -> Result<BlogPostList> {
    service.repository.search_posts(query, page, limit).await
}

/// Get all tags with optional status filter
pub async fn get_all_tags(service: &BlogService, status: Option<String>) -> Result<Vec<TagCount>> {
    service.repository.get_all_tags(status).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockBlogRepository, MockImageStorage};
    use crate::test_utils::BlogPostBuilder;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_post_by_id_found() {
        // Given: A mock repository with a post
        let mut mock_repo = MockBlogRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_id(test_id)
                        .with_title("Test Post")
                        .build(),
                ))
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        // When: Getting post by ID
        let result = service.get_post_by_id(test_id).await;

        // Then: Post returned
        assert!(result.is_ok());
        let post = result.unwrap();
        assert!(post.is_some());
        assert_eq!(post.unwrap().id, test_id);
    }

    #[tokio::test]
    async fn test_get_post_by_id_not_found() {
        // Given: A mock repository with no post
        let mut mock_repo = MockBlogRepository::new();
        let test_id = Uuid::new_v4();

        mock_repo
            .expect_get_post_by_id()
            .with(eq(test_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        // When: Getting non-existent post
        let result = service.get_post_by_id(test_id).await;

        // Then: None returned
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_post_by_slug_found() {
        // Given: A mock repository with a post
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_get_post_by_slug()
            .with(eq("test-post"))
            .times(1)
            .returning(|_| {
                Ok(Some(
                    BlogPostBuilder::new()
                        .with_slug("test-post")
                        .with_title("Test Post")
                        .build(),
                ))
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        // When: Getting post by slug
        let result = service.get_post_by_slug("test-post").await;

        // Then: Post returned
        assert!(result.is_ok());
        let post = result.unwrap();
        assert!(post.is_some());
        assert_eq!(post.unwrap().slug, "test-post");
    }

    #[tokio::test]
    async fn test_list_posts() {
        // Given: A mock repository with posts
        let mut mock_repo = MockBlogRepository::new();

        mock_repo.expect_list_posts().times(1).returning(|_| {
            Ok(BlogPostList {
                posts: vec![
                    BlogPostBuilder::new().with_title("Post 1").build(),
                    BlogPostBuilder::new().with_title("Post 2").build(),
                ],
                total: 2,
                page: 1,
                total_pages: 1,
            })
        });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        // When: Listing posts
        let filters = BlogPostFilters {
            status: Some("published".to_string()),
            tag: None,
            page: 1,
            limit: 10,
        };
        let result = service.list_posts(filters).await;

        // Then: Posts returned
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.posts.len(), 2);
        assert_eq!(list.total, 2);
    }

    #[tokio::test]
    async fn test_search_posts() {
        // Given: A mock repository with search results
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_search_posts()
            .with(eq("rust"), eq(1), eq(10))
            .times(1)
            .returning(|_, _, _| {
                Ok(BlogPostList {
                    posts: vec![
                        BlogPostBuilder::new()
                            .with_title("Rust Programming")
                            .with_tags(vec!["rust"])
                            .build(),
                    ],
                    total: 1,
                    page: 1,
                    total_pages: 1,
                })
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        // When: Searching posts
        let result = service.search_posts("rust", 1, 10).await;

        // Then: Search results returned
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.posts.len(), 1);
        assert_eq!(list.posts[0].title, "Rust Programming");
    }

    #[tokio::test]
    async fn test_get_all_tags() {
        // Given: A mock repository with tags
        let mut mock_repo = MockBlogRepository::new();

        mock_repo
            .expect_get_all_tags()
            .with(eq(Some("published".to_string())))
            .times(1)
            .returning(|_| {
                Ok(vec![
                    TagCount {
                        tag: "rust".to_string(),
                        count: 5,
                    },
                    TagCount {
                        tag: "web".to_string(),
                        count: 3,
                    },
                ])
            });

        let service = BlogService::new(Box::new(mock_repo), Box::new(MockImageStorage::new()));

        // When: Getting tags
        let result = service.get_all_tags(Some("published".to_string())).await;

        // Then: Tags returned
        assert!(result.is_ok());
        let tags = result.unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].tag, "rust");
        assert_eq!(tags[0].count, 5);
    }
}
