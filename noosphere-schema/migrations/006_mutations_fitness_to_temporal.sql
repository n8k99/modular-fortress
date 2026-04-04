-- Migration 006: persona_mutations + agent_fitness → temporal identity_N_mutations / identity_N_fitness columns
-- 
-- persona_mutations: 67 rows across 2 dates (2026-03-09, 2026-03-10)
-- agent_fitness: 164 rows across 6 dates (2026-03-10, 2026-03-16, 2026-03-17, 2026-03-22, 2026-03-25, 2026-03-27)
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/006_mutations_fitness_to_temporal.sql

BEGIN;

-- ============================================================
-- STEP 1: Add identity_N_mutations and identity_N_fitness columns (1-64)
-- ============================================================

DO $$
BEGIN
    FOR i IN 1..64 LOOP
        EXECUTE format('ALTER TABLE temporal ADD COLUMN IF NOT EXISTS identity_%s_mutations TEXT', i);
        EXECUTE format('ALTER TABLE temporal ADD COLUMN IF NOT EXISTS identity_%s_fitness TEXT', i);
    END LOOP;
    RAISE NOTICE 'Added 128 columns (64 mutations + 64 fitness)';
END $$;

-- ============================================================
-- STEP 2: Create missing daily temporal rows
-- ============================================================

INSERT INTO temporal (slug, kind, title, created_at, updated_at)
SELECT d.date::text, 'daily', d.date::text, now(), now()
FROM (VALUES ('2026-03-09'::date), ('2026-03-10'::date), ('2026-03-22'::date)) d(date)
WHERE d.date::text NOT IN (SELECT slug FROM temporal WHERE kind='daily');

DO $$
DECLARE cnt INTEGER;
BEGIN
    SELECT count(*) INTO cnt FROM temporal WHERE kind='daily';
    RAISE NOTICE 'Total daily temporal rows: %', cnt;
END $$;

-- ============================================================
-- STEP 3: Build agent mapping
-- ============================================================

CREATE TEMP TABLE pm_agent_map (agent_slug TEXT PRIMARY KEY, identity_id INTEGER NOT NULL);

INSERT INTO pm_agent_map (agent_slug, identity_id)
SELECT lower(replace(slug, ' ', '')), id FROM identity WHERE kind IN ('agent','user')
ON CONFLICT DO NOTHING;

INSERT INTO pm_agent_map VALUES
  ('casey', 30), ('danielle', 41), ('devin', 29), ('eliana', 26),
  ('elise', 55), ('isaac', 50), ('kathryn', 6), ('lara', 62),
  ('morgan', 28), ('samir', 27), ('sanjay', 40), ('sarah', 2),
  ('sylvia', 11), ('vincent', 16), ('jmax', 21), ('lrm', 31)
ON CONFLICT DO NOTHING;

-- ============================================================
-- STEP 4: Write persona_mutations into identity_N_mutations columns
-- ============================================================

DO $$
DECLARE
    r RECORD;
    col_name TEXT;
    mut_text TEXT;
    update_count INTEGER := 0;
    cur_agent TEXT := '';
    cur_date TEXT := '';
    accum TEXT := '';
