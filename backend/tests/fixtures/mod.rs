// Test fixtures module - builders and utilities for creating test data
//
// This module provides builder patterns and utilities for constructing test data with sensible defaults.
// Only specify fields that matter for your test; the builder handles the rest.
//
// Benefits:
// - Single source of truth: Update structs once, builders adapt automatically
// - Readable tests: `.verified().with_email("test@example.com")`
// - Discoverable API: IDE autocomplete shows all available configurations
// - Future-proof: Adding new fields doesn't break existing tests

pub mod user_builder;
pub mod test_context;
pub mod jwt;
pub mod data_generators;
pub mod database;

// Re-export commonly used items
pub use user_builder::UserBuilder;
pub use test_context::{TestContext, TestContainer};
pub use jwt::create_test_jwt_token;
pub use data_generators::{
    unique_test_email,
    unique_test_slug,
    test_password_hash,
    unique_test_phrase,
};
pub use database::{
    create_test_user_in_db,
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
