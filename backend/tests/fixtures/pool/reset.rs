//! Database reset SQL for container recycling.

/// TRUNCATE all test data tables.
/// Tests create their own data and don't depend on seeds.
pub const RESET_SQL: &str = r#"
TRUNCATE TABLE
    user_excluded_phrases,
    phrase_suggestions,
    phrases,
    incident_timers,
    refresh_tokens,
    verification_tokens,
    password_reset_tokens,
    access_requests,
    user_roles,
    user_preferences,
    user_profiles,
    user_credentials,
    user_external_logins,
    email_suppressions,
    blog_posts,
    users,
    roles
CASCADE;
"#;
