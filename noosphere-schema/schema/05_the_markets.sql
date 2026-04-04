-- 05_the_markets.sql
-- Complete Success — financial pipeline, OANDA, Kalshi, trading

CREATE TABLE the_markets (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_markets IS 'Complete Success — financial pipeline. OANDA forex, Kalshi prediction markets. Fires 1hr before London/New York/Tokyo opens. Hard constraints: no stoploss = no order, max 20% bankroll, paper first.';

-- Kind taxonomy:
-- paper_trade      Forex paper trade entry
-- position         Open/closed position
-- position_event   Position lifecycle event (open, modify, close)
-- order            Trade order
-- signal           Market signal (technical/sentiment)
-- alert            Market alert trigger
-- sentiment        Sentiment score snapshot
-- probability      Probability score (Kalshi)
-- news_score       Forex news impact score
-- snapshot         Market data snapshot (kalshi or forex)
-- watchlist        Market watchlist definition
-- thesis           Trading thesis document
-- briefing         Market briefing (surf report)
-- journal_entry    Trade journal entry
-- trade_log        Individual trade execution log
-- fitness          Forex fitness snapshot

-- meta for paper_trade:
--   pair, direction, entry_price, exit_price, stoploss, takeprofit
--   risk_pct, result, pips, session (london/newyork/tokyo)

-- meta for signal:
--   symbol/pair, signal_type, strength, timeframe, source_agent

-- Hard constraints (enforced at application layer, documented here):
-- 1. No stoploss → no order. Structural.
-- 2. Max 20% bankroll per position.
-- 3. Paper first. Real money after proven.

CREATE INDEX idx_markets_kind ON the_markets(kind);
CREATE INDEX idx_markets_status ON the_markets(status);
CREATE INDEX idx_markets_meta ON the_markets USING gin(meta);
CREATE INDEX idx_markets_created ON the_markets(created_at);
CREATE INDEX idx_markets_pair ON the_markets((meta->>'pair')) WHERE meta->>'pair' IS NOT NULL;
CREATE INDEX idx_markets_symbol ON the_markets((meta->>'symbol')) WHERE meta->>'symbol' IS NOT NULL;
CREATE INDEX idx_markets_session ON the_markets((meta->>'session')) WHERE meta->>'session' IS NOT NULL;
CREATE INDEX idx_markets_slug_trgm ON the_markets USING gin(slug gin_trgm_ops);

CREATE TRIGGER trg_markets_updated_at
    BEFORE UPDATE ON the_markets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
