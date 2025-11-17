use backend::repositories::postgres::postgres_blog_repository::PostgresBlogRepository;
use backend::repositories::traits::blog_repository::{
    BlogPostFilters, BlogRepository, CreateBlogPost, UpdateBlogPost,
};
use backend::test_utils::BlogPostBuilder;
use chrono::Utc;
use uuid::Uuid;

// ============================================================================
// TEST 1: Create Draft Post
// ============================================================================

#[tokio::test]
async fn test_create_post() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    let post_data = CreateBlogPost {
        slug: "test-post".to_string(),
        title: "Test Post".to_string(),
        excerpt: Some("This is a test excerpt".to_string()),
        content: "# Hello World\n\nThis is test content.".to_string(),
        featured_image_url: None,
        featured_image_alt: None,
        status: "draft".to_string(),
        tags: vec!["rust".to_string(), "testing".to_string()],
        published_at: None,
        meta_description: Some("Test meta description".to_string()),
    };

    let post = repo.create_post(post_data).await.unwrap();

    assert_eq!(post.title, "Test Post");
    assert_eq!(post.slug, "test-post");
    assert_eq!(post.status, "draft");
    assert!(post.published_at.is_none()); // Draft has no published_at
    assert_eq!(post.tags.len(), 2);
    assert!(post.tags.contains(&"rust".to_string()));
    assert_eq!(post.excerpt, Some("This is a test excerpt".to_string()));
}

// ============================================================================
// TEST 2: Create Published Post
// ============================================================================

#[tokio::test]
async fn test_create_post_published() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    let now = Utc::now();
    let post_data = CreateBlogPost {
        slug: "published-post".to_string(),
        title: "Published Post".to_string(),
        excerpt: None,
        content: "# Published Content".to_string(),
        featured_image_url: Some("https://example.com/image.jpg".to_string()),
        featured_image_alt: Some("Featured image".to_string()),
        status: "published".to_string(),
        tags: vec!["announcement".to_string()],
        published_at: Some(now),
        meta_description: None,
    };

    let post = repo.create_post(post_data).await.unwrap();

    assert_eq!(post.status, "published");
    assert!(post.published_at.is_some());
    assert_eq!(
        post.featured_image_url,
        Some("https://example.com/image.jpg".to_string())
    );
}

// ============================================================================
// TEST 3: Get Post by ID
// ============================================================================

#[tokio::test]
async fn test_get_post_by_id() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create a post using builder
    let created_post = BlogPostBuilder::new()
        .with_title("Find Me By ID")
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Find it by ID
    let found = repo.get_post_by_id(created_post.id).await.unwrap();

    assert!(found.is_some());
    let found_post = found.unwrap();
    assert_eq!(found_post.id, created_post.id);
    assert_eq!(found_post.title, "Find Me By ID");
}

// ============================================================================
// TEST 4: Get Post by ID - Not Found
// ============================================================================

#[tokio::test]
async fn test_get_post_by_id_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    let non_existent_id = Uuid::new_v4();
    let found = repo.get_post_by_id(non_existent_id).await.unwrap();

    assert!(found.is_none());
}

// ============================================================================
// TEST 5: Get Post by Slug
// ============================================================================

#[tokio::test]
async fn test_get_post_by_slug() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create a post with specific slug
    let _created_post = BlogPostBuilder::new()
        .with_title("Find Me By Slug")
        .with_slug("find-me-by-slug")
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Find it by slug
    let found = repo.get_post_by_slug("find-me-by-slug").await.unwrap();

    assert!(found.is_some());
    let found_post = found.unwrap();
    assert_eq!(found_post.slug, "find-me-by-slug");
    assert_eq!(found_post.title, "Find Me By Slug");
}

// ============================================================================
// TEST 6: Get Post by Slug - Not Found
// ============================================================================

#[tokio::test]
async fn test_get_post_by_slug_not_found() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    let found = repo.get_post_by_slug("non-existent-slug").await.unwrap();

    assert!(found.is_none());
}

// ============================================================================
// TEST 7: List Posts with Pagination
// ============================================================================

