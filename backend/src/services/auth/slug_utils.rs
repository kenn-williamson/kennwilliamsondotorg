use sqlx::PgPool;
use anyhow::Result;
use crate::models::api::{SlugPreviewRequest, SlugPreviewResponse};

#[derive(Clone)]
pub struct SlugUtils {
    pool: PgPool,
}

impl SlugUtils {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn generate_slug(display_name: &str) -> String {
        display_name
            .to_lowercase()
            .trim()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }

    pub async fn preview_slug(&self, request: SlugPreviewRequest) -> Result<SlugPreviewResponse> {
        let base_slug = Self::generate_slug(&request.display_name);
        let (available, final_slug) = self.find_available_slug(base_slug.clone()).await?;
        
        Ok(SlugPreviewResponse {
            slug: base_slug,
            available,
            final_slug,
        })
    }

    pub async fn find_available_slug(&self, base_slug: String) -> Result<(bool, String)> {
        // Check if base slug exists
        if !self.slug_exists(&base_slug).await? {
            return Ok((true, base_slug));
        }
        
        // Try numbered variants: slug-2, slug-3, etc.
        for i in 2..=999 {
            let candidate = format!("{}-{}", base_slug, i);
            if !self.slug_exists(&candidate).await? {
                return Ok((false, candidate));
            }
        }
        
        // Fallback: append timestamp if all numbered variants taken
        let timestamp = chrono::Utc::now().timestamp();
        Ok((false, format!("{}-{}", base_slug, timestamp)))
    }

    async fn slug_exists(&self, slug: &str) -> Result<bool> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE slug = $1")
            .bind(slug)
            .fetch_one(&self.pool)
            .await?;
        Ok(count > 0)
    }
}
