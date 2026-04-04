-- Migration 005: agent_daily_memory -> temporal identity_N_memory columns
-- Source: 135 rows from agent_daily_memory
-- Target: UPDATE temporal SET identity_N_memory = ... WHERE slug = log_date
--
-- Each agent's daily memory (actions, decisions, knowledge, blockers, plan, summary)
-- goes into the identity_N_memory column on the matching daily temporal row,
-- where N = the agent's identity.id.
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/005_agent_daily_memory_to_temporal.sql

BEGIN;

-- Build the agent→identity_id mapping
CREATE TEMP TABLE adm_map (
    agent_slug TEXT PRIMARY KEY,
    identity_id INTEGER NOT NULL
);

-- Full-slug matches (snake_case → PascalCase)
INSERT INTO adm_map (agent_slug, identity_id)
SELECT DISTINCT adm.agent_id, i.id
FROM agent_daily_memory adm
JOIN identity i ON lower(replace(i.slug, ' ', '')) = lower(replace(replace(adm.agent_id, '_', ''), ' ', ''))
WHERE i.kind = 'agent' OR i.kind = 'user'
ON CONFLICT DO NOTHING;

-- First-name-only matches
INSERT INTO adm_map VALUES
  ('casey', 30), ('danielle', 41), ('devin', 29), ('eliana', 26),
  ('elise', 55), ('isaac', 50), ('kathryn', 6), ('lara', 62),
  ('morgan', 28), ('samir', 27), ('sanjay', 40), ('sarah', 2),
  ('sylvia', 11), ('vincent', 16), ('jmax', 21), ('lrm', 31)
ON CONFLICT DO NOTHING;

-- nova has no identity row — skip it (it'll go to the_ledger instead)

-- Verify mapping
DO $$
DECLARE
    mapped INTEGER;
    unmapped INTEGER;
BEGIN
    SELECT count(*) INTO mapped FROM adm_map;
    SELECT count(DISTINCT agent_id) INTO unmapped 
    FROM agent_daily_memory 
    WHERE agent_id NOT IN (SELECT agent_slug FROM adm_map);
    RAISE NOTICE 'Agent mapping: % mapped, % unmapped (nova)', mapped, unmapped;
END $$;

-- Now update temporal rows for each agent.
-- We build the composite memory text from all non-null fields.
-- Using dynamic SQL since we need to target identity_N_memory columns.

DO $$
DECLARE
    r RECORD;
    col_name TEXT;
    mem_text TEXT;
    update_count INTEGER := 0;
BEGIN
    FOR r IN
        SELECT 
            adm.agent_id,
            adm.log_date,
            am.identity_id,
            COALESCE(adm.actions_taken, '') AS actions,
            COALESCE(adm.decisions_made, '') AS decisions,
            COALESCE(adm.knowledge_gained, '') AS knowledge,
            COALESCE(adm.blockers, '') AS blockers,
            COALESCE(adm.handoffs, '') AS handoffs,
            COALESCE(adm.plan_tomorrow, '') AS plan,
            COALESCE(adm.daily_summary, '') AS summary
        FROM agent_daily_memory adm
        JOIN adm_map am ON am.agent_slug = adm.agent_id
        ORDER BY adm.log_date, am.identity_id
    LOOP
        col_name := 'identity_' || r.identity_id || '_memory';
        
        -- Build composite memory text
        mem_text := '';
        IF r.summary != '' THEN
            mem_text := mem_text || '## Summary' || E'\n' || r.summary || E'\n\n';
        END IF;
        IF r.actions != '' THEN
            mem_text := mem_text || '## Actions' || E'\n' || r.actions || E'\n\n';
        END IF;
        IF r.decisions != '' THEN
            mem_text := mem_text || '## Decisions' || E'\n' || r.decisions || E'\n\n';
        END IF;
        IF r.knowledge != '' THEN
            mem_text := mem_text || '## Knowledge Gained' || E'\n' || r.knowledge || E'\n\n';
        END IF;
        IF r.blockers != '' THEN
            mem_text := mem_text || '## Blockers' || E'\n' || r.blockers || E'\n\n';
        END IF;
        IF r.handoffs != '' THEN
            mem_text := mem_text || '## Handoffs' || E'\n' || r.handoffs || E'\n\n';
        END IF;
        IF r.plan != '' THEN
            mem_text := mem_text || '## Plan Tomorrow' || E'\n' || r.plan || E'\n\n';
        END IF;
        
        -- Skip if empty
        IF mem_text = '' THEN
            CONTINUE;
        END IF;
        
        -- Update the temporal row
        EXECUTE format(
            'UPDATE temporal SET %I = $1 WHERE slug = $2 AND kind = ''daily'' AND %I IS NULL',
            col_name, col_name
        ) USING mem_text, r.log_date::text;
        
        update_count := update_count + 1;
    END LOOP;
    
    RAISE NOTICE 'Updated % agent memory slots across temporal daily rows', update_count;
END $$;

-- Nova's daily memories go to the_ledger since nova has no identity row
-- (Already handled in migration 004, but let's make sure the 3 with summaries are there)

DROP TABLE adm_map;

-- Verify: check a sample
DO $$
DECLARE
    filled INTEGER;
BEGIN
    SELECT count(*) INTO filled
    FROM temporal 
    WHERE kind = 'daily' 
    AND (identity_2_memory IS NOT NULL 
      OR identity_3_memory IS NOT NULL 
      OR identity_4_memory IS NOT NULL);
    RAISE NOTICE 'Temporal daily rows with agent memories: %', filled;
END $$;

COMMIT;
