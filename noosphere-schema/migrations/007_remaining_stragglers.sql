-- Migration 007: Remaining stragglers
--
-- agent_state (64) → identity columns (UPDATE)
-- agent_drives (157) → identity columns (UPDATE, aggregated per agent)
-- agent_areas (67) → the_links
-- area_content (1,106) → the_press / the_music / the_work / the_commons
-- areas (5) → the_commons kind='area'
-- templates (3) + templates_history (3) → the_commons kind='template'
-- resources (2) → the_commons kind='resource'
-- security_lint_* (13) → discard (noted)
-- codebase_scans (2) → discard (noted)
-- folders (5,532) → discard (Obsidian structure)
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/007_remaining_stragglers.sql

BEGIN;

-- ============================================================
-- STEP 1: agent_state → identity columns
-- ============================================================

ALTER TABLE identity ADD COLUMN IF NOT EXISTS energy INTEGER;
ALTER TABLE identity ADD COLUMN IF NOT EXISTS tier TEXT;
ALTER TABLE identity ADD COLUMN IF NOT EXISTS last_tick_at TIMESTAMPTZ;
ALTER TABLE identity ADD COLUMN IF NOT EXISTS ticks_at_current_tier INTEGER;
ALTER TABLE identity ADD COLUMN IF NOT EXISTS ticks_alive INTEGER;
ALTER TABLE identity ADD COLUMN IF NOT EXISTS dormant_since TIMESTAMPTZ;

-- Build agent slug map for state/drives/areas
CREATE TEMP TABLE state_map (agent_slug TEXT PRIMARY KEY, identity_id INTEGER NOT NULL);
INSERT INTO state_map SELECT lower(replace(slug,' ','')), id FROM identity WHERE kind IN ('agent','user') ON CONFLICT DO NOTHING;
INSERT INTO state_map VALUES
  ('casey',30),('danielle',41),('devin',29),('eliana',26),('elise',55),
  ('isaac',50),('kathryn',6),('lara',62),('morgan',28),('samir',27),
  ('sanjay',40),('sarah',2),('sylvia',11),('vincent',16),('jmax',21),('lrm',31),('nova',0)
ON CONFLICT DO NOTHING;

UPDATE identity i SET
  energy = ast.energy,
  tier = ast.tier,
  last_tick_at = ast.last_tick_at,
  ticks_at_current_tier = ast.ticks_at_current_tier,
  ticks_alive = ast.ticks_alive,
  dormant_since = ast.dormant_since
FROM agent_state ast
JOIN state_map sm ON sm.agent_slug = lower(replace(replace(ast.agent_id, '_', ''), ' ', ''))
WHERE i.id = sm.identity_id
AND i.energy IS NULL;

DO $$ DECLARE cnt INTEGER; BEGIN SELECT count(*) INTO cnt FROM identity WHERE energy IS NOT NULL; RAISE NOTICE 'agent_state: updated % identity rows', cnt; END $$;

-- ============================================================
-- STEP 2: agent_drives → identity drives column (aggregated JSON-ish text per agent)
-- ============================================================

ALTER TABLE identity ADD COLUMN IF NOT EXISTS drives TEXT;

UPDATE identity i SET drives = d.drive_text
FROM (
    SELECT sm.identity_id,
      string_agg(
        ad.drive_name || ': ' || ad.description || ' (sat=' || ad.satisfaction || ' pres=' || ad.pressure || ' frust=' || ad.frustration || ')',
        E'\n' ORDER BY ad.drive_name
      ) as drive_text
    FROM agent_drives ad
    JOIN state_map sm ON sm.agent_slug = lower(replace(replace(ad.agent_id, '_', ''), ' ', ''))
    GROUP BY sm.identity_id
) d
WHERE i.id = d.identity_id
AND i.drives IS NULL;

DO $$ DECLARE cnt INTEGER; BEGIN SELECT count(*) INTO cnt FROM identity WHERE drives IS NOT NULL; RAISE NOTICE 'agent_drives: updated % identity rows', cnt; END $$;

-- ============================================================
-- STEP 3: agent_areas → the_links
-- ============================================================

INSERT INTO the_links (source_table, source_id, source_field, target_slug, created_at)
SELECT
  'identity',
  COALESCE(sm.identity_id, 0)::bigint,
  'area_' || aa.role,
  'area-' || aa.area_id,
  now()
FROM agent_areas aa
JOIN state_map sm ON sm.agent_slug = lower(replace(replace(aa.agent_id, '_', ''), ' ', ''))
WHERE NOT EXISTS (
  SELECT 1 FROM the_links l 
  WHERE l.source_table = 'identity' AND l.source_id = sm.identity_id 
  AND l.source_field = 'area_' || aa.role AND l.target_slug = 'area-' || aa.area_id
);

DO $$ DECLARE cnt INTEGER; BEGIN SELECT count(*) INTO cnt FROM the_links WHERE source_field LIKE 'area_%'; RAISE NOTICE 'agent_areas: % links created', cnt; END $$;

-- ============================================================
-- STEP 4: area_content → multiple Nine Tables based on content_type
-- ============================================================

