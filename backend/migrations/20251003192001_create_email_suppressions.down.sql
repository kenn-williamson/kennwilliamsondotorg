-- Rollback email suppressions table

DROP TABLE IF EXISTS password_reset_tokens;
DROP TABLE IF EXISTS email_suppressions;
