-- the_music: Podcast episodes with temporal chains + musicology
-- Source: 849 episode documents
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.

CREATE TABLE IF NOT EXISTS the_music (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- 'episode', 'track', 'album', 'artist', 'playlist'
    title           TEXT NOT NULL,
    body            TEXT,
    type            TEXT,
    icon            TEXT,
    lifestage       TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),
    status          TEXT,

    -- Episode fields
    episode_published_date TIMESTAMPTZ,
    episode_description TEXT,
    narrative_timeframe TEXT,       -- wikilink
    podcast         TEXT,           -- podcast name/series
    audio_link      TEXT,
    audio_file      TEXT,
    cover_image_url TEXT,

    -- Temporal chains
    blocking        TEXT,           -- wikilink to next episode
    blocked_by      TEXT,           -- wikilink to previous episode

    -- Production
    objectives      TEXT,
    outcomes        TEXT,
    ai_summary      TEXT,
    tags            TEXT,

    -- People
    em_staff        TEXT,           -- wikilink list
    department_head TEXT,           -- wikilink
    ceo             TEXT,           -- wikilink
    assigned_to     TEXT,           -- wikilink

    -- Musicology (kind='track','album','artist')
    genre           TEXT,
    artist          TEXT,
    album           TEXT,
    track           TEXT,
    founded         TEXT,
    members         TEXT,
    artist_status   TEXT,

    -- Meta
    aliases         TEXT,
    description     TEXT,
    sources         TEXT,
    tasks_description TEXT,
    campaign_link   TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_music IS 'Podcast episodes with temporal chains + musicology catalog. 849 episodes.';

-- Indexes
CREATE INDEX idx_music_kind                ON the_music(kind);
CREATE INDEX idx_music_status              ON the_music(status);
CREATE INDEX idx_music_episode_pub_date    ON the_music(episode_published_date);
CREATE INDEX idx_music_podcast             ON the_music(podcast);
CREATE INDEX idx_music_genre               ON the_music(genre);
CREATE INDEX idx_music_artist              ON the_music(artist);
CREATE INDEX idx_music_blocking            ON the_music(blocking);

CREATE INDEX idx_music_fts ON the_music USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(episode_description, '') || ' ' ||
        coalesce(ai_summary, '')
    )
);
