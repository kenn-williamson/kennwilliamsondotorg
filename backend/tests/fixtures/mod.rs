// Test fixtures module - utilities for creating test data
//
// NOTE: UserBuilder has been moved to src/test_utils/ to be available to both
// unit tests (in src/) and integration tests (in tests/).
//
// Import with: use backend::test_utils::UserBuilder;

pub mod test_context;
pub mod jwt;
pub mod data_generators;
pub mod database;

// Re-export commonly used items
pub use test_context::{TestContext, TestContainer};
pub use jwt::create_test_jwt_token;
pub use data_generators::{
    unique_test_email,
    unique_test_slug,
    test_password_hash,
    unique_test_phrase,
};
pub use database::{
    create_verified_user,
    create_unverified_user,
    create_oauth_user,
    get_user_by_id,
    get_users_by_email,
    add_admin_role_to_user,
    assign_admin_role,
    assign_email_verified_role,
    create_test_refresh_token_in_db,
    cleanup_test_db,
    verify_test_database_url,
};
