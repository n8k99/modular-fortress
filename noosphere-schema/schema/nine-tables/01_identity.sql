-- identity: WHO people and ghosts are
-- Source 1: EM Master Database - People Directory.csv (#id column)
-- Source 2: document_versions WHERE version_path LIKE '%/EM Staff/%'
-- 64 rows: id=1 Nathan, id=2-63 ghosts, id=64 T.A.S.K.S.
--
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.
-- Sparse table: many NULLs are fine. Query speed matters more than tidiness.

CREATE TABLE IF NOT EXISTS identity (

    -- -------------------------------------------------------------------------
    -- Core
    -- -------------------------------------------------------------------------
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,   -- e.g. 'NathanEckenrode' (no spaces)
    kind        TEXT NOT NULL CHECK (kind IN ('user', 'agent')),
    title       TEXT NOT NULL,          -- display name
    full_name   TEXT,                   -- full human name
    icon        TEXT,                   -- emoji icon
    type        TEXT,                   -- wikilink e.g. '[[User]]'
    lifestage   TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),

    -- AF64 runtime reference
    -- Nathan=1, ghosts=2-63, T.A.S.K.S.=64
    af64_id     INTEGER UNIQUE,

    -- -------------------------------------------------------------------------
    -- Organizational
    -- -------------------------------------------------------------------------
    archetype           TEXT,   -- wikilink e.g. '[[TheLeader]]'
    team                TEXT,   -- wikilink e.g. '[[OfficeoftheCEO]]'
    team_collaborators  TEXT,   -- wikilink list
    reports_to          TEXT,   -- wikilink
    mentor              TEXT,   -- wikilink list
    mentee              TEXT,   -- wikilink list (from CSV)
    subordinates        TEXT,   -- wikilink list (from CSV)
    earth_role          TEXT,   -- e.g. 'CEO'
    earth_department    TEXT,   -- e.g. 'Executive'
    earth_position_level TEXT,  -- e.g. 'Executive'
    department_head     TEXT,   -- wikilink
    ceo                 TEXT,   -- wikilink e.g. '[[NathanEckenrode]]'
    staff               TEXT,   -- wikilink list
    position            TEXT,   -- from CSV: 'Executive'

    -- -------------------------------------------------------------------------
    -- Background
    -- -------------------------------------------------------------------------
    birth_date          DATE,
    death_date          DATE,
    birthweek           TEXT,   -- wikilink to weekly note e.g. '[[Cusp of Energy]]'
    age                 INTEGER,
    joined_date         TEXT,   -- wikilink e.g. '[[January 1, 2023]]'
    active_period       TEXT,
    date_range          TEXT,
    date                TEXT,
    tenure              INTEGER,  -- months of service (from CSV)
    education           TEXT,
    previous_experience TEXT,
    reason_for_joining  TEXT,
    background_skills   TEXT,   -- from CSV (maps to 'skills' in YAML)

    -- -------------------------------------------------------------------------
    -- Location
    -- -------------------------------------------------------------------------
    location            TEXT,   -- wikilink
    city                TEXT,   -- real-world city (from CSV) e.g. 'Jacksonville'
    burg                TEXT,   -- wikilink to Orbis burg
    district            TEXT,   -- wikilink
    province            TEXT,   -- wikilink
    state_full          TEXT,   -- wikilink
    poi                 TEXT,   -- wikilink
    orbis_specialization TEXT,
    latitude            TEXT,   -- from CSV e.g. '30.3322° N'
    longitude           TEXT,   -- from CSV e.g. '81.6557° W'
    timezone            TEXT,   -- from CSV e.g. 'UTC-5 (Standard), UTC-4 (DST)'

    -- -------------------------------------------------------------------------
    -- Attributes
    -- -------------------------------------------------------------------------
    responsibilities        TEXT,
    current_responsibilities TEXT,
    tasks_description       TEXT,
    observation_focus       TEXT,
    assigned_infrastructure TEXT,
    strengths               TEXT,
    weaknesses              TEXT,
    growth_areas            TEXT,
    skills                  TEXT,
    hobbies                 TEXT,
    goals                   TEXT,
    confidence              TEXT,
    content_focus           TEXT,  -- from CSV (AI persona tone description)
    project                 TEXT,  -- wikilink to primary project

    -- -------------------------------------------------------------------------
    -- Worldbuilding (Orbis)
    -- -------------------------------------------------------------------------
    significance            TEXT,
    cultural_impact         TEXT,
    historical_era          TEXT,  -- wikilink
    divine_manifestations   TEXT,
    divine_persona          TEXT,
    deity_codename          TEXT,
    faction_relationships   TEXT,
    stat_body               INTEGER,
    stat_mind               INTEGER,
    stat_spirit             INTEGER,

    -- -------------------------------------------------------------------------
    -- Contact / Social
    -- -------------------------------------------------------------------------
    email           TEXT,   -- from CSV
    discord_webhook TEXT,   -- from CSV
    avatar          TEXT,   -- wikilink to the_commons kind='image' e.g. '[[NathanEckenrode-avatar]]'
                            -- the_commons image row holds file_path, avatar_url, generated_by_ghost
                            -- embed in body via wikilink — single source of truth for avatar

    -- -------------------------------------------------------------------------
    -- Personal (from CSV)
    -- -------------------------------------------------------------------------
    orientation     TEXT,
    married         TEXT,   -- 'Yes'/'No' (named married not married? for SQL safety)
    children        TEXT,   -- 'Yes'/'No'

    -- -------------------------------------------------------------------------
    -- Meta
    -- -------------------------------------------------------------------------
    aliases         TEXT,   -- stored as text with brackets preserved
    sources         TEXT,
    description     TEXT,
    status          TEXT,
    current_status  TEXT,
    last_active     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE identity IS 'WHO people and ghosts are. 64 rows: id=1 Nathan, id=2-63 ghosts, id=64 T.A.S.K.S.';
COMMENT ON COLUMN identity.lifestage IS 'Document completeness: 🌱 Seed (minimal), 🌿 Sapling (growing), 🌳 Tree (complete)';
COMMENT ON COLUMN identity.af64_id IS 'Ghost number in AF64 Lisp runtime. Nathan=1, T.A.S.K.S.=64.';
COMMENT ON COLUMN identity.slug IS 'No-space identifier matching Obsidian wikilink target e.g. NathanEckenrode';
COMMENT ON COLUMN identity.married IS 'Column named married (not married?) for SQL compatibility. Values: Yes/No';

-- -------------------------------------------------------------------------
-- Indexes
-- -------------------------------------------------------------------------
CREATE INDEX idx_identity_kind         ON identity(kind);
CREATE INDEX idx_identity_af64_id      ON identity(af64_id);
CREATE INDEX idx_identity_team         ON identity(team);
CREATE INDEX idx_identity_archetype    ON identity(archetype);
CREATE INDEX idx_identity_department   ON identity(earth_department);
CREATE INDEX idx_identity_status       ON identity(status);
CREATE INDEX idx_identity_reports_to   ON identity(reports_to);

-- Full-text search across name, description, responsibilities
CREATE INDEX idx_identity_fts ON identity USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(full_name, '') || ' ' ||
        coalesce(description, '') || ' ' ||
        coalesce(responsibilities, '') || ' ' ||
        coalesce(earth_role, '')
    )
);
