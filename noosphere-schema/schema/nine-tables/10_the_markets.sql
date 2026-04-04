-- the_markets: Trading positions + Research feed accumulation
-- Source: 0 existing entries (not yet implemented) — schema ready for future use
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.

CREATE TABLE IF NOT EXISTS the_markets (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- 'position','feed','research','watchlist'
    title           TEXT NOT NULL,
    body            TEXT,
    type            TEXT,
    icon            TEXT,
    status          TEXT,

    -- Position fields (kind='position')
    position_id     TEXT,
    instrument      TEXT,           -- ticker/symbol
    units           NUMERIC,
    entry_price     NUMERIC,
    exit_price      NUMERIC,
    profit_loss     NUMERIC,
    opened_at       TIMESTAMPTZ,
    closed_at       TIMESTAMPTZ,
    strategy        TEXT,

    -- Feed fields (kind='feed')
    feed_url        TEXT,
    feed_title      TEXT,
    feed_category   TEXT,
    last_fetched    TIMESTAMPTZ,
    item_count      INTEGER,
    monitored_by_ghost TEXT,        -- wikilink to identity

    -- Research fields (kind='research')
    source_feed     TEXT,           -- wikilink to feed
    item_url        TEXT,
    item_title      TEXT,
    item_summary    TEXT,
    flagged_by_ghost TEXT,          -- wikilink to identity
    related_position TEXT,          -- wikilink to position
    insight         TEXT,

    -- People
    assigned_to     TEXT,           -- wikilink
    ceo             TEXT,
    department_head TEXT,

    -- Meta
    aliases         TEXT,
    tags            TEXT,
    description     TEXT,
    ai_summary      TEXT,
    tasks_description TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_markets IS 'Trading positions + research feed accumulation. 0 existing entries — schema ready for Complete Success commerce domain.';

-- Indexes
CREATE INDEX idx_markets_kind           ON the_markets(kind);
CREATE INDEX idx_markets_status         ON the_markets(status);
CREATE INDEX idx_markets_instrument     ON the_markets(instrument);
CREATE INDEX idx_markets_opened_at      ON the_markets(opened_at);
CREATE INDEX idx_markets_feed_url       ON the_markets(feed_url);
CREATE INDEX idx_markets_flagged_by     ON the_markets(flagged_by_ghost);
