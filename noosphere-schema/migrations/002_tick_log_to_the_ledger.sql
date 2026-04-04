-- Migration: tick_log -> the_ledger
-- Source: 184,915 rows from tick_log
-- Target: the_ledger (append-only, immutable after insert)
--
-- Run with: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/002_tick_log_to_the_ledger.sql
-- Rollback: DELETE FROM the_ledger WHERE id >= 3;  (id 1-2 are pre-existing)
--
-- NOTE: the_ledger has immutability triggers (no UPDATE/DELETE).
-- We temporarily disable them for the bulk insert, then re-enable.

BEGIN;

-- Verify source count
DO $$
DECLARE
    src_count INTEGER;
BEGIN
    SELECT count(*) INTO src_count FROM tick_log;
    RAISE NOTICE 'Source tick_log: % rows', src_count;
END $$;

-- Build agent_id -> identity.id mapping
CREATE TEMP TABLE agent_id_map (
    agent_slug TEXT PRIMARY KEY,
    identity_id INTEGER
);

INSERT INTO agent_id_map (agent_slug, identity_id)
SELECT 
    lower(replace(slug, ' ', '')) AS agent_slug,
    id AS identity_id
FROM identity;

-- Add first-name-only aliases for agents that tick_log uses
INSERT INTO agent_id_map (agent_slug, identity_id)
SELECT 
    lower(split_part(full_name, ' ', 1)) AS agent_slug,
    id AS identity_id
FROM identity
WHERE lower(split_part(full_name, ' ', 1)) NOT IN (SELECT agent_slug FROM agent_id_map)
ON CONFLICT (agent_slug) DO NOTHING;

-- Special mappings
INSERT INTO agent_id_map VALUES ('jmax', 21) ON CONFLICT DO NOTHING;  -- JMaxwellCharbourne
INSERT INTO agent_id_map VALUES ('lrm', 31) ON CONFLICT DO NOTHING;   -- LRMorgenstern
INSERT INTO agent_id_map VALUES ('nova', 0) ON CONFLICT DO NOTHING;   -- retired/unknown agent

-- Temporarily disable immutability triggers on the_ledger
ALTER TABLE the_ledger DISABLE TRIGGER trg_ledger_no_delete;
ALTER TABLE the_ledger DISABLE TRIGGER trg_ledger_no_update;

-- Insert tick_log entries into the_ledger
INSERT INTO the_ledger (
    tick_number,
    ghost_id,
    tick_started_at,
    tick_completed_at,
    tick_status,
    perception_summary,
    action_taken,
    energy_before,
    energy_after,
    error_message,
    created_at
)
SELECT
    tl.tick_number,
    COALESCE(am.identity_id, 0)                             AS ghost_id,
    tl.tick_at                                              AS tick_started_at,
    tl.tick_at                                              AS tick_completed_at,
    CASE tl.action_taken
        WHEN 'idle' THEN 'idle'
        WHEN 'dormant' THEN 'dormant'
        WHEN 'request_cognition' THEN 'completed'
        WHEN 'execute_action' THEN 'completed'
        ELSE 'completed'
    END                                                     AS tick_status,
    -- Store model/LLM info in perception_summary
    CASE 
        WHEN tl.llm_called THEN 'model=' || COALESCE(tl.model_used, 'unknown') || ' tier=' || COALESCE(tl.tier, 'unknown')
        ELSE 'tier=' || COALESCE(tl.tier, 'unknown')
    END                                                     AS perception_summary,
    -- action_taken + action_detail combined
    tl.action_taken || ': ' || COALESCE(tl.action_detail::text, '{}')  AS action_taken,
    tl.energy_before,
    tl.energy_after,
    NULL                                                    AS error_message,
    tl.tick_at                                              AS created_at
FROM tick_log tl
LEFT JOIN agent_id_map am ON am.agent_slug = lower(replace(replace(tl.agent_id, '_', ''), ' ', ''))
-- Avoid duplicates: skip if tick_number+ghost_id already exists
WHERE NOT EXISTS (
    SELECT 1 FROM the_ledger l
    WHERE l.tick_number = tl.tick_number 
    AND l.ghost_id = COALESCE(am.identity_id, 0)
    AND l.tick_started_at = tl.tick_at
)
ORDER BY tl.tick_at, tl.id;

-- Re-enable immutability triggers
ALTER TABLE the_ledger ENABLE TRIGGER trg_ledger_no_delete;
ALTER TABLE the_ledger ENABLE TRIGGER trg_ledger_no_update;

-- Drop temp table
DROP TABLE agent_id_map;

-- Verify result
DO $$
DECLARE
    new_count INTEGER;
BEGIN
    SELECT count(*) INTO new_count FROM the_ledger;
    RAISE NOTICE 'Total the_ledger rows after migration: %', new_count;
END $$;

COMMIT;
