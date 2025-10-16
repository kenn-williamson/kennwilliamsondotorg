use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::api::access_request::{AccessRequestListResponse, AccessRequestWithUserResponse};
use crate::repositories::traits::AccessRequestRepository;

/// Access request moderation service for admin operations
pub struct AccessRequestModerationService {
    access_request_repository: Arc<dyn AccessRequestRepository>,
}

impl AccessRequestModerationService {
    pub fn new(access_request_repository: Box<dyn AccessRequestRepository>) -> Self {
        Self {
            access_request_repository: Arc::from(access_request_repository),
        }
    }

    /// Create a new access request (user-facing)
    pub async fn create_request(
        &self,
        user_id: Uuid,
        message: String,
        requested_role: String,
    ) -> Result<()> {
        self.access_request_repository
            .create_request(user_id, message, requested_role)
            .await?;
        Ok(())
    }

    /// Get all pending access requests with user information
    pub async fn get_pending_requests(&self) -> Result<AccessRequestListResponse> {
        // Get pending requests from repository
        let requests = self
            .access_request_repository
            .get_pending_requests()
            .await?;

        // Convert to response format
        let pending_requests: Vec<AccessRequestWithUserResponse> = requests
            .into_iter()
            .map(|request| AccessRequestWithUserResponse {
                id: request.id,
                user_id: request.user_id,
                user_email: request.user_email,
                user_display_name: request.user_display_name,
                message: request.message,
                requested_role: request.requested_role,
                status: "pending".to_string(),
                admin_id: None,
                admin_reason: None,
                created_at: request.created_at,
                updated_at: request.created_at, // For pending requests, updated_at = created_at
            })
            .collect();

        let total = pending_requests.len() as i64;

        Ok(AccessRequestListResponse {
            requests: pending_requests,
            total,
        })
    }

    /// Approve an access request
    pub async fn approve_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        self.access_request_repository
            .approve_request(request_id, admin_id, admin_reason)
            .await
    }

    /// Reject an access request
    pub async fn reject_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        self.access_request_repository
            .reject_request(request_id, admin_id, admin_reason)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::MockAccessRequestRepository;
    use crate::repositories::traits::access_request_repository::PendingRequestWithUser;
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_request_success() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();
        let user_id = Uuid::new_v4();
        let message = "I would like access please".to_string();
        let requested_role = "trusted-contact".to_string();

        // Configure mock expectations
        mock_repo
            .expect_create_request()
            .with(
                eq(user_id),
                eq(message.clone()),
                eq(requested_role.clone()),
            )
            .times(1)
            .returning(|user_id, message, requested_role| {
                Ok(crate::models::db::AccessRequest {
                    id: Uuid::new_v4(),
                    user_id,
                    message,
                    requested_role,
                    status: "pending".to_string(),
                    admin_id: None,
                    admin_reason: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
            });

        // Create service
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service
            .create_request(user_id, message, requested_role)
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_pending_requests_success() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();

        // Create test data
        let request = PendingRequestWithUser {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            user_email: "test@example.com".to_string(),
            user_display_name: "Test User".to_string(),
            message: "I would like access please".to_string(),
            requested_role: "trusted-contact".to_string(),
            created_at: Utc::now(),
        };

        // Configure mock expectations
        mock_repo
            .expect_get_pending_requests()
            .times(1)
            .returning(move || Ok(vec![request.clone()]));

        // Create service
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service.get_pending_requests().await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.requests.len(), 1);
        assert_eq!(response.requests[0].user_email, "test@example.com");
        assert_eq!(response.requests[0].message, "I would like access please");
    }

    #[tokio::test]
    async fn test_get_pending_requests_empty() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();

        // Configure mock expectations
        mock_repo
            .expect_get_pending_requests()
            .times(1)
            .returning(|| Ok(vec![]));

        // Create service
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service.get_pending_requests().await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.total, 0);
        assert_eq!(response.requests.len(), 0);
    }

    #[tokio::test]
    async fn test_get_pending_requests_repo_error() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();

        // Configure mock expectations
        mock_repo
            .expect_get_pending_requests()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("Database error")));

        // Create service
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service.get_pending_requests().await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_approve_request_success() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();
        let request_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();

        // Configure mock expectations
        mock_repo
            .expect_approve_request()
            .with(
                eq(request_id),
                eq(admin_id),
                eq(Some("Approved".to_string())),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        // Create service
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service
            .approve_request(request_id, admin_id, Some("Approved".to_string()))
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reject_request_success() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();
        let request_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();

        // Configure mock expectations
        mock_repo
            .expect_reject_request()
            .with(
                eq(request_id),
                eq(admin_id),
                eq(Some("Not appropriate".to_string())),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        // Create service
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service
            .reject_request(request_id, admin_id, Some("Not appropriate".to_string()))
            .await;

        // Assert
        assert!(result.is_ok());
    }
}
