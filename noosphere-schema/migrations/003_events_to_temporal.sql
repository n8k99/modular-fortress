-- Migration: events -> temporal
-- Source: 4,842 rows from events
-- Target: temporal with kind='event'
--
-- Run with: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/003_events_to_temporal.sql
-- Rollback: 
--   DELETE FROM temporal WHERE kind = 'event';
--   ALTER TABLE temporal DROP COLUMN IF EXISTS time, DROP COLUMN IF EXISTS duration, 
--     DROP COLUMN IF EXISTS recurrence, DROP COLUMN IF EXISTS ics_uid, 
--     DROP COLUMN IF EXISTS ics_source, DROP COLUMN IF EXISTS is_external,
--     DROP COLUMN IF EXISTS description;

BEGIN;

-- Step 1: Add missing columns to temporal for event data
-- These columns exist in events but not in temporal
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS time          TEXT;
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS duration      TEXT;
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS recurrence    TEXT;
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS ics_uid       TEXT;
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS ics_source    TEXT;
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS is_external   BOOLEAN DEFAULT FALSE;
ALTER TABLE temporal ADD COLUMN IF NOT EXISTS description   TEXT;

-- Verify source count
DO $$
DECLARE
    src_count INTEGER;
BEGIN
    SELECT count(*) INTO src_count FROM events;
    RAISE NOTICE 'Source events: % rows', src_count;
END $$;

-- Insert events into temporal
INSERT INTO temporal (
    slug,
    kind,
    title,
    type,
    icon,
    body,
    description,
    start,
    time,
    duration,
    recurrence,
    tags,
    ics_uid,
    ics_source,
    is_external,
    created_at,
    updated_at
)
SELECT
    -- slug: use existing event id (already text-based like 'evt-dpn-refinements-001')
    e.id                                                    AS slug,
    'event'                                                 AS kind,
    e.title,
    e.type,
    COALESCE(e.icon, '📆')                                  AS icon,
    -- body: combine description + daily_note
    COALESCE(e.description, '') || 
    CASE WHEN e.daily_note IS NOT NULL 
        THEN E'\n\n---\nDaily note: ' || e.daily_note 
        ELSE '' 
    END                                                     AS body,
    e.description,
    e.date::date                                            AS start,
    e.time,
    e.duration,
    e.recurrence,
    e.tags::text                                            AS tags,
    e.ics_uid,
    e.ics_source,
    COALESCE(e.is_external, false)                          AS is_external,
    e.created_at,
    e.updated_at
FROM events e
-- Avoid duplicates if re-run
WHERE NOT EXISTS (
    SELECT 1 FROM temporal t
    WHERE t.slug = e.id
)
ORDER BY e.date, e.time;

-- Verify result
DO $$
DECLARE
    event_count INTEGER;
    total_count INTEGER;
BEGIN
    SELECT count(*) INTO event_count FROM temporal WHERE kind = 'event';
    SELECT count(*) INTO total_count FROM temporal;
    RAISE NOTICE 'Migrated events: % rows', event_count;
    RAISE NOTICE 'Total temporal rows: %', total_count;
END $$;

COMMIT;
