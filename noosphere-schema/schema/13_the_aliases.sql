-- 13_the_aliases.sql
-- Slug rename safety net. When an entry is renamed, the old slug
-- goes here so existing [[wikilinks]] don't silently break.

CREATE TABLE the_aliases (
    old_slug        TEXT PRIMARY KEY,
    new_slug        TEXT NOT NULL,
    source_table    TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_aliases IS 'Slug rename redirects. When an entry slug changes, the old slug is preserved here. Wikilink resolution checks aliases after the_index fails.';

-- Resolution fallback: if the_index has no match, check aliases
CREATE INDEX idx_aliases_new ON the_aliases(new_slug);
CREATE INDEX idx_aliases_table ON the_aliases(source_table);
