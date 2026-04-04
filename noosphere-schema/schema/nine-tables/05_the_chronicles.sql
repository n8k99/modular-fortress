-- the_chronicles: Narrative hierarchy + Historical timeline (dual hierarchies)
-- Source: document_versions with narrative/historical types
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.

CREATE TABLE IF NOT EXISTS the_chronicles (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- Narrative: 'grand_epic','epic','narrative_arc','adventure','chapter','scene'
                                   -- Historical: 'chronicle','age','epoch','era','period','event','moment'
    title           TEXT NOT NULL,
    body            TEXT,
    type            TEXT,
    icon            TEXT,
    lifestage       TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),
    status          TEXT,

    -- Narrative hierarchy
    epic            TEXT,           -- wikilink to parent epic
    arcs            TEXT,           -- wikilink list
    adventures      TEXT,           -- wikilink list
    scope           TEXT,
    timespan        TEXT,
    location        TEXT,           -- wikilink
    conflict_type   TEXT,
    assigned_to     TEXT,           -- wikilink to identity
    upstream        TEXT,           -- wikilink
    downstream      TEXT,           -- wikilink
    character       TEXT,           -- wikilink
    theme           TEXT,
    contains        TEXT,           -- wikilink list

    -- Historical hierarchy
    parent_chronicle TEXT,          -- wikilink
    parent_age      TEXT,           -- wikilink
    parent_epoch    TEXT,           -- wikilink
    parent_era      TEXT,           -- wikilink
    parent_period   TEXT,           -- wikilink
    parent_event    TEXT,           -- wikilink
    parent_scene    TEXT,           -- wikilink

    -- Age/Epoch/Era fields
    cosmic          TEXT,
    draconic        TEXT,
    cyclical        TEXT,
    technological   TEXT,
    pattern         TEXT,
    tension         TEXT,
    resolution      TEXT,
    cycle_position  TEXT,
    phase           TEXT,
    cull_relationship TEXT,
    first_cull      TEXT,
    second_cull     TEXT,
    third_cull      TEXT,
    last_cull       TEXT,
    post_cull_status TEXT,
    cull_survival   TEXT,

    -- Period fields
    political_structure TEXT,
    cultural_mood   TEXT,
    phase_type      TEXT,
    trajectory      TEXT,

    -- Event/Moment fields
    era             TEXT,           -- wikilink
    epicenter       TEXT,           -- wikilink (burg)
    scale           TEXT,
    casualties      TEXT,
    causes          TEXT,
    effects         TEXT,
    affected        TEXT,           -- wikilink list
    impact          TEXT,
    participants    TEXT,           -- wikilink list
    witnesses       TEXT,           -- wikilink list
    locations       TEXT,           -- wikilink list

    -- Temporal
    start           TEXT,
    "end"           TEXT,
    duration        TEXT,
    inception       TEXT,
    conclusion      TEXT,
    narrative_timeframe TEXT,       -- wikilink
    historical_era  TEXT,           -- wikilink
    historical_context TEXT,

    -- Meta
    confidence      TEXT,
    sources         TEXT,
    coverage        TEXT,
    aliases         TEXT,
    tags            TEXT,
    description     TEXT,
    ai_summary      TEXT,
    narrative       TEXT,
    significance    TEXT,
    cultural_impact TEXT,
    ceo             TEXT,
    department_head TEXT,
    tasks_description TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_chronicles IS 'Dual hierarchy: Narrative (epic→scene) + Historical (chronicle→moment). Moment has FOUR parents.';

-- Indexes
CREATE INDEX idx_chronicles_kind          ON the_chronicles(kind);
CREATE INDEX idx_chronicles_status        ON the_chronicles(status);
CREATE INDEX idx_chronicles_era           ON the_chronicles(era);
CREATE INDEX idx_chronicles_parent_era    ON the_chronicles(parent_era);
CREATE INDEX idx_chronicles_parent_period ON the_chronicles(parent_period);
CREATE INDEX idx_chronicles_parent_event  ON the_chronicles(parent_event);
CREATE INDEX idx_chronicles_epic          ON the_chronicles(epic);
CREATE INDEX idx_chronicles_location      ON the_chronicles(location);
CREATE INDEX idx_chronicles_assigned_to   ON the_chronicles(assigned_to);

CREATE INDEX idx_chronicles_fts ON the_chronicles USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(description, '') || ' ' ||
        coalesce(narrative, '')
    )
);