#[tokio::test]
async fn test_list_posts_pagination() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create 5 posts
    for i in 1..=5 {
        BlogPostBuilder::new()
            .with_title(format!("Post {}", i))
            .with_slug(format!("post-{}", i))
            .persist(&test_container.pool)
            .await
            .unwrap();
    }

    // Get first page (limit 2)
    let filters = BlogPostFilters {
        status: None,
        tag: None,
        page: 1,
        limit: 2,
    };
    let result = repo.list_posts(filters).await.unwrap();

    assert_eq!(result.posts.len(), 2);
    assert_eq!(result.total, 5);
    assert_eq!(result.page, 1);
    assert_eq!(result.total_pages, 3); // 5 posts / 2 per page = 3 pages

    // Get second page
    let filters = BlogPostFilters {
        status: None,
        tag: None,
        page: 2,
        limit: 2,
    };
    let result = repo.list_posts(filters).await.unwrap();

    assert_eq!(result.posts.len(), 2);
    assert_eq!(result.total, 5);
    assert_eq!(result.page, 2);
}

// ============================================================================
// TEST 8: List Posts Filtered by Status
// ============================================================================

#[tokio::test]
async fn test_list_posts_filter_by_status() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create 3 published posts
    for i in 1..=3 {
        BlogPostBuilder::new()
            .with_title(format!("Published Post {}", i))
            .with_slug(format!("published-{}", i))
            .published()
            .persist(&test_container.pool)
            .await
            .unwrap();
    }

    // Create 2 draft posts
    for i in 1..=2 {
        BlogPostBuilder::new()
            .with_title(format!("Draft Post {}", i))
            .with_slug(format!("draft-{}", i))
            .draft()
            .persist(&test_container.pool)
            .await
            .unwrap();
    }

    // Filter by published
    let filters = BlogPostFilters {
        status: Some("published".to_string()),
        tag: None,
        page: 1,
        limit: 10,
    };
    let result = repo.list_posts(filters).await.unwrap();

    assert_eq!(result.total, 3);
    assert!(result.posts.iter().all(|p| p.status == "published"));

    // Filter by draft
    let filters = BlogPostFilters {
        status: Some("draft".to_string()),
        tag: None,
        page: 1,
        limit: 10,
    };
    let result = repo.list_posts(filters).await.unwrap();

    assert_eq!(result.total, 2);
    assert!(result.posts.iter().all(|p| p.status == "draft"));
}

// ============================================================================
// TEST 9: List Posts Filtered by Tag
// ============================================================================

#[tokio::test]
async fn test_list_posts_filter_by_tag() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create posts with different tags
    BlogPostBuilder::new()
        .with_title("Rust Post")
        .with_slug("rust-post")
        .with_tags(vec!["rust", "programming"])
        .persist(&test_container.pool)
        .await
        .unwrap();

    BlogPostBuilder::new()
        .with_title("JavaScript Post")
        .with_slug("js-post")
        .with_tags(vec!["javascript", "programming"])
        .persist(&test_container.pool)
        .await
        .unwrap();

    BlogPostBuilder::new()
        .with_title("Python Post")
        .with_slug("python-post")
        .with_tags(vec!["python"])
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Filter by "rust" tag
    let filters = BlogPostFilters {
        status: None,
        tag: Some("rust".to_string()),
        page: 1,
        limit: 10,
    };
    let result = repo.list_posts(filters).await.unwrap();

    assert_eq!(result.total, 1);
    assert_eq!(result.posts[0].title, "Rust Post");

    // Filter by "programming" tag (should get 2 posts)
    let filters = BlogPostFilters {
        status: None,
        tag: Some("programming".to_string()),
        page: 1,
        limit: 10,
    };
    let result = repo.list_posts(filters).await.unwrap();

    assert_eq!(result.total, 2);
}

// ============================================================================
// TEST 10: Update Post
// ============================================================================

#[tokio::test]
async fn test_update_post() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create a post
    let created_post = BlogPostBuilder::new()
        .with_title("Original Title")
        .with_content("Original content")
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Update it
    let update_data = UpdateBlogPost {
        slug: None,
        title: Some("Updated Title".to_string()),
        excerpt: None,
        content: Some("Updated content".to_string()),
        featured_image_url: None,
        featured_image_alt: None,
        status: None,
        tags: Some(vec!["updated".to_string()]),
        published_at: None,
        meta_description: None,
    };

    let updated_post = repo
        .update_post(created_post.id, update_data)
        .await
        .unwrap();

    assert_eq!(updated_post.title, "Updated Title");
    assert_eq!(updated_post.content, "Updated content");
    assert_eq!(updated_post.tags, vec!["updated".to_string()]);
    assert!(updated_post.updated_at > created_post.updated_at);
}

