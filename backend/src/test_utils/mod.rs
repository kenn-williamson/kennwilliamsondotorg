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
pub mod incident_timer_builder;
pub mod phrase_builder;
pub mod refresh_token_builder;
pub mod access_request_builder;

// Re-export commonly used test builders
pub use user_builder::UserBuilder;
pub use incident_timer_builder::IncidentTimerBuilder;
pub use phrase_builder::{PhraseBuilder, PhraseSuggestionBuilder};
pub use refresh_token_builder::RefreshTokenBuilder;
pub use access_request_builder::AccessRequestBuilder;
