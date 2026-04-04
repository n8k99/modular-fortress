-- 11_the_links.sql
-- Wikilink relationship index — trigger-maintained backlink table.
-- Replaces the 397K-row document_edges table.
-- Every [[wikilink]] in body or meta across all 9 tables is tracked here.

CREATE TABLE the_links (
    id              BIGSERIAL PRIMARY KEY,
    source_table    TEXT NOT NULL,
    source_id       BIGINT NOT NULL,
    source_slug     TEXT NOT NULL,
    target_slug     TEXT NOT NULL,
    target_table    TEXT,           -- NULL if unresolved
    target_id       BIGINT,         -- NULL if unresolved
    link_context    TEXT NOT NULL,   -- 'body' or the meta field name
    qualifier       TEXT,           -- 'forge', 'realms', etc. from [[forge:Nova]]
    display_text    TEXT,           -- alias from [[Target|Display]]
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_links IS 'Wikilink relationship index. Trigger-maintained from all 9 domain tables. Replaces document_edges. Every [[wikilink]] tracked for instant backlink queries.';

-- Forward lookups: "what does entry X link to?"
CREATE INDEX idx_links_source ON the_links(source_table, source_id);
CREATE INDEX idx_links_source_slug ON the_links(source_slug);

-- Backward lookups: "what links TO entry X?" (the critical backlink query)
CREATE INDEX idx_links_target ON the_links(target_table, target_id)
    WHERE target_id IS NOT NULL;
CREATE INDEX idx_links_target_slug ON the_links(target_slug);

-- Unresolved links: for reporting broken [[wikilinks]]
CREATE INDEX idx_links_unresolved ON the_links(target_slug)
    WHERE target_id IS NULL;

-- Context filtering: "what links to X from meta.reports_to?"
CREATE INDEX idx_links_context ON the_links(link_context);
