-- temporal: WHAT happened each day/week/month/quarter/year
-- Source: Daily/Weekly/Monthly/Quarterly/Yearly notes from Obsidian vault
-- Special: 64 memory columns (one per identity)
--
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.
-- Sparse table: many NULLs are fine. Query speed matters more than tidiness.

CREATE TABLE IF NOT EXISTS temporal (

    -- -------------------------------------------------------------------------
    -- Core
    -- -------------------------------------------------------------------------
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,   -- e.g. '2024-04-03' or '2024-W14' or '2024-Q2'
    kind        TEXT NOT NULL CHECK (kind IN ('daily', 'weekly', 'monthly', 'quarterly', 'yearly')),
    title       TEXT NOT NULL,
    type        TEXT,           -- wikilink e.g. '[[Daily Note]]', '[[Weekly Note]]'
    icon        TEXT,           -- emoji icon
    lifestage   TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),

    -- -------------------------------------------------------------------------
    -- Temporal metadata
    -- -------------------------------------------------------------------------
    indexed             TIMESTAMPTZ,    -- the date/time this note represents
    day                 TEXT,           -- day of week or day reference
    start               DATE,           -- period start date (weekly/monthly/quarterly)
    "end"               DATE,           -- period end date (weekly/monthly/quarterly)
    week                TEXT,           -- wikilink to weekly note e.g. '[[2024-W14]]'
    month               TEXT,           -- wikilink to monthly note
    quarter             TEXT,           -- Q1, Q2, Q3, Q4
    year                TEXT,           -- year reference
    yearly_note         TEXT,           -- wikilink to yearly note e.g. '[[2024 Yearly Note]]'

    -- -------------------------------------------------------------------------
    -- Temporal chains (day-to-day / week-to-week linking)
    -- -------------------------------------------------------------------------
    blocking            TEXT,           -- wikilink to next period (today → tomorrow)
    blocked_by          TEXT,           -- wikilink to previous period (yesterday ← today)

    -- -------------------------------------------------------------------------
    -- Context
    -- -------------------------------------------------------------------------
    narrative_timeframe TEXT,           -- wikilink e.g. '[[5YearRoadmap]]'
    executive_ghost     TEXT,           -- wikilink to ghost managing this period
    ceo                 TEXT,           -- wikilink e.g. '[[NathanEckenrode]]'
    department_head     TEXT,           -- wikilink
    em_staff            TEXT,           -- wikilink list of relevant staff

    -- -------------------------------------------------------------------------
    -- Content
    -- -------------------------------------------------------------------------
    body                TEXT,           -- template body with ghost instructions
    tasks_description   TEXT,           -- T.A.S.K.S. field: AI-generated description/instructions
    objectives          TEXT,           -- key focus areas for the period
    outcomes            TEXT,           -- what actually happened
    ai_summary          TEXT,           -- AI-generated summary of the period
    cryptic_summary     TEXT,           -- cryptic/poetic summary

    -- -------------------------------------------------------------------------
    -- Meta
    -- -------------------------------------------------------------------------
    aliases             TEXT,
    tags                TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ DEFAULT NOW(),

    -- -------------------------------------------------------------------------
    -- 64 Memory columns (one per identity)
    -- Each identity gets their own column for what they observed/did this period.
    -- Daily memories roll up → Weekly → Monthly → Yearly (temporal compression).
    -- This IS the memory system. Not a separate table.
    -- -------------------------------------------------------------------------
    identity_1_memory   TEXT,   -- Nathan Eckenrode
    identity_2_memory   TEXT,
    identity_3_memory   TEXT,
    identity_4_memory   TEXT,
    identity_5_memory   TEXT,
    identity_6_memory   TEXT,
    identity_7_memory   TEXT,
    identity_8_memory   TEXT,
    identity_9_memory   TEXT,
    identity_10_memory  TEXT,
    identity_11_memory  TEXT,
    identity_12_memory  TEXT,
    identity_13_memory  TEXT,
    identity_14_memory  TEXT,
    identity_15_memory  TEXT,
    identity_16_memory  TEXT,
    identity_17_memory  TEXT,
    identity_18_memory  TEXT,
    identity_19_memory  TEXT,
    identity_20_memory  TEXT,
    identity_21_memory  TEXT,
    identity_22_memory  TEXT,
    identity_23_memory  TEXT,
    identity_24_memory  TEXT,
    identity_25_memory  TEXT,
    identity_26_memory  TEXT,
    identity_27_memory  TEXT,
    identity_28_memory  TEXT,
    identity_29_memory  TEXT,
    identity_30_memory  TEXT,
    identity_31_memory  TEXT,
    identity_32_memory  TEXT,
    identity_33_memory  TEXT,
    identity_34_memory  TEXT,
    identity_35_memory  TEXT,
    identity_36_memory  TEXT,
    identity_37_memory  TEXT,
    identity_38_memory  TEXT,
    identity_39_memory  TEXT,
    identity_40_memory  TEXT,
    identity_41_memory  TEXT,
    identity_42_memory  TEXT,
    identity_43_memory  TEXT,
    identity_44_memory  TEXT,
    identity_45_memory  TEXT,
    identity_46_memory  TEXT,
    identity_47_memory  TEXT,
    identity_48_memory  TEXT,
    identity_49_memory  TEXT,
    identity_50_memory  TEXT,
    identity_51_memory  TEXT,
    identity_52_memory  TEXT,
    identity_53_memory  TEXT,
    identity_54_memory  TEXT,
    identity_55_memory  TEXT,
    identity_56_memory  TEXT,
    identity_57_memory  TEXT,
    identity_58_memory  TEXT,
    identity_59_memory  TEXT,
    identity_60_memory  TEXT,
    identity_61_memory  TEXT,
    identity_62_memory  TEXT,
    identity_63_memory  TEXT,
    identity_64_memory  TEXT    -- T.A.S.K.S.
);

