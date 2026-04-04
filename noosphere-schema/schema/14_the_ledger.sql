-- the_ledger: Append-only ghost tick log (audit trail)
-- IMMUTABLE: No UPDATEs. Only INSERTs. Permanent audit trail.
-- Every ghost tick writes a row: what they perceived, decided, and did.

CREATE TABLE IF NOT EXISTS the_ledger (
    id                  BIGSERIAL PRIMARY KEY,
    tick_number         BIGINT NOT NULL,
    ghost_id            INTEGER NOT NULL,    -- references identity.af64_id
    tick_started_at     TIMESTAMPTZ NOT NULL,
    tick_completed_at   TIMESTAMPTZ,
    tick_status         TEXT NOT NULL,        -- 'running','completed','failed','skipped'
    perception_summary  TEXT,
    action_taken        TEXT,
    energy_before       INTEGER,
    energy_after        INTEGER,
    error_message       TEXT,
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_ledger IS 'Append-only ghost tick audit trail. IMMUTABLE — no UPDATEs, only INSERTs.';

-- Indexes
CREATE INDEX idx_ledger_tick_number     ON the_ledger(tick_number);
CREATE INDEX idx_ledger_ghost_id        ON the_ledger(ghost_id);
CREATE INDEX idx_ledger_tick_status     ON the_ledger(tick_status);
CREATE INDEX idx_ledger_started_at      ON the_ledger(tick_started_at);

-- Composite: find all ticks for a specific ghost
CREATE INDEX idx_ledger_ghost_tick      ON the_ledger(ghost_id, tick_number);

-- Immutability trigger: prevent UPDATEs and DELETEs
CREATE OR REPLACE FUNCTION prevent_ledger_mutation()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'the_ledger is append-only. UPDATEs and DELETEs are not allowed.';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_ledger_no_update
    BEFORE UPDATE ON the_ledger
    FOR EACH ROW
    EXECUTE FUNCTION prevent_ledger_mutation();

CREATE TRIGGER trg_ledger_no_delete
    BEFORE DELETE ON the_ledger
    FOR EACH ROW
    EXECUTE FUNCTION prevent_ledger_mutation();
