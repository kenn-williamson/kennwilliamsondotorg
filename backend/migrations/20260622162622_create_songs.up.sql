-- Create songs table for the music section
-- Mirrors the blog_posts conventions: uuidv7 PK, slug, draft/published status
-- workflow, full-text search vector, and updated_at trigger.
CREATE TABLE songs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,           -- Short blurb / liner notes
    lyrics TEXT,               -- Full lyrics (plain text or markdown)
    credits TEXT,              -- Credits / AI disclosure (lyrics, vocals, AI tools used)
    audio_url TEXT,            -- S3 URL of the streaming audio file
    artwork_url TEXT,          -- S3 URL of the cover art
    duration_seconds INTEGER,  -- Track length, computed server-side on upload
    display_order INTEGER NOT NULL DEFAULT 0,  -- Manual ordering (ascending)
    status VARCHAR(20) NOT NULL DEFAULT 'draft',  -- 'draft' | 'published'
    published_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    -- Full-text search across title, description, and lyrics
    search_vector tsvector GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(description, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(lyrics, '')), 'C')
    ) STORED
);

-- Indexes for performance
CREATE INDEX idx_songs_status ON songs(status);
CREATE INDEX idx_songs_published_at ON songs(published_at DESC);
CREATE INDEX idx_songs_slug ON songs(slug);
CREATE INDEX idx_songs_display_order ON songs(display_order);
CREATE INDEX idx_songs_search ON songs USING GIN(search_vector);

-- Trigger for updated_at (reuse existing function)
CREATE TRIGGER update_songs_updated_at
    BEFORE UPDATE ON songs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
