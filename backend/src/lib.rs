pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

// Test utilities - Available to both unit tests (in src/) and integration tests (in tests/)
//
// Not behind #[cfg(test)] so integration tests can use it.
// This is idiomatic Rust - test utilities can be regular modules.
// They don't affect production if not imported.
#[doc(hidden)]  // Hide from public docs but allow use in tests
pub mod test_utils;