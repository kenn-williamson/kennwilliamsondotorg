pub mod events;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

// Test utilities - Available to both unit tests (in src/) and integration tests (in tests/)
// Gated behind the mocks feature to prevent compilation in production
#[cfg(any(test, feature = "mocks"))]
pub mod test_utils;