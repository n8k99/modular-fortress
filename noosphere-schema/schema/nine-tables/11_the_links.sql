-- the_links: Automatic wikilink tracking via triggers
-- Every [[wikilink]] in every domain table gets extracted and tracked here.
-- Enables: backlinks, graph view, "what references this slug?"

CREATE TABLE IF NOT EXISTS the_links (
    id              BIGSERIAL PRIMARY KEY,
    source_table    TEXT NOT NULL,   -- e.g. 'identity', 'temporal', 'the_work'
    source_id       BIGINT NOT NULL, -- row id in source table
    source_field    TEXT NOT NULL,   -- which column contained the wikilink
    target_slug     TEXT NOT NULL,   -- the slug inside the [[brackets]]
    link_text       TEXT,            -- display text if [[slug|display text]]
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_links IS 'Automatic wikilink tracking. Every [[wikilink]] in every domain table extracted here via triggers.';

-- Indexes for common queries
CREATE INDEX idx_links_source           ON the_links(source_table, source_id);
CREATE INDEX idx_links_target_slug      ON the_links(target_slug);
CREATE INDEX idx_links_source_field     ON the_links(source_table, source_field);

-- "What links TO this slug?" (backlinks)
-- SELECT * FROM the_links WHERE target_slug = 'NathanEckenrode';

-- "What does this row link TO?" (outlinks)
-- SELECT * FROM the_links WHERE source_table = 'identity' AND source_id = 1;
