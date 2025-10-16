#![cfg(feature = "mocks")]

use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::AccessRequest;
use crate::repositories::traits::access_request_repository::{
    AccessRequestRepository, PendingRequestWithUser,
};

mock! {
    pub AccessRequestRepository {}

    #[async_trait]
    impl AccessRequestRepository for AccessRequestRepository {
        async fn create_request(
            &self,
            user_id: Uuid,
            message: String,
            requested_role: String,
        ) -> Result<AccessRequest>;

        async fn get_request_by_id(&self, request_id: Uuid) -> Result<Option<AccessRequest>>;

        async fn get_user_requests(&self, user_id: Uuid) -> Result<Vec<AccessRequest>>;

        async fn get_pending_requests(&self) -> Result<Vec<PendingRequestWithUser>>;

        async fn approve_request(
            &self,
            request_id: Uuid,
            admin_id: Uuid,
            admin_reason: Option<String>,
        ) -> Result<()>;

        async fn reject_request(
            &self,
            request_id: Uuid,
            admin_id: Uuid,
            admin_reason: Option<String>,
        ) -> Result<()>;

        async fn count_all_requests(&self) -> Result<i64>;

        async fn count_pending_requests(&self) -> Result<i64>;
    }
}
