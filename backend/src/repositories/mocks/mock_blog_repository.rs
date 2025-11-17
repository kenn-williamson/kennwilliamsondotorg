use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::BlogPost;
use crate::repositories::traits::blog_repository::{
    BlogPostFilters, BlogPostList, BlogRepository, CreateBlogPost, TagCount, UpdateBlogPost,
};

// Generate mock for BlogRepository trait
mock! {
    pub BlogRepository {}

    #[async_trait]
    impl BlogRepository for BlogRepository {
        async fn create_post(&self, post: CreateBlogPost) -> Result<BlogPost>;
        async fn get_post_by_id(&self, id: Uuid) -> Result<Option<BlogPost>>;
        async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>>;
        async fn list_posts(&self, filters: BlogPostFilters) -> Result<BlogPostList>;
        async fn update_post(&self, id: Uuid, post: UpdateBlogPost) -> Result<BlogPost>;
        async fn delete_post(&self, id: Uuid) -> Result<()>;
        async fn search_posts(&self, query: &str, page: i32, limit: i32) -> Result<BlogPostList>;
        async fn get_all_tags(&self, status: Option<String>) -> Result<Vec<TagCount>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    // Helper function to create a test blog post
    fn create_test_post() -> BlogPost {
        crate::test_utils::BlogPostBuilder::new()
            .with_title("Test Post")
            .with_slug("test-post")
            .build()
    }

    #[tokio::test]
    async fn test_mock_create_post() {
        let mut mock_repo = MockBlogRepository::new();

        let create_data = CreateBlogPost {
            slug: "test-post".to_string(),
            title: "Test Post".to_string(),
            excerpt: None,
            content: "Content".to_string(),
            featured_image_url: None,
            featured_image_alt: None,
            status: "draft".to_string(),
            tags: vec![],
            published_at: None,
            meta_description: None,
        };

        // Setup mock expectation
        mock_repo
            .expect_create_post()
            .times(1)
            .returning(|_| Ok(create_test_post()));

        // Test the mock
        let result = mock_repo.create_post(create_data).await;
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.title, "Test Post");
    }

    #[tokio::test]
    async fn test_mock_get_post_by_id() {
        let mut mock_repo = MockBlogRepository::new();
        let post_id = Uuid::new_v4();

        // Setup mock expectation
        mock_repo
            .expect_get_post_by_id()
            .times(1)
            .with(eq(post_id))
            .returning(|_| Ok(Some(create_test_post())));

        // Test the mock
        let result = mock_repo.get_post_by_id(post_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_mock_get_post_by_slug() {
        let mut mock_repo = MockBlogRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_get_post_by_slug()
            .times(1)
            .with(eq("test-post"))
            .returning(|_| Ok(Some(create_test_post())));

        // Test the mock
        let result = mock_repo.get_post_by_slug("test-post").await;
        assert!(result.is_ok());
        let post = result.unwrap();
        assert!(post.is_some());
        assert_eq!(post.unwrap().slug, "test-post");
    }

    #[tokio::test]
    async fn test_mock_list_posts() {
        let mut mock_repo = MockBlogRepository::new();

        let filters = BlogPostFilters {
            status: Some("published".to_string()),
            tag: None,
            page: 1,
            limit: 10,
        };

        // Setup mock expectation
        mock_repo.expect_list_posts().times(1).returning(|_| {
            Ok(BlogPostList {
                posts: vec![create_test_post()],
                total: 1,
                page: 1,
                total_pages: 1,
            })
        });

        // Test the mock
        let result = mock_repo.list_posts(filters).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.posts.len(), 1);
        assert_eq!(list.total, 1);
    }

    #[tokio::test]
    async fn test_mock_delete_post() {
        let mut mock_repo = MockBlogRepository::new();
        let post_id = Uuid::new_v4();

        // Setup mock expectation
        mock_repo
            .expect_delete_post()
            .times(1)
            .with(eq(post_id))
            .returning(|_| Ok(()));

        // Test the mock
        let result = mock_repo.delete_post(post_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_search_posts() {
        let mut mock_repo = MockBlogRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_search_posts()
            .times(1)
            .with(eq("rust"), eq(1), eq(10))
            .returning(|_, _, _| {
                Ok(BlogPostList {
                    posts: vec![create_test_post()],
                    total: 1,
                    page: 1,
                    total_pages: 1,
                })
            });

        // Test the mock
        let result = mock_repo.search_posts("rust", 1, 10).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.posts.len(), 1);
    }

    #[tokio::test]
    async fn test_mock_get_all_tags() {
        let mut mock_repo = MockBlogRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_get_all_tags()
            .times(1)
            .with(eq(Some("published".to_string())))
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

        // Test the mock
        let result = mock_repo.get_all_tags(Some("published".to_string())).await;
        assert!(result.is_ok());
        let tags = result.unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].tag, "rust");
        assert_eq!(tags[0].count, 5);
    }

    #[tokio::test]
    async fn test_mock_error_handling() {
        let mut mock_repo = MockBlogRepository::new();

        // Setup mock to return an error
        mock_repo
            .expect_get_post_by_id()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database connection failed")));

        // Test error handling
        let result = mock_repo.get_post_by_id(Uuid::new_v4()).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Database connection failed")
        );
    }
}
