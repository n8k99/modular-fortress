-- 02_the_chronicles.sql
-- Narrative canon, historical ages, story arcs
-- Immutable once status = 'canon'

CREATE TABLE the_chronicles (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'draft',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_chronicles IS 'Grand Epics & Master Chronicles — narrative canon, historical ages, story arcs. Immutable once canon is set.';

-- Kind taxonomy:
-- age          Historical age (e.g., Age of Expansion)
-- era          Sub-period within an age
-- arc          Narrative story arc
-- event        Canon historical event
-- prophecy     Prophecy or prediction
-- law          Canon law or decree
-- myth         Mythological narrative
-- cosmology    Creation/cosmological canon
-- canon_doc    General canon document

CREATE INDEX idx_chronicles_kind ON the_chronicles(kind);
CREATE INDEX idx_chronicles_status ON the_chronicles(status);
CREATE INDEX idx_chronicles_meta ON the_chronicles USING gin(meta);
CREATE INDEX idx_chronicles_created ON the_chronicles(created_at);
CREATE INDEX idx_chronicles_slug_trgm ON the_chronicles USING gin(slug gin_trgm_ops);
CREATE INDEX idx_chronicles_title_trgm ON the_chronicles USING gin(title gin_trgm_ops);

-- Auto-update timestamp
CREATE TRIGGER trg_chronicles_updated_at
    BEFORE UPDATE ON the_chronicles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- Canon immutability: entries with status='canon' cannot have body/title modified
-- Must set status to 'retcon' first
CREATE OR REPLACE FUNCTION enforce_canon_immutability()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.status = 'canon' AND NEW.status != 'retcon' THEN
        IF OLD.body IS DISTINCT FROM NEW.body OR OLD.title IS DISTINCT FROM NEW.title THEN
            RAISE EXCEPTION 'Canon entries are immutable. Set status to retcon first.';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_canon_immutability
    BEFORE UPDATE ON the_chronicles
    FOR EACH ROW EXECUTE FUNCTION enforce_canon_immutability();