COMMENT ON TABLE temporal IS 'WHAT happened each period. 64 identity memory columns for temporal compression (daily→weekly→monthly→yearly).';
COMMENT ON COLUMN temporal.slug IS 'Period identifier: 2024-04-03 (daily), 2024-W14 (weekly), 2024-04 (monthly), 2024-Q2 (quarterly), 2024 (yearly)';
COMMENT ON COLUMN temporal.tasks_description IS 'T.A.S.K.S. field from YAML — AI-generated period description and ghost instructions';
COMMENT ON COLUMN temporal.blocking IS 'Wikilink to NEXT period (today→tomorrow, this week→next week). Temporal chain forward.';
COMMENT ON COLUMN temporal.blocked_by IS 'Wikilink to PREVIOUS period (today←yesterday). Temporal chain backward.';
COMMENT ON COLUMN temporal.identity_1_memory IS 'Nathan memory column. Each identity_N_memory stores what identity N observed/did during this period.';
COMMENT ON COLUMN temporal.identity_64_memory IS 'T.A.S.K.S. memory column.';

-- -------------------------------------------------------------------------
-- Indexes
-- -------------------------------------------------------------------------
CREATE INDEX idx_temporal_kind           ON temporal(kind);
CREATE INDEX idx_temporal_indexed        ON temporal(indexed);
CREATE INDEX idx_temporal_start          ON temporal(start);
CREATE INDEX idx_temporal_quarter        ON temporal(quarter);
CREATE INDEX idx_temporal_year           ON temporal(year);
CREATE INDEX idx_temporal_blocking       ON temporal(blocking);
CREATE INDEX idx_temporal_blocked_by     ON temporal(blocked_by);

-- Composite: find all dailies in a date range
CREATE INDEX idx_temporal_kind_indexed   ON temporal(kind, indexed);

-- Full-text search across title, body, objectives, outcomes
CREATE INDEX idx_temporal_fts ON temporal USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(objectives, '') || ' ' ||
        coalesce(outcomes, '') || ' ' ||
        coalesce(ai_summary, '')
    )
);
