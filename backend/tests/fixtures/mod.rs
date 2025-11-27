// Test fixtures module - utilities for creating test data
//
// NOTE: UserBuilder has been moved to src/test_utils/ to be available to both
// unit tests (in src/) and integration tests (in tests/).
//
// Import with: use backend::test_utils::UserBuilder;

pub mod data_generators;
pub mod database;
pub mod jwt;
pub mod pool;
pub mod test_context;

// Re-export commonly used items
// Note: Not all exports are used in every test file, which is expected for a fixtures module
#[allow(unused_imports)]
pub use data_generators::{
    test_password_hash, unique_test_email, unique_test_phrase, unique_test_slug,
};
#[allow(unused_imports)]
pub use database::{
    add_admin_role_to_user, assign_admin_role, assign_email_verified_role, cleanup_test_db,
    create_oauth_user, create_test_refresh_token_in_db, create_unverified_user,
    create_verified_user, get_user_by_id, get_users_by_email, verify_test_database_url,
};
#[allow(unused_imports)]
pub use jwt::create_test_jwt_token;
#[allow(unused_imports)]
pub use pool::checkout;
#[allow(unused_imports)]
pub use test_context::{TestContainer, TestContext};
