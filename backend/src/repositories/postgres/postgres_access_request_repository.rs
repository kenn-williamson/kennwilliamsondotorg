use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::AccessRequest;
use crate::repositories::traits::access_request_repository::{
    AccessRequestRepository, PendingRequestWithUser,
};

pub struct PostgresAccessRequestRepository {
    pool: PgPool,
}

impl PostgresAccessRequestRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccessRequestRepository for PostgresAccessRequestRepository {
    async fn create_request(
        &self,
        user_id: Uuid,
        message: String,
        requested_role: String,
    ) -> Result<AccessRequest> {
        let request = sqlx::query_as!(
            AccessRequest,
            r#"
            INSERT INTO access_requests (user_id, message, requested_role, status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id, user_id, message, requested_role, status, admin_id, admin_reason, created_at, updated_at
            "#,
            user_id,
            message,
            requested_role
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(request)
    }

    async fn get_request_by_id(&self, request_id: Uuid) -> Result<Option<AccessRequest>> {
        let request = sqlx::query_as!(
            AccessRequest,
            r#"
            SELECT id, user_id, message, requested_role, status, admin_id, admin_reason, created_at, updated_at
            FROM access_requests
            WHERE id = $1
            "#,
            request_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(request)
    }

    async fn get_user_requests(&self, user_id: Uuid) -> Result<Vec<AccessRequest>> {
        let requests = sqlx::query_as!(
            AccessRequest,
            r#"
            SELECT id, user_id, message, requested_role, status, admin_id, admin_reason, created_at, updated_at
            FROM access_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(requests)
    }

    async fn get_pending_requests(&self) -> Result<Vec<PendingRequestWithUser>> {
        let requests = sqlx::query!(
            r#"
            SELECT
                ar.id,
                ar.user_id,
                u.email as user_email,
                u.display_name as user_display_name,
                ar.message,
                ar.requested_role,
                ar.created_at
            FROM access_requests ar
            JOIN users u ON ar.user_id = u.id
            WHERE ar.status = 'pending'
            ORDER BY ar.created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(requests
            .into_iter()
            .map(|row| PendingRequestWithUser {
                id: row.id,
                user_id: row.user_id,
                user_email: row.user_email,
                user_display_name: row.user_display_name,
                message: row.message,
                requested_role: row.requested_role,
                created_at: row.created_at,
            })
            .collect())
    }

    async fn approve_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        // Use a transaction to ensure both operations succeed or fail together
        let mut tx = self.pool.begin().await?;

        // First, get the request details to know which role to grant and to which user
        let request = sqlx::query!(
            r#"
            SELECT user_id, requested_role
            FROM access_requests
            WHERE id = $1 AND status = 'pending'
            "#,
            request_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        let request = request.ok_or_else(|| {
            anyhow::anyhow!("Access request not found or already processed")
        })?;

        // Update the access request status
        sqlx::query!(
            r#"
            UPDATE access_requests
            SET status = 'approved', admin_id = $1, admin_reason = $2, updated_at = NOW()
            WHERE id = $3 AND status = 'pending'
            "#,
            admin_id,
            admin_reason,
            request_id
        )
        .execute(&mut *tx)
        .await?;

        // Grant the requested role to the user (if not already assigned)
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = $2
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
            request.user_id,
            request.requested_role
        )
        .execute(&mut *tx)
        .await?;

        // Commit the transaction
        tx.commit().await?;

        Ok(())
    }

    async fn reject_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE access_requests
            SET status = 'rejected', admin_id = $1, admin_reason = $2, updated_at = NOW()
            WHERE id = $3 AND status = 'pending'
            "#,
            admin_id,
            admin_reason,
            request_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn count_all_requests(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM access_requests")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn count_pending_requests(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM access_requests WHERE status = 'pending'",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}