// ============================================================================
// TEST 11: Delete Post
// ============================================================================

#[tokio::test]
async fn test_delete_post() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create a post
    let created_post = BlogPostBuilder::new()
        .with_title("To Be Deleted")
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Delete it
    repo.delete_post(created_post.id).await.unwrap();

    // Verify it's gone
    let found = repo.get_post_by_id(created_post.id).await.unwrap();
    assert!(found.is_none());
}

// ============================================================================
// TEST 12: Search Posts (Full-Text Search)
// ============================================================================

#[tokio::test]
async fn test_search_posts_full_text() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create posts with searchable content
    BlogPostBuilder::new()
        .with_title("Rust Programming Guide")
        .with_content("Learn Rust programming language basics")
        .persist(&test_container.pool)
        .await
        .unwrap();

    BlogPostBuilder::new()
        .with_title("JavaScript Tutorial")
        .with_content("Introduction to JavaScript programming")
        .persist(&test_container.pool)
        .await
        .unwrap();

    BlogPostBuilder::new()
        .with_title("Python Basics")
        .with_content("Getting started with Python")
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Search for "rust"
    let result = repo.search_posts("rust", 1, 10).await.unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.posts[0].title, "Rust Programming Guide");

    // Search for "programming" (should match multiple)
    let result = repo.search_posts("programming", 1, 10).await.unwrap();
    assert_eq!(result.total, 2);
}

// ============================================================================
// TEST 13: Get All Tags with Counts
// ============================================================================

#[tokio::test]
async fn test_get_all_tags_with_counts() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create posts with various tags
    BlogPostBuilder::new()
        .with_tags(vec!["rust", "programming"])
        .published()
        .persist(&test_container.pool)
        .await
        .unwrap();

    BlogPostBuilder::new()
        .with_tags(vec!["rust", "web"])
        .published()
        .persist(&test_container.pool)
        .await
        .unwrap();

    BlogPostBuilder::new()
        .with_tags(vec!["javascript", "programming"])
        .draft()
        .persist(&test_container.pool)
        .await
        .unwrap();

    // Get all tags (no filter)
    let tags = repo.get_all_tags(None).await.unwrap();

    // Should have 4 unique tags
    assert_eq!(tags.len(), 4);

    // Find rust tag - should appear twice
    let rust_tag = tags.iter().find(|t| t.tag == "rust").unwrap();
    assert_eq!(rust_tag.count, 2);

    // Find programming tag - should appear twice
    let prog_tag = tags.iter().find(|t| t.tag == "programming").unwrap();
    assert_eq!(prog_tag.count, 2);

    // Get only published tags
    let published_tags = repo
        .get_all_tags(Some("published".to_string()))
        .await
        .unwrap();

    // Should have 3 tags (rust, programming, web)
    assert_eq!(published_tags.len(), 3);

    // javascript should not be in published tags (it's draft)
    assert!(published_tags.iter().all(|t| t.tag != "javascript"));
}

// ============================================================================
// TEST 14: Slug Uniqueness Constraint
// ============================================================================

#[tokio::test]
async fn test_slug_uniqueness_constraint() {
    let test_container = crate::fixtures::TestContainer::builder()
        .build()
        .await
        .expect("Failed to create test container");
    let repo = PostgresBlogRepository::new(test_container.pool.clone());

    // Create first post with slug
    let post_data = CreateBlogPost {
        slug: "unique-slug".to_string(),
        title: "First Post".to_string(),
        excerpt: None,
        content: "Content".to_string(),
        featured_image_url: None,
        featured_image_alt: None,
        status: "draft".to_string(),
        tags: vec![],
        published_at: None,
        meta_description: None,
    };

    repo.create_post(post_data).await.unwrap();

    // Try to create second post with same slug
    let duplicate_post = CreateBlogPost {
        slug: "unique-slug".to_string(), // Same slug!
        title: "Second Post".to_string(),
        excerpt: None,
        content: "Different content".to_string(),
        featured_image_url: None,
        featured_image_alt: None,
        status: "draft".to_string(),
        tags: vec![],
        published_at: None,
        meta_description: None,
    };

    let result = repo.create_post(duplicate_post).await;

    // Should error due to unique constraint
    assert!(result.is_err());
}
