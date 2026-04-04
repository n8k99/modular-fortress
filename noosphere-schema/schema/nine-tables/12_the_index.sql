-- the_index: Full-text search across all content
-- Unified search index: one query searches across ALL nine domain tables.
-- Triggers on domain tables update this on INSERT/UPDATE.

CREATE TABLE IF NOT EXISTS the_index (
    id              BIGSERIAL PRIMARY KEY,
    source_table    TEXT NOT NULL,    -- e.g. 'identity', 'the_work', 'the_realms'
    source_id       BIGINT NOT NULL,  -- row id in source table
    source_slug     TEXT NOT NULL,    -- slug for direct lookup
    source_kind     TEXT,             -- kind from source table
    source_title    TEXT,             -- title for display in results
    search_vector   tsvector NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_index IS 'Unified full-text search index across all nine domain tables. One query to search everything.';

-- Indexes
CREATE INDEX idx_index_search_vector    ON the_index USING GIN(search_vector);
CREATE INDEX idx_index_source           ON the_index(source_table, source_id);
CREATE INDEX idx_index_source_slug      ON the_index(source_slug);
CREATE INDEX idx_index_source_kind      ON the_index(source_kind);
CREATE UNIQUE INDEX idx_index_unique_source ON the_index(source_table, source_id);

-- Usage: SELECT source_table, source_slug, source_title,
--               ts_rank(search_vector, q) AS rank
--        FROM the_index, to_tsquery('english', 'nathan & leader') q
--        WHERE search_vector @@ q
--        ORDER BY rank DESC LIMIT 20;