BEGIN
    -- Accumulate mutations per agent per date, then write once
    FOR r IN
        SELECT 
            pm.agent_id,
            pm.created_at::date::text as log_date,
            am.identity_id,
            pm.mutation_type,
            pm.description,
            COALESCE(pm.proposed_text, '') as proposed,
            pm.status,
            COALESCE(pm.fitness_delta::text, '0') as delta,
            COALESCE(pm.evidence, '') as evidence
        FROM persona_mutations pm
        JOIN pm_agent_map am ON am.agent_slug = lower(replace(replace(pm.agent_id, '_', ''), ' ', ''))
        ORDER BY pm.created_at::date, am.identity_id, pm.id
    LOOP
        -- If new agent+date combo, flush previous
        IF r.agent_id != cur_agent OR r.log_date != cur_date THEN
            IF accum != '' AND cur_agent != '' THEN
                col_name := 'identity_' || (SELECT identity_id FROM pm_agent_map WHERE agent_slug = lower(replace(replace(cur_agent, '_', ''), ' ', ''))) || '_mutations';
                EXECUTE format(
                    'UPDATE temporal SET %I = $1 WHERE slug = $2 AND kind = ''daily'' AND %I IS NULL',
                    col_name, col_name
                ) USING accum, cur_date;
                update_count := update_count + 1;
            END IF;
            cur_agent := r.agent_id;
            cur_date := r.log_date;
            accum := '';
        END IF;
        
        accum := accum || '- **' || r.mutation_type || '** (' || r.status || ', Δ' || r.delta || '): ' || r.description || E'\n';
        IF r.proposed != '' THEN
            accum := accum || '  Proposed: ' || left(r.proposed, 200) || E'\n';
        END IF;
    END LOOP;
    
    -- Flush last
    IF accum != '' AND cur_agent != '' THEN
        col_name := 'identity_' || (SELECT identity_id FROM pm_agent_map WHERE agent_slug = lower(replace(replace(cur_agent, '_', ''), ' ', ''))) || '_mutations';
        EXECUTE format(
            'UPDATE temporal SET %I = $1 WHERE slug = $2 AND kind = ''daily'' AND %I IS NULL',
            col_name, col_name
        ) USING accum, cur_date;
        update_count := update_count + 1;
    END IF;
    
    RAISE NOTICE 'Wrote % persona_mutation entries to temporal', update_count;
END $$;

-- ============================================================
-- STEP 5: Write agent_fitness into identity_N_fitness columns
-- ============================================================

DO $$
DECLARE
    r RECORD;
    col_name TEXT;
    fit_text TEXT;
    update_count INTEGER := 0;
    cur_id INTEGER := 0;
    cur_date TEXT := '';
    accum TEXT := '';
BEGIN
    FOR r IN
        SELECT 
            af.agent_id,
            af.created_at::date::text as log_date,
            am.identity_id,
            af.outcome,
            af.score,
            COALESCE(af.context, '') as context
        FROM agent_fitness af
        JOIN pm_agent_map am ON am.agent_slug = lower(replace(replace(af.agent_id::text, '_', ''), ' ', ''))
        ORDER BY af.created_at::date, am.identity_id, af.id
    LOOP
        IF r.identity_id != cur_id OR r.log_date != cur_date THEN
            IF accum != '' AND cur_id > 0 THEN
                col_name := 'identity_' || cur_id || '_fitness';
                EXECUTE format(
                    'UPDATE temporal SET %I = $1 WHERE slug = $2 AND kind = ''daily'' AND %I IS NULL',
                    col_name, col_name
                ) USING accum, cur_date;
                update_count := update_count + 1;
            END IF;
            cur_id := r.identity_id;
            cur_date := r.log_date;
            accum := '';
        END IF;
        
        accum := accum || '- ' || r.outcome || ' (score=' || r.score || ')';
        IF r.context != '' THEN
            accum := accum || ': ' || left(r.context, 150);
        END IF;
        accum := accum || E'\n';
    END LOOP;
    
    -- Flush last
    IF accum != '' AND cur_id > 0 THEN
        col_name := 'identity_' || cur_id || '_fitness';
        EXECUTE format(
            'UPDATE temporal SET %I = $1 WHERE slug = $2 AND kind = ''daily'' AND %I IS NULL',
            col_name, col_name
        ) USING accum, cur_date;
        update_count := update_count + 1;
    END IF;
    
    RAISE NOTICE 'Wrote % fitness entries to temporal', update_count;
END $$;

DROP TABLE pm_agent_map;

-- Verify
DO $$
DECLARE
    mut_filled INTEGER;
    fit_filled INTEGER;
BEGIN
    SELECT count(*) INTO mut_filled FROM temporal WHERE kind='daily' AND identity_11_mutations IS NOT NULL;
    SELECT count(*) INTO fit_filled FROM temporal WHERE kind='daily' AND identity_2_fitness IS NOT NULL;
    RAISE NOTICE 'Days with Sylvia mutations: %, Days with Sarah fitness: %', mut_filled, fit_filled;
END $$;

COMMIT;
