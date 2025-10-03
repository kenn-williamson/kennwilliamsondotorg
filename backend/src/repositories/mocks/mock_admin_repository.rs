use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::UserWithRoles;
use crate::repositories::traits::admin_repository::AdminRepository;

// Generate mock for AdminRepository trait
mock! {
    pub AdminRepository {}

    #[async_trait]
    impl AdminRepository for AdminRepository {
        async fn update_user_status(&self, user_id: Uuid, active: bool) -> Result<()>;
        async fn add_user_role(&self, user_id: Uuid, role: &str) -> Result<()>;
        async fn remove_user_role(&self, user_id: Uuid, role: &str) -> Result<()>;
        async fn get_all_users_with_roles(
            &self,
            search: Option<String>,
            limit: Option<i64>,
            offset: Option<i64>,
        ) -> Result<Vec<UserWithRoles>>;
        async fn count_all_users(&self) -> Result<i64>;
        async fn count_active_users(&self) -> Result<i64>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    // Helper function to create a test user with roles (repository DTO)
    fn create_test_user_with_roles() -> UserWithRoles {
        UserWithRoles {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            real_name: None,
            google_user_id: None,
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            roles: Some(vec!["user".to_string()]),
        }
    }

    // Helper function to create an admin user with roles (repository DTO)
    fn create_test_admin_user() -> UserWithRoles {
        UserWithRoles {
            id: Uuid::new_v4(),
            email: "admin@example.com".to_string(),
            display_name: "Admin User".to_string(),
            slug: "admin-user".to_string(),
            real_name: None,
            google_user_id: None,
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            roles: Some(vec!["user".to_string(), "admin".to_string()]),
        }
    }

    // Helper function to create multiple test users (repository DTOs)
    fn create_test_users() -> Vec<UserWithRoles> {
        vec![
            create_test_user_with_roles(),
            create_test_admin_user(),
            UserWithRoles {
                id: Uuid::new_v4(),
                email: "inactive@example.com".to_string(),
                display_name: "Inactive User".to_string(),
                slug: "inactive-user".to_string(),
                real_name: None,
                google_user_id: None,
                active: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                roles: Some(vec!["user".to_string()]),
            },
        ]
    }

    #[tokio::test]
    async fn test_update_user_status_activate() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_update_user_status()
            .times(1)
            .with(eq(user_id), eq(true))
            .returning(|_, _| Ok(()));

        let result = mock.update_user_status(user_id, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_status_deactivate() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_update_user_status()
            .times(1)
            .with(eq(user_id), eq(false))
            .returning(|_, _| Ok(()));

        let result = mock.update_user_status(user_id, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_status_error() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_update_user_status()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Database error")));

        let result = mock.update_user_status(user_id, true).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_add_user_role_admin() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_add_user_role()
            .times(1)
            .with(eq(user_id), eq("admin"))
            .returning(|_, _| Ok(()));

        let result = mock.add_user_role(user_id, "admin").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_user_role_moderator() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_add_user_role()
            .times(1)
            .with(eq(user_id), eq("moderator"))
            .returning(|_, _| Ok(()));

        let result = mock.add_user_role(user_id, "moderator").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_user_role_error() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_add_user_role()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Role not found")));

        let result = mock.add_user_role(user_id, "nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Role not found"));
    }

    #[tokio::test]
    async fn test_remove_user_role_admin() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_remove_user_role()
            .times(1)
            .with(eq(user_id), eq("admin"))
            .returning(|_, _| Ok(()));

        let result = mock.remove_user_role(user_id, "admin").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_user_role_error() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();

        mock.expect_remove_user_role()
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Role not found")));

        let result = mock.remove_user_role(user_id, "nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Role not found"));
    }

    #[tokio::test]
    async fn test_get_all_users_with_roles_no_params() {
        let mut mock = MockAdminRepository::new();
        let test_users = create_test_users();

        mock.expect_get_all_users_with_roles()
            .times(1)
            .with(eq(None), eq(None), eq(None))
            .returning(move |_, _, _| Ok(test_users.clone()));

        let result = mock.get_all_users_with_roles(None, None, None).await;
        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 3);
    }

    #[tokio::test]
    async fn test_get_all_users_with_roles_with_search() {
        let mut mock = MockAdminRepository::new();
        let test_users = create_test_users();

        mock.expect_get_all_users_with_roles()
            .times(1)
            .with(eq(Some("admin".to_string())), eq(None), eq(None))
            .returning(move |_, _, _| Ok(test_users.clone()));

        let result = mock
            .get_all_users_with_roles(Some("admin".to_string()), None, None)
            .await;
        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 3);
    }

    #[tokio::test]
    async fn test_get_all_users_with_roles_with_pagination() {
        let mut mock = MockAdminRepository::new();
        let test_users = create_test_users();

        mock.expect_get_all_users_with_roles()
            .times(1)
            .with(eq(None), eq(Some(10)), eq(Some(0)))
            .returning(move |_, _, _| Ok(test_users.clone()));

        let result = mock.get_all_users_with_roles(None, Some(10), Some(0)).await;
        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 3);
    }

    #[tokio::test]
    async fn test_get_all_users_with_roles_error() {
        let mut mock = MockAdminRepository::new();

        mock.expect_get_all_users_with_roles()
            .times(1)
            .returning(|_, _, _| Err(anyhow::anyhow!("Database connection failed")));

        let result = mock.get_all_users_with_roles(None, None, None).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Database connection failed"));
    }

    #[tokio::test]
    async fn test_count_all_users() {
        let mut mock = MockAdminRepository::new();

        mock.expect_count_all_users().times(1).returning(|| Ok(42));

        let result = mock.count_all_users().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_count_all_users_error() {
        let mut mock = MockAdminRepository::new();

        mock.expect_count_all_users()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("Database error")));

        let result = mock.count_all_users().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_count_active_users() {
        let mut mock = MockAdminRepository::new();

        mock.expect_count_active_users()
            .times(1)
            .returning(|| Ok(40));

        let result = mock.count_active_users().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 40);
    }

    #[tokio::test]
    async fn test_count_active_users_error() {
        let mut mock = MockAdminRepository::new();

        mock.expect_count_active_users()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("Database error")));

        let result = mock.count_active_users().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_multiple_operations() {
        let mut mock = MockAdminRepository::new();
        let user_id = Uuid::new_v4();
        let test_users = create_test_users();

        // Setup multiple expectations
        mock.expect_update_user_status()
            .times(1)
            .with(eq(user_id), eq(false))
            .returning(|_, _| Ok(()));

        mock.expect_add_user_role()
            .times(1)
            .with(eq(user_id), eq("admin"))
            .returning(|_, _| Ok(()));

        mock.expect_get_all_users_with_roles()
            .times(1)
            .with(eq(None), eq(None), eq(None))
            .returning(move |_, _, _| Ok(test_users.clone()));

        mock.expect_count_all_users().times(1).returning(|| Ok(3));

        // Execute multiple operations
        let deactivate_result = mock.update_user_status(user_id, false).await;
        assert!(deactivate_result.is_ok());

        let add_role_result = mock.add_user_role(user_id, "admin").await;
        assert!(add_role_result.is_ok());

        let users_result = mock.get_all_users_with_roles(None, None, None).await;
        assert!(users_result.is_ok());
        assert_eq!(users_result.unwrap().len(), 3);

        let count_result = mock.count_all_users().await;
        assert!(count_result.is_ok());
        assert_eq!(count_result.unwrap(), 3);
    }
}
