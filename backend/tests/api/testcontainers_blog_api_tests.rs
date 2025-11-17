use crate::fixtures::TestContext;
use backend::test_utils::BlogPostBuilder;
use serde_json::json;

// ============================================================================
// PUBLIC ENDPOINT TESTS (No auth required)
// ============================================================================

#[actix_web::test]
async fn test_get_published_posts_public() {
    let ctx = TestContext::builder().build().await;

    // Create a published post directly in database
    let _post = BlogPostBuilder::new()
        .with_title("Public Test Post")
        .with_slug("public-test-post")
        .with_status("published")
        .published_at(chrono::Utc::now())
        .persist(&ctx.pool)
        .await
        .expect("Failed to create test post");

    // Get published posts without auth
    let mut resp = ctx
        .server
        .get("/backend/public/blog/posts?status=published")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("posts").is_some());
    assert!(body["posts"].as_array().unwrap().len() > 0);
}

#[actix_web::test]
async fn test_get_post_by_slug_public_success() {
    let ctx = TestContext::builder().build().await;

    // Create a published post
    let post = BlogPostBuilder::new()
        .with_title("Test Post Title")
        .with_slug("test-post-slug")
        .with_content("# Test Content")
        .with_status("published")
        .published_at(chrono::Utc::now())
        .persist(&ctx.pool)
        .await
        .expect("Failed to create test post");

    // Get post by slug without auth
    let mut resp = ctx
        .server
        .get(&format!("/backend/public/blog/posts/{}", post.slug))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["title"], "Test Post Title");
    assert_eq!(body["slug"], "test-post-slug");
    assert_eq!(body["content"], "# Test Content");
}

#[actix_web::test]
async fn test_get_post_by_slug_not_found() {
    let ctx = TestContext::builder().build().await;

    // Try to get non-existent post
    let resp = ctx
        .server
        .get("/backend/public/blog/posts/non-existent-slug")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_tags_public_success() {
    let ctx = TestContext::builder().build().await;

    // Create posts with tags
    let _post1 = BlogPostBuilder::new()
        .with_title("Rust Post")
        .with_slug("rust-post")
        .with_tags(vec!["rust".to_string(), "programming".to_string()])
        .with_status("published")
        .published_at(chrono::Utc::now())
        .persist(&ctx.pool)
        .await
        .expect("Failed to create test post");

    // Get tags without auth
    let mut resp = ctx
        .server
        .get("/backend/public/blog/tags")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("tags").is_some());
}

#[actix_web::test]
async fn test_search_posts_public() {
    let ctx = TestContext::builder().build().await;

    // Create a published post with searchable content
    let _post = BlogPostBuilder::new()
        .with_title("Searchable Post Title")
        .with_slug("searchable-post")
        .with_content("This post contains searchable content about Rust programming")
        .with_status("published")
        .published_at(chrono::Utc::now())
        .persist(&ctx.pool)
        .await
        .expect("Failed to create test post");

    // Search without auth
    let mut resp = ctx
        .server
        .get("/backend/public/blog/search?q=searchable")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("posts").is_some());
}

// ============================================================================
// AUTHENTICATION AND AUTHORIZATION TESTS
// ============================================================================

#[actix_web::test]
async fn test_create_post_requires_auth() {
    let ctx = TestContext::builder().build().await;

    let post_request = json!({
        "title": "Test Post",
        "content": "# Test Content",
        "status": "draft"
    });

    // Try to create post without auth
    let resp = ctx
        .server
        .post("/backend/protected/admin/blog/posts")
        .send_json(&post_request)
        .await
        .unwrap();

    assert_eq!(resp.status(), 401); // Unauthorized
}

#[actix_web::test]
async fn test_create_post_requires_admin() {
    let ctx = TestContext::builder().build().await;

    // Create a regular verified user (not admin)
    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    let register_request = json!({
        "email": email,
        "password": password,
        "display_name": "Regular User"
    });

    let mut register_resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_request)
        .await
        .unwrap();

    assert!(register_resp.status().is_success());
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_body["user"]["id"].as_str().unwrap();

    // Assign email-verified role (but not admin)
    crate::fixtures::assign_email_verified_role(&ctx.pool, user_id).await;

    // Login to get token
    let mut login_resp = ctx
        .server
        .post("/backend/public/auth/login")
        .send_json(&json!({"email": email, "password": password}))
        .await
        .unwrap();

    assert!(login_resp.status().is_success());
    let login_body: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_body.get("token").unwrap().as_str().unwrap();

    // Try to create post with regular user token
    let post_request = json!({
        "title": "Test Post",
        "content": "# Test Content",
        "status": "draft"
    });

    let resp = ctx
        .server
        .post("/backend/protected/admin/blog/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&post_request)
        .await
        .unwrap();

    assert_eq!(resp.status(), 403); // Forbidden
}

// ============================================================================
// ADMIN CRUD OPERATIONS TESTS
// ============================================================================

