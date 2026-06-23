use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::BlogPost;
use crate::repositories::traits::blog_repository::{
    BlogPostFilters, BlogPostList, BlogRepository, CreateBlogPost, TagCount, UpdateBlogPost,
};

pub struct PostgresBlogRepository {
    pool: PgPool,
}

impl PostgresBlogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BlogRepository for PostgresBlogRepository {
    async fn create_post(&self, post: CreateBlogPost) -> Result<BlogPost> {
        let created_post = sqlx::query_as!(
            BlogPost,
            r#"
            INSERT INTO blog_posts (
                slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags, published_at, meta_description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags as "tags!", published_at, created_at, updated_at, meta_description
            "#,
            post.slug,
            post.title,
            post.excerpt,
            post.content,
            post.featured_image_url,
            post.featured_image_alt,
            post.status,
            &post.tags,
            post.published_at,
            post.meta_description,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_post)
    }

    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<BlogPost>> {
        let post = sqlx::query_as!(
            BlogPost,
            r#"
            SELECT
                id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags as "tags!", published_at, created_at, updated_at, meta_description
            FROM blog_posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>> {
        let post = sqlx::query_as!(
            BlogPost,
            r#"
            SELECT
                id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags as "tags!", published_at, created_at, updated_at, meta_description
            FROM blog_posts
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn list_posts(&self, filters: BlogPostFilters) -> Result<BlogPostList> {
        let limit = filters.limit as i64;
        let offset = ((filters.page - 1) * filters.limit) as i64;

        // Build query based on filters using match for type safety
        let (total, posts) = match (&filters.status, &filters.tag) {
            (Some(status), Some(tag)) => {
                let total = sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM blog_posts WHERE status = $1 AND $2 = ANY(tags)",
                    status,
                    tag
                )
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(0);

                let posts = sqlx::query_as!(
                    BlogPost,
                    r#"
                    SELECT
                        id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                        status, tags as "tags!", published_at, created_at, updated_at, meta_description
                    FROM blog_posts
                    WHERE status = $1 AND $2 = ANY(tags)
                    ORDER BY created_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                    status,
                    tag,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
            (Some(status), None) => {
                let total = sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM blog_posts WHERE status = $1",
                    status
                )
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(0);

                let posts = sqlx::query_as!(
                    BlogPost,
                    r#"
                    SELECT
                        id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                        status, tags as "tags!", published_at, created_at, updated_at, meta_description
                    FROM blog_posts
                    WHERE status = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    status,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
            (None, Some(tag)) => {
                let total = sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM blog_posts WHERE $1 = ANY(tags)",
                    tag
                )
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(0);

                let posts = sqlx::query_as!(
                    BlogPost,
                    r#"
                    SELECT
                        id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                        status, tags as "tags!", published_at, created_at, updated_at, meta_description
                    FROM blog_posts
                    WHERE $1 = ANY(tags)
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    tag,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
            (None, None) => {
                let total = sqlx::query_scalar!("SELECT COUNT(*) FROM blog_posts")
                    .fetch_one(&self.pool)
                    .await?
                    .unwrap_or(0);

                let posts = sqlx::query_as!(
                    BlogPost,
                    r#"
                    SELECT
                        id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                        status, tags as "tags!", published_at, created_at, updated_at, meta_description
                    FROM blog_posts
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
        };

        let total_pages = ((total as f64) / (filters.limit as f64)).ceil() as i32;

        Ok(BlogPostList {
            posts,
            total,
            page: filters.page,
            total_pages,
        })
    }

    async fn update_post(&self, id: Uuid, post: UpdateBlogPost) -> Result<BlogPost> {
        // COALESCE keeps the existing value when a field is not provided (None).
        let updated_post = sqlx::query_as!(
            BlogPost,
            r#"
            UPDATE blog_posts
            SET
                slug = COALESCE($2, slug),
                title = COALESCE($3, title),
                excerpt = COALESCE($4, excerpt),
                content = COALESCE($5, content),
                featured_image_url = COALESCE($6, featured_image_url),
                featured_image_alt = COALESCE($7, featured_image_alt),
                status = COALESCE($8, status),
                tags = COALESCE($9, tags),
                published_at = COALESCE($10, published_at),
                meta_description = COALESCE($11, meta_description),
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags as "tags!", published_at, created_at, updated_at, meta_description
            "#,
            id,
            post.slug,
            post.title,
            post.excerpt,
            post.content,
            post.featured_image_url,
            post.featured_image_alt,
            post.status,
            post.tags.as_deref(),
            post.published_at,
            post.meta_description,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_post)
    }

    async fn delete_post(&self, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM blog_posts WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn search_posts(&self, query: &str, page: i32, limit: i32) -> Result<BlogPostList> {
        let limit_i64 = limit as i64;
        let offset = ((page - 1) * limit) as i64;

        // Prefix-match full-text search using the search_vector generated column
        let search_query = query
            .split_whitespace()
            .map(|word| format!("{}:*", word))
            .collect::<Vec<_>>()
            .join(" & ");

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM blog_posts WHERE search_vector @@ to_tsquery('english', $1)",
            search_query
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        let posts = sqlx::query_as!(
            BlogPost,
            r#"
            SELECT
                id, slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags as "tags!", published_at, created_at, updated_at, meta_description
            FROM blog_posts
            WHERE search_vector @@ to_tsquery('english', $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            search_query,
            limit_i64,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total_pages = ((total as f64) / (limit as f64)).ceil() as i32;

        Ok(BlogPostList {
            posts,
            total,
            page,
            total_pages,
        })
    }

    async fn get_all_tags(&self, status: Option<String>) -> Result<Vec<TagCount>> {
        let tags = if let Some(status_filter) = status {
            sqlx::query_as!(
                TagCount,
                r#"
                SELECT tag AS "tag!", COUNT(*) AS "count!"
                FROM blog_posts, UNNEST(tags) AS tag
                WHERE status = $1
                GROUP BY tag
                ORDER BY COUNT(*) DESC, tag
                "#,
                status_filter
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                TagCount,
                r#"
                SELECT tag AS "tag!", COUNT(*) AS "count!"
                FROM blog_posts, UNNEST(tags) AS tag
                GROUP BY tag
                ORDER BY COUNT(*) DESC, tag
                "#,
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(tags)
    }
}
