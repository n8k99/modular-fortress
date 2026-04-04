-- the_work: Project → Phase → Goal → Task hierarchy
-- Source: document_versions where type contains Project, Phase, Goal, Task
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.
-- Sparse table: tasks won't have phase_number, projects won't have due_date. NULL is fine.

CREATE TABLE IF NOT EXISTS the_work (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL CHECK (kind IN ('project', 'phase', 'goal', 'task', 'decision', 'routine', 'issue')),
    title           TEXT NOT NULL,
    body            TEXT,
    type            TEXT,           -- wikilink e.g. '[[Project]]'
    icon            TEXT,
    lifestage       TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),
    status          TEXT,

    -- Hierarchy (wikilinks — sparse by kind)
    parent_project  TEXT,           -- wikilink to parent project
    parent_phase    TEXT,           -- wikilink to parent phase
    parent_goal     TEXT,           -- wikilink to parent goal
    parent          TEXT,           -- generic parent wikilink

    -- Chains
    blocking        TEXT,           -- wikilink to what this blocks
    blocked_by      TEXT,           -- wikilink to what blocks this

    -- Project-specific
    owner           TEXT,           -- wikilink to identity
    schedule        TEXT,
    area_slug       TEXT,           -- wikilink to area
    current_context TEXT,
    related         TEXT,           -- wikilink list of related items

    -- Phase-specific
    phase_number    INTEGER,
    phase_description TEXT,

    -- Goal-specific
    success_criteria TEXT,
    measurement     TEXT,

    -- Task-specific
    due_date        DATE,
    assigned_to     TEXT,           -- wikilink to identity
    priority        TEXT,
    estimated_hours NUMERIC,
    tasks_description TEXT,         -- T.A.S.K.S. field

    -- Decision-specific
    decision_date   DATE,
    decision_outcome TEXT,

    -- People
    ceo             TEXT,           -- wikilink
    department_head TEXT,           -- wikilink
    em_staff        TEXT,           -- wikilink list

    -- Meta
    aliases         TEXT,
    tags            TEXT,
    description     TEXT,
    ai_summary      TEXT,
    narrative_timeframe TEXT,       -- wikilink
    sources         TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_work IS 'Project/Phase/Goal/Task hierarchy. Sparse columns: tasks have due_date, projects have owner, etc.';

-- Indexes
CREATE INDEX idx_work_kind            ON the_work(kind);
CREATE INDEX idx_work_status          ON the_work(status);
CREATE INDEX idx_work_parent_project  ON the_work(parent_project);
CREATE INDEX idx_work_parent_phase    ON the_work(parent_phase);
CREATE INDEX idx_work_parent_goal     ON the_work(parent_goal);
CREATE INDEX idx_work_assigned_to     ON the_work(assigned_to);
CREATE INDEX idx_work_due_date        ON the_work(due_date);
CREATE INDEX idx_work_owner           ON the_work(owner);
CREATE INDEX idx_work_priority        ON the_work(priority);

CREATE INDEX idx_work_fts ON the_work USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(description, '') || ' ' ||
        coalesce(success_criteria, '')
    )
);
