use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::events::EventPublisher;
use crate::events::types::{
    AccessRequestApprovedEvent, AccessRequestCreatedEvent, AccessRequestRejectedEvent,
};
use crate::models::api::access_request::{
    AccessRequestListResponse, AccessRequestWithUserResponse,
};
use crate::repositories::traits::{AccessRequestRepository, AdminRepository};
use crate::services::email::{
    EmailService,
    templates::{AccessRequestNotificationTemplate, Email, EmailTemplate},
};

/// Access request moderation service for admin operations
pub struct AccessRequestModerationService {
    access_request_repository: Arc<dyn AccessRequestRepository>,
    admin_repository: Option<Arc<dyn AdminRepository>>,
    email_service: Option<Arc<dyn EmailService>>,
    frontend_url: Option<String>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

impl std::fmt::Debug for AccessRequestModerationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessRequestModerationService")
            .field(
                "access_request_repository",
                &"Arc<dyn AccessRequestRepository>",
            )
            .field(
                "admin_repository",
                &self
                    .admin_repository
                    .as_ref()
                    .map(|_| "Arc<dyn AdminRepository>"),
            )
            .field(
                "email_service",
                &self.email_service.as_ref().map(|_| "Arc<dyn EmailService>"),
            )
            .field("frontend_url", &self.frontend_url)
            .field(
                "event_bus",
                &self.event_bus.as_ref().map(|_| "Arc<dyn EventPublisher>"),
            )
            .finish()
    }
}

/// Builder for AccessRequestModerationService
pub struct AccessRequestModerationServiceBuilder {
    access_request_repository: Option<Box<dyn AccessRequestRepository>>,
    admin_repository: Option<Box<dyn AdminRepository>>,
    email_service: Option<Box<dyn EmailService>>,
    frontend_url: Option<String>,
    event_bus: Option<Arc<dyn EventPublisher>>,
}

impl Default for AccessRequestModerationServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AccessRequestModerationServiceBuilder {
    pub fn new() -> Self {
        Self {
            access_request_repository: None,
            admin_repository: None,
            email_service: None,
            frontend_url: None,
            event_bus: None,
        }
    }

    pub fn with_access_request_repository(
        mut self,
        repo: Box<dyn AccessRequestRepository>,
    ) -> Self {
        self.access_request_repository = Some(repo);
        self
    }

    pub fn with_admin_repository(mut self, repo: Box<dyn AdminRepository>) -> Self {
        self.admin_repository = Some(repo);
        self
    }

    pub fn with_email_service(mut self, service: Box<dyn EmailService>) -> Self {
        self.email_service = Some(service);
        self
    }

    pub fn with_frontend_url(mut self, url: impl Into<String>) -> Self {
        self.frontend_url = Some(url.into());
        self
    }

    pub fn with_event_bus(mut self, event_bus: Arc<dyn EventPublisher>) -> Self {
        self.event_bus = Some(event_bus);
        self
    }

    pub fn build(self) -> Result<AccessRequestModerationService> {
        let access_request_repository = self
            .access_request_repository
            .ok_or_else(|| anyhow::anyhow!("AccessRequestRepository is required"))?;

        Ok(AccessRequestModerationService {
            access_request_repository: Arc::from(access_request_repository),
            admin_repository: self.admin_repository.map(Arc::from),
            email_service: self.email_service.map(Arc::from),
            frontend_url: self.frontend_url,
            event_bus: self.event_bus,
        })
    }
}

impl AccessRequestModerationService {
    /// Create a new builder for AccessRequestModerationService
    pub fn builder() -> AccessRequestModerationServiceBuilder {
        AccessRequestModerationServiceBuilder::new()
    }

    /// Create service with minimal dependencies (backward compatibility)
    ///
    /// For new code, prefer using the builder pattern:
    /// ```ignore
    /// AccessRequestModerationService::builder()
    ///     .with_access_request_repository(repo)
    ///     .with_admin_repository(admin_repo)
    ///     .with_email_service(email_service)
    ///     .with_frontend_url("https://kennwilliamson.org")
    ///     .with_event_bus(event_bus)
    ///     .build()
    /// ```
    pub fn new(access_request_repository: Box<dyn AccessRequestRepository>) -> Self {
        Self {
            access_request_repository: Arc::from(access_request_repository),
            admin_repository: None,
            email_service: None,
            frontend_url: None,
            event_bus: None,
        }
    }

