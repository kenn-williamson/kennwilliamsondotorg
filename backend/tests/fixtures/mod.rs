// Test fixtures module - builders for creating test data
//
// This module provides builder patterns for constructing test data with sensible defaults.
// Only specify fields that matter for your test; the builder handles the rest.
//
// Benefits:
// - Single source of truth: Update User struct once, builders adapt automatically
// - Readable tests: `.verified().with_email("test@example.com")`
// - Discoverable API: IDE autocomplete shows all available configurations
// - Future-proof: Adding new fields doesn't break existing tests

pub mod user_builder;

pub use user_builder::UserBuilder;
