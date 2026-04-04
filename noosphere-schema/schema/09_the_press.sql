-- the_press: Publishing output (blog articles, editorials from RSS comments, Thought Police)
-- Source: 6 existing Thought Police drafts + future publishing pipeline
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.

CREATE TABLE IF NOT EXISTS the_press (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- 'article','editorial','draft','thought_police'
    title           TEXT NOT NULL,
    body            TEXT,
    type            TEXT,
    icon            TEXT,
    lifestage       TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),
    status          TEXT,           -- 'draft','review','published','archived'

    -- Article fields
    excerpt         TEXT,
    published_date  TIMESTAMPTZ,
    published_url   TEXT,
    rss_guid        TEXT,

    -- Editorial fields (generated from RSS comments)
    source_comment  TEXT,           -- wikilink to the_post rss_comment
    source_article_url TEXT,
    generated_by_ghost TEXT,        -- wikilink to identity

    -- Publishing metadata
    featured_image  TEXT,           -- wikilink to the_commons image
    tags            TEXT,
    categories      TEXT,
    seo_description TEXT,
    seo_keywords    TEXT,
    department      TEXT,

    -- People
    assigned_to     TEXT,           -- wikilink to identity (author)
    ceo             TEXT,
    department_head TEXT,
    em_staff        TEXT,

    -- Meta
    aliases         TEXT,
    description     TEXT,
    ai_summary      TEXT,
    tasks_description TEXT,
    sources         TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_press IS 'Publishing output: blog articles, editorials, Thought Police pieces. 6 existing drafts.';

-- Indexes
CREATE INDEX idx_press_kind             ON the_press(kind);
CREATE INDEX idx_press_status           ON the_press(status);
CREATE INDEX idx_press_published_date   ON the_press(published_date);
CREATE INDEX idx_press_assigned_to      ON the_press(assigned_to);
CREATE INDEX idx_press_generated_by     ON the_press(generated_by_ghost);

CREATE INDEX idx_press_fts ON the_press USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(excerpt, '') || ' ' ||
        coalesce(seo_description, '')
    )
);
