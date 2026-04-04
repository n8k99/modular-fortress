-- 08_the_commons.sql
-- @Resources — shared substrate. No domain owns this. All domains draw from it.
-- Feeds, articles, images, research surfaces, maps, avatars, contacts, events.

CREATE TABLE the_commons (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    domain      TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_commons IS '@Resources — shared substrate. No domain owns this. All domains draw from it. Feeds, articles, images, research surfaces, maps, avatars.';

-- Kind taxonomy:
-- document         General document (47K+ from current documents table)
-- feed             RSS/Atom feed definition. meta: url, last_fetched, category
-- feed_entry       Individual RSS entry. meta: feed_slug, author, published_at, url
-- article          Fetched full article. meta: source_url, author, fetched_at
-- research_surface Research finding (Sophie Lee tags land here). meta: tags[], source,
--                  domain_relevance[] (which sovereign domains care about this)
-- image            Image asset. meta: path, dimensions, format, alt_text, generator
-- avatar           Ghost avatar image. meta: agent_id, style, path
-- map              Map asset (burg map, dungeon map, province emblem). meta: entity_ref, map_type
-- contact          External contact. meta: email, org, context, reached_out_at
-- event            Calendar/scheduled event. meta: event_date, recurrence, location
-- location         Physical location. meta: address, lat, lng, type
-- media            Generic media file. meta: mime_type, size, path
-- template         Innate expression template. meta: category, parameters[], version
-- template_history Template version history (previous body/meta snapshots)
-- archive          Archived material (immutable reference copy)
-- comment          Collected comment from external source

-- domain column: optional affinity to a sovereign domain
-- e.g., a research_surface about AI advances might have domain='the_forge'
-- but it's still in the_commons because it's shared substrate

CREATE INDEX idx_commons_kind ON the_commons(kind);
CREATE INDEX idx_commons_status ON the_commons(status);
CREATE INDEX idx_commons_domain ON the_commons(domain) WHERE domain IS NOT NULL;
CREATE INDEX idx_commons_meta ON the_commons USING gin(meta);
CREATE INDEX idx_commons_created ON the_commons(created_at);
CREATE INDEX idx_commons_updated ON the_commons(updated_at);
CREATE INDEX idx_commons_slug_trgm ON the_commons USING gin(slug gin_trgm_ops);
CREATE INDEX idx_commons_title_trgm ON the_commons USING gin(title gin_trgm_ops);

-- For backlink resolution: entries that link to a given target
CREATE INDEX idx_commons_outlinks ON the_commons USING gin((meta->'outlinks'))
    WHERE meta->'outlinks' IS NOT NULL;

CREATE TRIGGER trg_commons_updated_at
    BEFORE UPDATE ON the_commons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
