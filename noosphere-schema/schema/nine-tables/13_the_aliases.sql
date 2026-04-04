-- the_aliases: Slug redirects for renamed pages
-- When a page is renamed, old slug → new slug redirect.
-- Enables: wikilinks keep working after renames.

CREATE TABLE IF NOT EXISTS the_aliases (
    id              BIGSERIAL PRIMARY KEY,
    alias_slug      TEXT NOT NULL UNIQUE,  -- the old/alternate slug
    canonical_slug  TEXT NOT NULL,          -- the current correct slug
    source_table    TEXT NOT NULL,          -- which domain table this belongs to
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_aliases IS 'Slug redirects for renamed pages. Old wikilinks keep working after renames.';

-- Indexes
CREATE INDEX idx_aliases_canonical      ON the_aliases(canonical_slug);
CREATE INDEX idx_aliases_source_table   ON the_aliases(source_table);

-- Usage: When resolving [[SomeOldName]], check the_aliases first:
-- SELECT canonical_slug, source_table FROM the_aliases WHERE alias_slug = 'SomeOldName';