    /// Create a new access request (user-facing)
    ///
    /// Creates the access request in the database and sends email notifications
    /// to all admin users (if email dependencies are configured).
    ///
    /// # Arguments
    /// * `user_id` - ID of the user making the request
    /// * `user_email` - Email of the user (for logging/debugging)
    /// * `user_display_name` - Display name for email personalization
    /// * `message` - User's message explaining why they need access
    /// * `requested_role` - Role being requested (e.g., "trusted-contact")
    ///
    /// # Email Notifications
    /// Email sending is fire-and-forget - failures are logged but don't block the request.
    /// Emails are sent via domain events if EventBus is configured, otherwise via direct
    /// email service if email dependencies are present.
    pub async fn create_request(
        &self,
        user_id: Uuid,
        user_email: String,
        user_display_name: String,
        message: String,
        requested_role: String,
    ) -> Result<()> {
        // Create the access request in database
        self.access_request_repository
            .create_request(user_id, message.clone(), requested_role.clone())
            .await?;

        // Emit domain event if EventBus is configured (Phase 2)
        if let Some(event_bus) = &self.event_bus {
            let event = AccessRequestCreatedEvent::new(
                user_id,
                &user_email,
                &user_display_name,
                &message,
                &requested_role,
            );

            // Fire-and-forget event publishing (box for type erasure)
            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!("Failed to publish AccessRequestCreatedEvent: {}", e);
            } else {
                log::debug!(
                    "Published AccessRequestCreatedEvent for user {} ({})",
                    user_display_name,
                    user_email
                );
            }
        } else {
            // Fallback to Phase 1 direct email sending
            self.send_notification_emails(
                &user_email,
                &user_display_name,
                message,
                &requested_role,
            )
            .await;
        }