-- the_press: blog, thought-police, speaking, morning-pages, label (307 rows)
INSERT INTO the_press (slug, kind, title, body, status, created_at, updated_at)
SELECT
  'ac-' || ac.id,
  CASE ac.content_type
    WHEN 'blog' THEN 'blog'
    WHEN 'thought-police' THEN 'editorial'
    WHEN 'speaking' THEN 'speech'
    WHEN 'morning-pages' THEN 'morning_pages'
    WHEN 'label' THEN 'label_copy'
  END,
  ac.title,
  ac.body,
  COALESCE(ac.status, 'draft'),
  ac.created_at,
  ac.updated_at
FROM area_content ac
WHERE ac.content_type IN ('blog','thought-police','speaking','morning-pages','label')
AND NOT EXISTS (SELECT 1 FROM the_press p WHERE p.slug = 'ac-' || ac.id);

-- the_music: podcast (321 rows)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT
  'ac-' || ac.id,
  'podcast_content',
  ac.title,
  ac.body,
  COALESCE(ac.status, 'draft'),
  ac.created_at,
  ac.updated_at
FROM area_content ac
WHERE ac.content_type = 'podcast'
AND NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'ac-' || ac.id);

-- Widen the_work kind constraint BEFORE inserting tool/pipeline
ALTER TABLE the_work DROP CONSTRAINT IF EXISTS the_work_kind_check;
ALTER TABLE the_work ADD CONSTRAINT the_work_kind_check
  CHECK (kind = ANY (ARRAY['project','phase','goal','task','decision','routine','issue','extracted_task','project_history','tool','pipeline']));

-- the_work: tool, pipeline (79 rows)
INSERT INTO the_work (slug, kind, title, body, status, created_at, updated_at)
SELECT
  'ac-' || ac.id,
  CASE ac.content_type WHEN 'tool' THEN 'tool' WHEN 'pipeline' THEN 'pipeline' END,
  ac.title,
  ac.body,
  COALESCE(ac.status, 'draft'),
  ac.created_at,
  ac.updated_at
FROM area_content ac
WHERE ac.content_type IN ('tool','pipeline')
AND NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = 'ac-' || ac.id);

-- the_commons: general, branding, engineering, systems, collaboration (399 rows)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
  'ac-' || ac.id,
  ac.content_type,
  ac.title,
  ac.body,
  COALESCE(ac.status, 'draft'),
  ac.created_at,
  ac.updated_at
FROM area_content ac
WHERE ac.content_type IN ('general','branding','engineering','systems','collaboration')
AND NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'ac-' || ac.id);

DO $$ BEGIN RAISE NOTICE 'area_content routed to 4 tables'; END $$;

-- ============================================================
-- STEP 5: areas, templates, resources → the_commons
-- ============================================================

-- areas (5) → already in the_commons as kind='area' from earlier? Check
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT 'area-' || a.id, 'area', a.name, a.description, a.status, a.created_at, a.updated_at
FROM areas a
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'area-' || a.id);

-- templates (3)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT 'tpl-' || t.id, 'template', t.name, t.body, 'active', t.created_at, t.updated_at
FROM templates t
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'tpl-' || t.id);

-- templates_history (3)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT 'tplhist-' || th.id, 'template_history', 'Template v' || th.version || ' (tpl#' || th.template_id || ')', th.body, 'archived', th.changed_at, th.changed_at
FROM templates_history th
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'tplhist-' || th.id);

-- resources (2)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT 'res-' || r.id, 'resource', r.name, r.description, CASE WHEN r.frozen THEN 'frozen' ELSE 'active' END, r.created_at, r.updated_at
FROM resources r
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'res-' || r.id);

DO $$ BEGIN RAISE NOTICE 'areas/templates/resources → the_commons'; END $$;

DROP TABLE state_map;

-- ============================================================
-- FINAL: Summary
-- ============================================================

DO $$
DECLARE r RECORD;
BEGIN
    FOR r IN
        SELECT 'identity' as tbl, count(*) as cnt FROM identity
        UNION ALL SELECT 'temporal', count(*) FROM temporal
        UNION ALL SELECT 'the_work', count(*) FROM the_work
        UNION ALL SELECT 'the_post', count(*) FROM the_post
        UNION ALL SELECT 'the_markets', count(*) FROM the_markets
        UNION ALL SELECT 'the_music', count(*) FROM the_music
        UNION ALL SELECT 'the_realms', count(*) FROM the_realms
        UNION ALL SELECT 'the_chronicles', count(*) FROM the_chronicles
        UNION ALL SELECT 'the_commons', count(*) FROM the_commons
        UNION ALL SELECT 'the_press', count(*) FROM the_press
        UNION ALL SELECT 'the_ledger', count(*) FROM the_ledger
        UNION ALL SELECT 'the_links', count(*) FROM the_links
        UNION ALL SELECT 'the_index', count(*) FROM the_index
        UNION ALL SELECT 'the_aliases', count(*) FROM the_aliases
        ORDER BY tbl
    LOOP
        RAISE NOTICE '% = % rows', r.tbl, r.cnt;
    END LOOP;
END $$;

COMMIT;
