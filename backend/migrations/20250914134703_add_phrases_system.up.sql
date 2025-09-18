-- Add phrases system with exclusion-based user preferences
-- Creates system user and all phrase-related tables with initial data from Sayings.json

-- Create system user for initial phrase creation (standard practice)
INSERT INTO users (email, password_hash, display_name, slug)
SELECT 'system@kennwilliamson.org',
       '$2b$12$system.hash.placeholder',
       'System',
       'system'
WHERE NOT EXISTS (SELECT 1 FROM users WHERE email = 'system@kennwilliamson.org');

-- Create phrases table for storing motivational phrases
CREATE TABLE phrases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    phrase_text VARCHAR(500) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_phrases_active ON phrases(active);
CREATE INDEX idx_phrases_created_by ON phrases(created_by);
CREATE TRIGGER update_phrases_updated_at
    BEFORE UPDATE ON phrases
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Create user_excluded_phrases table (stores phrases users DON'T want to see)
-- Default behavior: users see ALL active phrases EXCEPT those in this table
CREATE TABLE user_excluded_phrases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    phrase_id UUID NOT NULL REFERENCES phrases(id) ON DELETE CASCADE,
    excluded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, phrase_id)
);

CREATE INDEX idx_user_excluded_phrases_user_id ON user_excluded_phrases(user_id);
CREATE INDEX idx_user_excluded_phrases_phrase_id ON user_excluded_phrases(phrase_id);

-- Create phrase_suggestions table for user suggestion workflow
CREATE TABLE phrase_suggestions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    phrase_text VARCHAR(500) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
    admin_id UUID REFERENCES users(id) ON DELETE SET NULL,
    admin_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_phrase_suggestions_user_id ON phrase_suggestions(user_id);
CREATE INDEX idx_phrase_suggestions_status ON phrase_suggestions(status);
CREATE INDEX idx_phrase_suggestions_admin_id ON phrase_suggestions(admin_id);
CREATE TRIGGER update_phrase_suggestions_updated_at
    BEFORE UPDATE ON phrase_suggestions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert all 21 phrases from Sayings.json
INSERT INTO phrases (phrase_text, created_by)
SELECT phrase_data.phrase, (SELECT id FROM users WHERE email = 'system@kennwilliamson.org' LIMIT 1)
FROM (VALUES
    ('The Watch Continues. The Walls Must Hold.'),
    ('By Grace you stand; by vigilance you endure.'),
    ('A Chain is Forged One Link at a Time. Keep it Unbroken.'),
    ('Each Day Forges a Stronger Will. Temper it with Awareness.'),
    ('This Path is a Razor''s Edge. Walk with Purpose and a Steady Soul.'),
    ('The Spirit is willing, but the flesh is weak. Clothe yourself in vigilance.'),
    ('Complacency is the Serpent in the Garden. Guard the Gates.'),
    ('This number is a monument to your resolve. Protect its foundation.'),
    ('A Garden Untended Grows to Weed. Tend the Soul with Prayer and Watchfulness.'),
    ('Navigating a Steady Course. Be Wary of the Coming Tides.'),
    ('The Beast is Caged, Not Slain. Do Not Forget the Strength of its Bars.'),
    ('The Higher the Ascent, the Sheerer the Ledge. Keep Your Gaze Fixed on the Summit.'),
    ('Kept by Grace, Tested by the World. Stay Close to the Shepherd.'),
    ('This Quiet is a Hard-Won Peace. Do Not Mistake it for a Truce.'),
    ('Refined by the Fire of Trial. May Your Resolve be as Pure as Gold.'),
    ('The Past is a Shoreline Behind You. The Ocean Ahead is Vast and Unseen.'),
    ('A Fortress of Habit Stands One Stone at a Time. Lay Today''s Stone with Care.'),
    ('The Light Holds, but Shadows Lengthen. Stay Rooted in your Conviction.'),
    ('Let This Streak be a Testament, Not a Trophy. A Trophy Gathers Dust; a Testament Must be Lived.'),
    ('Strength is a Gift, Not a Given. Honor it with Continued Discipline.')
) AS phrase_data(phrase);
