-- 12_the_index.sql
-- Global wikilink resolution index — materialized view across all 9 tables.
-- [[Target]] resolves by matching slug or title against this view.
-- Refresh periodically or on demand.

CREATE MATERIALIZED VIEW the_index AS

SELECT slug, title, 'the_chronicles' AS source_table, id, kind
FROM the_chronicles

UNION ALL

SELECT slug, title, 'the_realms' AS source_table, id, kind
FROM the_realms

UNION ALL

SELECT slug, title, 'the_press' AS source_table, id, kind
FROM the_press

UNION ALL

SELECT slug, title, 'the_markets' AS source_table, id, kind
FROM the_markets

UNION ALL

SELECT slug, title, 'the_music' AS source_table, id, kind
FROM the_music

UNION ALL

-- the_forge: exclude append-only kinds from the index
-- fitness_event, cognition_job, memory_daily are not wikilink targets
-- (tick_log and tick_report live in the_ledger now)
SELECT slug, title, 'the_forge' AS source_table, id, kind
FROM the_forge
WHERE kind NOT IN ('fitness_event', 'cognition_job', 'memory_daily')

UNION ALL

SELECT slug, title, 'the_commons' AS source_table, id, kind
FROM the_commons

UNION ALL

SELECT slug, title, 'the_work' AS source_table, id, kind
FROM the_work

UNION ALL

-- the_post: only index threads and named conversations, not individual messages
SELECT slug, title, 'the_post' AS source_table, id, kind
FROM the_post
WHERE kind IN ('inbox')

;

-- Resolution indexes
CREATE UNIQUE INDEX idx_the_index_slug ON the_index(slug);
CREATE INDEX idx_the_index_title ON the_index(LOWER(title));
CREATE INDEX idx_the_index_source ON the_index(source_table);
CREATE INDEX idx_the_index_kind ON the_index(kind);
CREATE INDEX idx_the_index_title_trgm ON the_index USING gin(title gin_trgm_ops);

-- Resolution priority for disambiguation:
-- 1. the_forge  (agents, projects, pipelines — highest priority)
-- 2. the_work   (tasks, goals)
-- 3. the_realms (world entities)
-- 4. the_chronicles (canon)
-- 5. the_press  (publications)
-- 6. the_markets (trading)
-- 7. the_music  (musicology)
-- 8. the_commons (shared resources)
-- 9. the_post   (conversations)
--
-- This priority is enforced in application code, not in the view.
-- Query pattern:
--   SELECT * FROM the_index WHERE slug = $1
--   ORDER BY CASE source_table
--     WHEN 'the_forge' THEN 1
--     WHEN 'the_work' THEN 2
--     WHEN 'the_realms' THEN 3
--     WHEN 'the_chronicles' THEN 4
--     WHEN 'the_press' THEN 5
--     WHEN 'the_markets' THEN 6
--     WHEN 'the_music' THEN 7
--     WHEN 'the_commons' THEN 8
--     WHEN 'the_post' THEN 9
--   END
--   LIMIT 1;