#[actix_web::test]
async fn test_create_post_success() {
    let ctx = TestContext::builder().build().await;

    // Create admin user
    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    let register_request = json!({
        "email": email,
        "password": password,
        "display_name": "Admin User"
    });

    let mut register_resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_request)
        .await
        .unwrap();

    assert!(register_resp.status().is_success());
    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_body["user"]["id"].as_str().unwrap();

    // Assign admin role
    crate::fixtures::assign_admin_role(&ctx.pool, uuid::Uuid::parse_str(user_id).unwrap()).await;

    // Login to get admin token
    let mut login_resp = ctx
        .server
        .post("/backend/public/auth/login")
        .send_json(&json!({"email": email, "password": password}))
        .await
        .unwrap();

    assert!(login_resp.status().is_success());
    let login_body: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_body.get("token").unwrap().as_str().unwrap();

    // Create post with admin token
    let post_request = json!({
        "title": "Admin Created Post",
        "content": "# Admin Content",
        "status": "draft",
        "tags": ["rust", "testing"]
    });

    let mut resp = ctx
        .server
        .post("/backend/protected/admin/blog/posts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&post_request)
        .await
        .unwrap();

    println!("Create post response status: {}", resp.status());
    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("Create post error response: {:?}", body);
    }
    assert_eq!(resp.status(), 201); // Created

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["title"], "Admin Created Post");
    assert_eq!(body["status"], "draft");
    assert!(body.get("id").is_some());
    assert!(body.get("slug").is_some());
}

#[actix_web::test]
async fn test_update_post_success() {
    let ctx = TestContext::builder().build().await;

    // Create admin user and get token
    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    let register_request = json!({
        "email": email,
        "password": password,
        "display_name": "Admin User"
    });

    let mut register_resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_request)
        .await
        .unwrap();

    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_body["user"]["id"].as_str().unwrap();

    crate::fixtures::assign_admin_role(&ctx.pool, uuid::Uuid::parse_str(user_id).unwrap()).await;

    let mut login_resp = ctx
        .server
        .post("/backend/public/auth/login")
        .send_json(&json!({"email": email, "password": password}))
        .await
        .unwrap();

    let login_body: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_body.get("token").unwrap().as_str().unwrap();

    // Create a post
    let post = BlogPostBuilder::new()
        .with_title("Original Title")
        .with_content("Original content")
        .with_status("draft")
        .persist(&ctx.pool)
        .await
        .expect("Failed to create test post");

    // Update the post
    let update_request = json!({
        "title": "Updated Title",
        "content": "Updated content"
    });

    let mut resp = ctx
        .server
        .put(&format!("/backend/protected/admin/blog/posts/{}", post.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_json(&update_request)
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["title"], "Updated Title");
    assert_eq!(body["content"], "Updated content");
}

#[actix_web::test]
async fn test_delete_post_success() {
    let ctx = TestContext::builder().build().await;

    // Create admin user and get token
    let email = crate::fixtures::unique_test_email();
    let password = "TestPassword123!";

    let register_request = json!({
        "email": email,
        "password": password,
        "display_name": "Admin User"
    });

    let mut register_resp = ctx
        .server
        .post("/backend/public/auth/register")
        .send_json(&register_request)
        .await
        .unwrap();

    let register_body: serde_json::Value = register_resp.json().await.unwrap();
    let user_id = register_body["user"]["id"].as_str().unwrap();

    crate::fixtures::assign_admin_role(&ctx.pool, uuid::Uuid::parse_str(user_id).unwrap()).await;

    let mut login_resp = ctx
        .server
        .post("/backend/public/auth/login")
        .send_json(&json!({"email": email, "password": password}))
        .await
        .unwrap();

    let login_body: serde_json::Value = login_resp.json().await.unwrap();
    let token = login_body.get("token").unwrap().as_str().unwrap();

    // Create a post
    let post = BlogPostBuilder::new()
        .with_title("Post to Delete")
        .persist(&ctx.pool)
        .await
        .expect("Failed to create test post");

    // Delete the post
    let resp = ctx
        .server
        .delete(&format!("/backend/protected/admin/blog/posts/{}", post.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 204); // No Content
}

#[actix_web::test]
async fn test_list_posts_pagination() {
    let ctx = TestContext::builder().build().await;

    // Create multiple published posts
    for i in 1..=15 {
        let _ = BlogPostBuilder::new()
            .with_title(&format!("Post {}", i))
            .with_slug(&format!("post-{}", i))
            .with_status("published")
            .published_at(chrono::Utc::now())
            .persist(&ctx.pool)
            .await
            .expect("Failed to create test post");
    }

    // Get first page
    let mut resp = ctx
        .server
        .get("/backend/public/blog/posts?status=published&page=1&limit=10")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["posts"].as_array().unwrap().len(), 10);
    assert!(body.get("total").is_some());
    assert!(body.get("page").is_some());
}
