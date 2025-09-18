-- Create incident_timers table for tracking user-specific timer resets
CREATE TABLE incident_timers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reset_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add indexes for performance
CREATE INDEX idx_incident_timers_user_id ON incident_timers(user_id);
CREATE INDEX idx_incident_timers_reset_timestamp ON incident_timers(reset_timestamp DESC);
CREATE INDEX idx_incident_timers_user_reset ON incident_timers(user_id, reset_timestamp DESC);

-- Add updated_at trigger (function already defined in users migration)
CREATE TRIGGER update_incident_timers_updated_at
    BEFORE UPDATE ON incident_timers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
