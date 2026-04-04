-- the_commons: Templates (79) + Images + Files + Assets + RSS
-- Source: document_versions where type contains Template, Image, RSS
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.
-- Massively sparse: most columns NULL for most rows. This is the trade-off.

CREATE TABLE IF NOT EXISTS the_commons (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- 'template:daily', 'template:weekly', 'template:identity', 'image', 'file', 'rss_feed', 'rss_article'
    title           TEXT NOT NULL,
    body            TEXT,
    type            TEXT,           -- wikilink
    icon            TEXT,
    lifestage       TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),
    status          TEXT,

    -- Template fields (from YAML across 79 templates)
    indexed         TIMESTAMPTZ,
    aliases         TEXT,
    tasks_description TEXT,         -- T.A.S.K.S. field
    required_fields TEXT,
    required_sections TEXT,
    requirements    TEXT,
    prompt_template TEXT,
    tone_style      TEXT,
    voice_style     TEXT,
    content_type    TEXT,
    validation      TEXT,

    -- Geographic template fields (for burg/province/state templates)
    burg            TEXT,           -- wikilink
    district        TEXT,           -- wikilink
    province        TEXT,           -- wikilink
    state_full      TEXT,           -- wikilink
    latitude        TEXT,
    longitude       TEXT,
    elevation_ft    NUMERIC,
    population      INTEGER,
    capital         TEXT,           -- wikilink
    citadel         TEXT,
    culture         TEXT,
    religion        TEXT,

    -- RPG template fields
    ac              INTEGER,
    hp              TEXT,
    cr              TEXT,
    level           INTEGER,
    class           TEXT,
    alignment       TEXT,
    race            TEXT,
    languages       TEXT,
    speed           TEXT,
    scope           TEXT,

    -- Temporal template fields
    start           DATE,
    "end"           DATE,
    week            TEXT,
    month           TEXT,
    quarter         TEXT,
    year            TEXT,
    blocking        TEXT,
    blocked_by      TEXT,
    parent          TEXT,

    -- Publishing template fields
    department      TEXT,
    education       TEXT,
    role            TEXT,
    genre           TEXT,
    founded         TEXT,
    members         TEXT,

    -- Image fields (kind='image')
    file_path       TEXT,
    file_type       TEXT,
    generated_by_ghost TEXT,        -- wikilink to identity
    openai_prompt   TEXT,
    associated_article TEXT,        -- wikilink
    associated_burg TEXT,           -- wikilink
    associated_identity TEXT,       -- wikilink
    avatar_url      TEXT,

    -- File fields (kind='file')
    original_filename TEXT,
    uploaded_by_identity TEXT,      -- wikilink
    uploaded_at     TIMESTAMPTZ,
    referenced_by   TEXT,           -- wikilink list

    -- RSS feed fields (kind='rss_feed')
    feed_url        TEXT,
    feed_title      TEXT,
    feed_category   TEXT,
    last_fetched    TIMESTAMPTZ,
    item_count      INTEGER,
    monitored_by_ghost TEXT,        -- wikilink
    monitored_since TIMESTAMPTZ,

    -- RSS article fields (kind='rss_article')
    source_feed     TEXT,           -- wikilink to rss_feed
    article_url     TEXT,
    article_published TIMESTAMPTZ,
    article_author  TEXT,
    article_summary TEXT,
    read_status     TEXT CHECK (read_status IN ('unread', 'read', 'starred')),
    flagged_by_ghost TEXT,          -- wikilink

    -- People
    ceo             TEXT,
    department_head TEXT,
    em_staff        TEXT,

    -- Meta
    tags            TEXT,
    description     TEXT,
    ai_summary      TEXT,
    sources         TEXT,
    cover_image_url TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_commons IS 'Templates, images, files, RSS feeds/articles. Massively sparse — most columns NULL for most rows.';

-- Indexes
CREATE INDEX idx_commons_kind              ON the_commons(kind);
CREATE INDEX idx_commons_status            ON the_commons(status);
CREATE INDEX idx_commons_feed_url          ON the_commons(feed_url);
CREATE INDEX idx_commons_read_status       ON the_commons(read_status);
CREATE INDEX idx_commons_article_published ON the_commons(article_published);
CREATE INDEX idx_commons_source_feed       ON the_commons(source_feed);

CREATE INDEX idx_commons_fts ON the_commons USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(description, '') || ' ' ||
        coalesce(article_summary, '')
    )
);
