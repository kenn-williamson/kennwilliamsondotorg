// Test utilities module - only compiled during tests
//
// This module provides builder patterns for test fixtures with sensible defaults.
// These are TEST-ONLY builders, separate from any production builders.
//
// Idiomatic Rust approach:
// - Production builders (if needed): enforce required fields, no defaults
// - Test builders (here): sensible defaults, override what matters for the test
//
// This pattern is used by major Rust projects like Cargo.

pub mod user_builder;

// Re-export commonly used test builders
pub use user_builder::UserBuilder;
