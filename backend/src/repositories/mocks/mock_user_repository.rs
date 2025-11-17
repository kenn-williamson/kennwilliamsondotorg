use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::models::db::user::{User, UserWithTimer};
use crate::repositories::traits::user_repository::{
    CreateOAuthUserData, CreateUserData, UserRepository, UserUpdates,
};

// Generate mock for UserRepository trait
mock! {
    pub UserRepository {}

    #[async_trait]
    impl UserRepository for UserRepository {
        async fn create_user(&self, user_data: &CreateUserData) -> Result<User>;
        async fn create_user_with_auth_data(&self, user_data: &CreateUserData, password_hash: String) -> Result<User>;
        async fn create_oauth_user(&self, user_data: &CreateOAuthUserData) -> Result<User>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
        async fn find_by_google_user_id(&self, google_user_id: &str) -> Result<Option<User>>;
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
        async fn update_user(&self, id: Uuid, updates: &UserUpdates) -> Result<User>;
        async fn link_google_account(&self, user_id: Uuid, google_user_id: &str, real_name: Option<String>) -> Result<()>;
        async fn update_real_name(&self, user_id: Uuid, real_name: Option<String>) -> Result<()>;
        async fn slug_exists(&self, slug: &str) -> Result<bool>;
        async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<()>;
        async fn slug_exists_excluding_user(&self, slug: &str, user_id: Uuid) -> Result<bool>;
        async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>>;
        async fn add_role_to_user(&self, user_id: Uuid, role_name: &str) -> Result<()>;
        async fn has_role(&self, user_id: Uuid, role_name: &str) -> Result<bool>;
        async fn delete_user(&self, user_id: Uuid) -> Result<()>;
        async fn update_timer_privacy(&self, user_id: Uuid, is_public: bool, show_in_list: bool) -> Result<User>;
        async fn get_users_with_public_timers(&self, limit: i64, offset: i64, search: Option<String>) -> Result<Vec<UserWithTimer>>;
        async fn get_by_slug(&self, slug: &str) -> Result<User>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::db::user::test_helpers::*;
    use mockall::predicate::eq;

    // Helper function to create test data
    fn create_test_user_data() -> CreateUserData {
        CreateUserData {
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
        }
    }

    #[tokio::test]
    async fn test_mock_create_user() {
        let mut mock_repo = MockUserRepository::new();
        let user_data = create_test_user_data();

        // Setup mock expectation - create user inside closure
        mock_repo
            .expect_create_user()
            .times(1)
            .returning(|_| Ok(build_test_user()));

        // Test the mock
        let result = mock_repo.create_user(&user_data).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.email.contains("@example.com"));
    }

    #[tokio::test]
    async fn test_mock_find_by_email() {
        let mut mock_repo = MockUserRepository::new();

        // Setup mock expectation - create user inside closure
        mock_repo
            .expect_find_by_email()
            .times(1)
            .with(eq("test@example.com"))
            .returning(|_| Ok(Some(build_test_user())));

        // Test the mock
        let result = mock_repo.find_by_email("test@example.com").await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert!(user.is_some());
        assert!(user.unwrap().email.contains("@example.com"));
    }

    #[tokio::test]
    async fn test_mock_slug_exists() {
        let mut mock_repo = MockUserRepository::new();

        // Setup mock expectation
        mock_repo
            .expect_slug_exists()
            .times(1)
            .with(eq("test-slug"))
            .returning(|_| Ok(true));

        // Test the mock
        let result = mock_repo.slug_exists("test-slug").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_error_handling() {
        let mut mock_repo = MockUserRepository::new();

        // Setup mock to return an error
        mock_repo
            .expect_find_by_email()
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Database connection failed")));

        // Test error handling
        let result = mock_repo.find_by_email("error@example.com").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Database connection failed")
        );
    }
}