        Ok(())
    }

    /// Send notification emails to all admins (fire-and-forget)
    ///
    /// This method logs errors but never returns them, implementing the fire-and-forget pattern.
    /// Email failures should never block user operations.
    async fn send_notification_emails(
        &self,
        user_email: &str,
        user_display_name: &str,
        message: String,
        requested_role: &str,
    ) {
        // Check if email dependencies are configured
        let admin_repo = match &self.admin_repository {
            Some(repo) => repo,
            None => {
                log::info!("Email notifications disabled: AdminRepository not configured");
                return;
            }
        };

        let email_service = match &self.email_service {
            Some(service) => service,
            None => {
                log::info!("Email notifications disabled: EmailService not configured");
                return;
            }
        };

        let frontend_url = match &self.frontend_url {
            Some(url) => url,
            None => {
                log::info!("Email notifications disabled: FRONTEND_URL not configured");
                return;
            }
        };

        // Get admin emails
        let admin_emails = match admin_repo.get_admin_emails().await {
            Ok(emails) => emails,
            Err(e) => {
                log::error!("Failed to fetch admin emails: {}", e);
                return;
            }
        };

        if admin_emails.is_empty() {
            log::warn!("No admin emails found - cannot send access request notification");
            return;
        }

        // Build email template
        let template = AccessRequestNotificationTemplate::new(
            user_display_name,
            Some(message),
            requested_role,
            frontend_url,
        );

        // Render template
        let html_body = match template.render_html() {
            Ok(html) => html,
            Err(e) => {
                log::error!("Failed to render email HTML: {}", e);
                return;
            }
        };

        let text_body = template.render_plain_text();
        let subject = template.subject();

        // Build email
        let email = match Email::builder()
            .with_recipients(admin_emails.clone())
            .subject(subject)
            .text_body(text_body)
            .html_body(html_body)
            .build()
        {
            Ok(email) => email,
            Err(e) => {
                log::error!("Failed to build email: {}", e);
                return;
            }
        };

        // Send email (fire-and-forget)
        if let Err(e) = email_service.send_email(email).await {
            log::error!(
                "Failed to send access request notification email to {} admin(s): {}",
                admin_emails.len(),
                e
            );
        } else {
            log::info!(
                "Sent access request notification for user '{}' ({}) to {} admin(s)",
                user_display_name,
                user_email,
                admin_emails.len()
            );
        }
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
        // Fetch the access request details first to get user_id and requested_role
        let access_request = self
            .access_request_repository
            .get_request_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Access request not found"))?;

        // Approve the request in database
        self.access_request_repository
            .approve_request(request_id, admin_id, admin_reason.clone())
            .await?;

        // Emit event if EventBus is configured
        if let Some(event_bus) = &self.event_bus {
            let event = AccessRequestApprovedEvent::new(
                access_request.user_id,
                &access_request.requested_role,
                admin_reason,
            );

            // Fire-and-forget event publishing
            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!("Failed to publish AccessRequestApprovedEvent: {}", e);
            } else {
                log::debug!(
                    "Published AccessRequestApprovedEvent for user_id {}",
                    access_request.user_id
                );
            }
        }

        Ok(())
    }

    /// Reject an access request
    pub async fn reject_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        admin_reason: Option<String>,
    ) -> Result<()> {
        // Fetch the access request details first to get user_id
        let access_request = self
            .access_request_repository
            .get_request_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Access request not found"))?;

        // Reject the request in database
        self.access_request_repository
            .reject_request(request_id, admin_id, admin_reason.clone())
            .await?;

        // Emit event if EventBus is configured
        if let Some(event_bus) = &self.event_bus {
            let event = AccessRequestRejectedEvent::new(access_request.user_id, admin_reason);

            // Fire-and-forget event publishing
            if let Err(e) = event_bus.publish(Box::new(event)).await {
                log::error!("Failed to publish AccessRequestRejectedEvent: {}", e);
            } else {
                log::debug!(
                    "Published AccessRequestRejectedEvent for user_id {}",
                    access_request.user_id
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::mocks::{MockAccessRequestRepository, MockAdminRepository};
    use crate::repositories::traits::access_request_repository::PendingRequestWithUser;
    use crate::services::email::MockEmailService;
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_request_success_without_email() {
        // Setup mocks (no email service configured)
        let mut mock_repo = MockAccessRequestRepository::new();
        let user_id = Uuid::new_v4();
        let message = "I would like access please".to_string();
        let requested_role = "trusted-contact".to_string();

        // Configure mock expectations
        mock_repo
            .expect_create_request()
            .with(eq(user_id), eq(message.clone()), eq(requested_role.clone()))
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

        // Create service without email dependencies
        let service = AccessRequestModerationService::new(Box::new(mock_repo));

        // Test
        let result = service
            .create_request(
                user_id,
                "test@example.com".to_string(),
                "Test User".to_string(),
                message,
                requested_role,
            )
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_request_success_with_email() {
        // Setup mocks
        let mut mock_repo = MockAccessRequestRepository::new();
        let mut mock_admin_repo = MockAdminRepository::new();
        let mock_email_service = MockEmailService::new();

        let user_id = Uuid::new_v4();
        let message = "I would like access please".to_string();
        let requested_role = "trusted-contact".to_string();

        // Configure access request repository mock
        mock_repo
            .expect_create_request()
            .with(eq(user_id), eq(message.clone()), eq(requested_role.clone()))
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

        // Configure admin repository to return admin emails
        mock_admin_repo
            .expect_get_admin_emails()
            .times(1)
            .returning(|| Ok(vec!["admin@example.com".to_string()]));

        // Clone email service to verify emails after service consumes it
        let email_service_clone = mock_email_service.clone();

        // Create service with all dependencies
        let service = AccessRequestModerationService::builder()
            .with_access_request_repository(Box::new(mock_repo))
            .with_admin_repository(Box::new(mock_admin_repo))
            .with_email_service(Box::new(mock_email_service))
            .with_frontend_url("https://kennwilliamson.org")
            .build()
            .expect("Failed to build service");

        // Test
        let result = service
            .create_request(
                user_id,
                "test@example.com".to_string(),
                "Test User".to_string(),
                message,
                requested_role,
            )
            .await;

        // Assert
        assert!(result.is_ok());

        // Verify email was sent
        assert_eq!(email_service_clone.count(), 1);

        // Verify email content
        let sent_emails = email_service_clone.get_sent_emails();
        assert_eq!(sent_emails.len(), 1);
        assert_eq!(sent_emails[0].to, vec!["admin@example.com"]);
        assert!(sent_emails[0].subject.contains("Access Request"));
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
        let user_id = Uuid::new_v4();

        // Configure mock expectations - service now fetches request first
        mock_repo
            .expect_get_request_by_id()
            .with(eq(request_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(crate::models::db::AccessRequest {
                    id: request_id,
                    user_id,
                    message: "Test message".to_string(),
                    requested_role: "trusted-contact".to_string(),
                    status: "pending".to_string(),
                    admin_id: None,
                    admin_reason: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }))
            });

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
        let user_id = Uuid::new_v4();

        // Configure mock expectations - service now fetches request first
        mock_repo
            .expect_get_request_by_id()
            .with(eq(request_id))
            .times(1)
            .returning(move |_| {
                Ok(Some(crate::models::db::AccessRequest {
                    id: request_id,
                    user_id,
                    message: "Test message".to_string(),
                    requested_role: "trusted-contact".to_string(),
                    status: "pending".to_string(),
                    admin_id: None,
                    admin_reason: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }))
            });

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

    #[tokio::test]
    async fn test_builder_pattern() {
        let mock_repo = MockAccessRequestRepository::new();
        let mock_admin_repo = MockAdminRepository::new();
        let mock_email_service = MockEmailService::new();

        let result = AccessRequestModerationService::builder()
            .with_access_request_repository(Box::new(mock_repo))
            .with_admin_repository(Box::new(mock_admin_repo))
            .with_email_service(Box::new(mock_email_service))
            .with_frontend_url("https://kennwilliamson.org")
            .build();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_builder_pattern_missing_required() {
        let result = AccessRequestModerationService::builder()
            .with_frontend_url("https://kennwilliamson.org")
            .build();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("AccessRequestRepository is required")
        );
    }
}
