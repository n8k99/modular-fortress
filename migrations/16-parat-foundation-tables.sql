-- PARAT Foundation Tables Migration
-- Phase 16, Plan 01: Areas, Archives, Resources, Templates
-- Executed against master_chronicle

BEGIN;

-- ============================================================
-- AREAS TABLE (D-01, D-02, SCHEMA-01)
-- ============================================================

CREATE TABLE areas (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    slug VARCHAR(256) NOT NULL UNIQUE,
    description TEXT,
    owner VARCHAR(64) REFERENCES agents(id),
    status VARCHAR(32) DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'archived')),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_areas_owner ON areas(owner);
CREATE INDEX idx_areas_status ON areas(status);

CREATE TRIGGER update_areas_updated_at
    BEFORE UPDATE ON areas
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Seed data (D-01, D-02)
INSERT INTO areas (name, slug, description, owner, status) VALUES
    ('EM Corp', 'em-corp', 'Eckenrode Muziekopname corporate operations', 'kathryn', 'active'),
    ('Orbis', 'orbis', 'Orbis worldbuilding and TTRPG content', 'sylvia', 'active'),
    ('Living Room Music', 'living-room-music', 'Music production and composition', 'lrm', 'active'),
    ('N8K99/Personal', 'n8k99-personal', 'Personal projects and content', 'nathan', 'active'),
    ('Infrastructure/Systems', 'infrastructure-systems', 'Technical infrastructure and platform systems', 'nova', 'active');

-- ============================================================
-- ARCHIVES TABLE (D-03, D-04, SCHEMA-02)
-- ============================================================

CREATE TABLE archives (
    id SERIAL PRIMARY KEY,
    title VARCHAR(512),
    content TEXT,
    source_type VARCHAR(128) NOT NULL,
    source_id INTEGER,
    original_path TEXT,
    period_start DATE,
    period_end DATE,
    topic VARCHAR(256),
    tags JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
    -- No updated_at: archives are immutable
);

CREATE INDEX idx_archives_source_type ON archives(source_type);
CREATE INDEX idx_archives_source ON archives(source_type, source_id);
CREATE INDEX idx_archives_period ON archives(period_start, period_end);
CREATE INDEX idx_archives_topic ON archives(topic);

-- Full-text search (SCHEMA-02)
ALTER TABLE archives ADD COLUMN tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector('english', coalesce(title, '') || ' ' || coalesce(content, ''))
    ) STORED;
CREATE INDEX idx_archives_fts ON archives USING gin(tsv);

-- Immutability trigger (D-03)
CREATE OR REPLACE FUNCTION enforce_archive_immutability() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.content IS DISTINCT FROM NEW.content
       OR OLD.title IS DISTINCT FROM NEW.title
       OR OLD.source_type IS DISTINCT FROM NEW.source_type
       OR OLD.source_id IS DISTINCT FROM NEW.source_id
       OR OLD.original_path IS DISTINCT FROM NEW.original_path
       OR OLD.period_start IS DISTINCT FROM NEW.period_start
       OR OLD.period_end IS DISTINCT FROM NEW.period_end THEN
        RAISE EXCEPTION 'Archives are immutable: content fields cannot be updated';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_archive_immutability
    BEFORE UPDATE ON archives
    FOR EACH ROW EXECUTE FUNCTION enforce_archive_immutability();

-- ============================================================
-- RESOURCES TABLE (D-05, D-06, SCHEMA-03)
-- ============================================================

CREATE TABLE resources (
    id SERIAL PRIMARY KEY,
    name VARCHAR(512) NOT NULL,
    slug VARCHAR(512) NOT NULL UNIQUE,
    resource_type VARCHAR(64) NOT NULL,
    source_type VARCHAR(64) NOT NULL,
    source_id INTEGER NOT NULL,
    description TEXT,
    tags JSONB DEFAULT '[]',
    frozen BOOLEAN DEFAULT FALSE,
    area_id INTEGER REFERENCES areas(id),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_resources_type ON resources(resource_type);
CREATE INDEX idx_resources_source ON resources(source_type, source_id);
CREATE INDEX idx_resources_area ON resources(area_id);
CREATE INDEX idx_resources_frozen ON resources(frozen) WHERE frozen = true;

CREATE TRIGGER update_resources_updated_at
    BEFORE UPDATE ON resources
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Frozen enforcement trigger (D-05)
CREATE OR REPLACE FUNCTION enforce_resource_frozen() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.frozen = true THEN
        RAISE EXCEPTION 'Resource is frozen and cannot be updated';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_resource_frozen
    BEFORE UPDATE ON resources
    FOR EACH ROW EXECUTE FUNCTION enforce_resource_frozen();

-- ============================================================
-- TEMPLATES TABLE (D-07, D-08, D-09, SCHEMA-04)
-- ============================================================

CREATE TABLE templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    slug VARCHAR(256) NOT NULL UNIQUE,
    category VARCHAR(128),
    description TEXT,
    body TEXT NOT NULL,
    parameters JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    version INTEGER DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_templates_category ON templates(category);
CREATE INDEX idx_templates_slug ON templates(slug);

CREATE TRIGGER update_templates_updated_at
    BEFORE UPDATE ON templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Templates history table (D-07)
CREATE TABLE templates_history (
    id SERIAL PRIMARY KEY,
    template_id INTEGER NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    body TEXT NOT NULL,
    parameters JSONB DEFAULT '[]',
    changed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_templates_history_template ON templates_history(template_id);
CREATE INDEX idx_templates_history_version ON templates_history(template_id, version);

-- Version history trigger (D-08)
CREATE OR REPLACE FUNCTION track_template_version() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.body IS DISTINCT FROM NEW.body THEN
        INSERT INTO templates_history (template_id, version, body, parameters, changed_at)
        VALUES (OLD.id, OLD.version, OLD.body, OLD.parameters, NOW());
        NEW.version := OLD.version + 1;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_template_version_history
    BEFORE UPDATE ON templates
    FOR EACH ROW EXECUTE FUNCTION track_template_version();

COMMIT;
