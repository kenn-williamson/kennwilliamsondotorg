use sqlx::PgPool;

/// Statistics service for admin dashboard
pub struct StatsService;

impl StatsService {
    /// Get system statistics for admin dashboard
    pub async fn get_system_stats(pool: &PgPool) -> Result<SystemStats, sqlx::Error> {
        // Get all stats in parallel
        let (total_users, active_users, pending_suggestions, total_phrases) = tokio::try_join!(
            Self::get_total_users(pool),
            Self::get_active_users(pool),
            Self::get_pending_suggestions_count(pool),
            Self::get_total_phrases(pool)
        )?;

        Ok(SystemStats {
            total_users,
            active_users,
            pending_suggestions,
            total_phrases,
        })
    }

    /// Get total user count
    async fn get_total_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0))
    }

    /// Get active user count (non-deactivated users)
    async fn get_active_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM users WHERE active = true"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0))
    }

    /// Get pending phrase suggestions count
    async fn get_pending_suggestions_count(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM phrase_suggestions WHERE status = 'pending'"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0))
    }

    /// Get total active phrases count
    async fn get_total_phrases(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM phrases WHERE active = true"
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0))
    }
}

/// System statistics for admin dashboard
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemStats {
    pub total_users: i64,
    pub active_users: i64,
    pub pending_suggestions: i64,
    pub total_phrases: i64,
}
