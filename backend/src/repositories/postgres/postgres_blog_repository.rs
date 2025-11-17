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
        let created_post = sqlx::query_as::<_, BlogPost>(
            r#"
            INSERT INTO blog_posts (
                slug, title, excerpt, content, featured_image_url, featured_image_alt,
                status, tags, published_at, meta_description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(post.slug)
        .bind(post.title)
        .bind(post.excerpt)
        .bind(post.content)
        .bind(post.featured_image_url)
        .bind(post.featured_image_alt)
        .bind(post.status)
        .bind(post.tags)
        .bind(post.published_at)
        .bind(post.meta_description)
        .fetch_one(&self.pool)
        .await?;

        Ok(created_post)
    }

    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<BlogPost>> {
        let post = sqlx::query_as::<_, BlogPost>(
            r#"
            SELECT * FROM blog_posts WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>> {
        let post = sqlx::query_as::<_, BlogPost>(
            r#"
            SELECT * FROM blog_posts WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn list_posts(&self, filters: BlogPostFilters) -> Result<BlogPostList> {
        // Calculate offset from page number
        let offset = (filters.page - 1) * filters.limit;

        // Build query based on filters using match for type safety
        let (total, posts) = match (&filters.status, &filters.tag) {
            (Some(status), Some(tag)) => {
                let total = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM blog_posts WHERE status = $1 AND $2 = ANY(tags)",
                )
                .bind(status)
                .bind(tag)
                .fetch_one(&self.pool)
                .await?;

                let posts = sqlx::query_as(
                    "SELECT * FROM blog_posts WHERE status = $1 AND $2 = ANY(tags) ORDER BY created_at DESC LIMIT $3 OFFSET $4"
                )
                .bind(status)
                .bind(tag)
                .bind(filters.limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
            (Some(status), None) => {
                let total = sqlx::query_scalar("SELECT COUNT(*) FROM blog_posts WHERE status = $1")
                    .bind(status)
                    .fetch_one(&self.pool)
                    .await?;

                let posts = sqlx::query_as(
                    "SELECT * FROM blog_posts WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
                )
                .bind(status)
                .bind(filters.limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
            (None, Some(tag)) => {
                let total =
                    sqlx::query_scalar("SELECT COUNT(*) FROM blog_posts WHERE $1 = ANY(tags)")
                        .bind(tag)
                        .fetch_one(&self.pool)
                        .await?;

                let posts = sqlx::query_as(
                    "SELECT * FROM blog_posts WHERE $1 = ANY(tags) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
                )
                .bind(tag)
                .bind(filters.limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

                (total, posts)
            }
            (None, None) => {
                let total = sqlx::query_scalar("SELECT COUNT(*) FROM blog_posts")
                    .fetch_one(&self.pool)
                    .await?;

                let posts = sqlx::query_as(
                    "SELECT * FROM blog_posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
                )
                .bind(filters.limit)
                .bind(offset)
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
        // Build SET clause dynamically based on provided fields
        let mut set_clauses = Vec::new();
        let mut param_index = 2; // Start at 2 because $1 is the ID

        if post.slug.is_some() {
            set_clauses.push(format!("slug = ${}", param_index));
            param_index += 1;
        }
        if post.title.is_some() {
            set_clauses.push(format!("title = ${}", param_index));
            param_index += 1;
        }
        if post.excerpt.is_some() {
            set_clauses.push(format!("excerpt = ${}", param_index));
            param_index += 1;
        }
        if post.content.is_some() {
            set_clauses.push(format!("content = ${}", param_index));
            param_index += 1;
        }
        if post.featured_image_url.is_some() {
            set_clauses.push(format!("featured_image_url = ${}", param_index));
            param_index += 1;
        }
        if post.featured_image_alt.is_some() {
            set_clauses.push(format!("featured_image_alt = ${}", param_index));
            param_index += 1;
        }
        if post.status.is_some() {
            set_clauses.push(format!("status = ${}", param_index));
            param_index += 1;
        }
        if post.tags.is_some() {
            set_clauses.push(format!("tags = ${}", param_index));
            param_index += 1;
        }
        if post.published_at.is_some() {
            set_clauses.push(format!("published_at = ${}", param_index));
            param_index += 1;
        }
        if post.meta_description.is_some() {
            set_clauses.push(format!("meta_description = ${}", param_index));
        }

        // Always update updated_at
        set_clauses.push("updated_at = NOW()".to_string());

        let set_clause = set_clauses.join(", ");
        let query = format!(
            "UPDATE blog_posts SET {} WHERE id = $1 RETURNING *",
            set_clause
        );

        let mut query_builder = sqlx::query_as::<_, BlogPost>(&query);
        query_builder = query_builder.bind(id);

        if let Some(slug) = post.slug {
            query_builder = query_builder.bind(slug);
        }
        if let Some(title) = post.title {
            query_builder = query_builder.bind(title);
        }
        if let Some(excerpt) = post.excerpt {
            query_builder = query_builder.bind(excerpt);
        }
        if let Some(content) = post.content {
            query_builder = query_builder.bind(content);
        }
        if let Some(featured_image_url) = post.featured_image_url {
            query_builder = query_builder.bind(featured_image_url);
        }
        if let Some(featured_image_alt) = post.featured_image_alt {
            query_builder = query_builder.bind(featured_image_alt);
        }
        if let Some(status) = post.status {
            query_builder = query_builder.bind(status);
        }
        if let Some(tags) = post.tags {
            query_builder = query_builder.bind(tags);
        }
        if let Some(published_at) = post.published_at {
            query_builder = query_builder.bind(published_at);
        }
        if let Some(meta_description) = post.meta_description {
            query_builder = query_builder.bind(meta_description);
        }

        let updated_post = query_builder.fetch_one(&self.pool).await?;

        Ok(updated_post)
    }

    async fn delete_post(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM blog_posts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn search_posts(&self, query: &str, page: i32, limit: i32) -> Result<BlogPostList> {
        let offset = (page - 1) * limit;

        // PostgreSQL full-text search using the search_vector generated column
        let search_query = query
            .split_whitespace()
            .map(|word| format!("{}:*", word))
            .collect::<Vec<_>>()
            .join(" & ");

        // Count total matching posts
        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM blog_posts
            WHERE search_vector @@ to_tsquery('english', $1)
            "#,
        )
        .bind(&search_query)
        .fetch_one(&self.pool)
        .await?;

        // Get matching posts with pagination
        let posts = sqlx::query_as::<_, BlogPost>(
            r#"
            SELECT *
            FROM blog_posts
            WHERE search_vector @@ to_tsquery('english', $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_query)
        .bind(limit)
        .bind(offset)
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
            sqlx::query_as::<_, TagCount>(
                r#"
                SELECT tag, COUNT(*) as count
                FROM blog_posts, UNNEST(tags) as tag
                WHERE status = $1
                GROUP BY tag
                ORDER BY count DESC, tag
                "#,
            )
            .bind(status_filter)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, TagCount>(
                r#"
                SELECT tag, COUNT(*) as count
                FROM blog_posts, UNNEST(tags) as tag
                GROUP BY tag
                ORDER BY count DESC, tag
                "#,
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(tags)
    }
}
