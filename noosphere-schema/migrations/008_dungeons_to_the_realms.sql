-- Migration 008: Resources/Dungeons → the_realms (kind='dungeon')
-- Source: 778 rows from document_versions WHERE source_location LIKE 'Resources/Dungeons/%'
-- Target: the_realms with Azgaar/watabou metadata in proper columns
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/008_dungeons_to_the_realms.sql

BEGIN;

-- Verify source
DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM document_versions WHERE source_location LIKE 'Resources/Dungeons/%';
    RAISE NOTICE 'Source: % dungeon entries in document_versions', cnt;
END $$;

-- Insert dungeons into the_realms
-- Skip any that already exist by title match
INSERT INTO the_realms (
    slug, kind, title, body, status,
    x, y, cell, json_id, afwg_marker_id, marker_type,
    parent_province, parent_state,
    created_at, updated_at
)
SELECT
    -- slug: dungeon-{afwg_marker_id} or dungeon-{id} for entries without marker
    'dungeon-' || dv.id,
    'dungeon',
    dv.title,
    dv.version_content,
    'active',
    -- Azgaar coordinates
    NULLIF((dv.version_frontmatter::jsonb)->>'x', '')::numeric,
    NULLIF((dv.version_frontmatter::jsonb)->>'y', '')::numeric,
    NULLIF((dv.version_frontmatter::jsonb)->>'cell', '')::integer,
    NULLIF((dv.version_frontmatter::jsonb)->>'json_id', '')::integer,
    NULLIF((dv.version_frontmatter::jsonb)->>'afwg_marker_id', '')::integer,
    -- marker_type preserves the original Azgaar label (Wrymling/Hatchlings, Adult-dragon-lair, etc.)
    COALESCE(
        (dv.version_frontmatter::jsonb)->>'marker_type',
        CASE
            WHEN dv.title LIKE 'Adult-dragon-lair%' THEN 'Adult-dragon-lair'
            WHEN dv.title LIKE 'ancient-dragon%' THEN 'ancient-dragon-lair'
            WHEN dv.title LIKE 'wyrmling%' OR dv.title LIKE 'Wyrmling%' THEN 'Wrymling/Hatchlings'
            ELSE 'dungeon'
        END
    ),
    -- parent references (wikilink text from frontmatter)
    (dv.version_frontmatter::jsonb)->>'parent_province',
    (dv.version_frontmatter::jsonb)->>'parent_state',
    dv.created_at,
    dv.created_at
FROM document_versions dv
WHERE dv.source_location LIKE 'Resources/Dungeons/%'
AND NOT EXISTS (
    SELECT 1 FROM the_realms r WHERE r.slug = 'dungeon-' || dv.id
)
ORDER BY dv.title;

-- Verify
DO $$ DECLARE 
    new_count INTEGER;
    total INTEGER;
BEGIN
    SELECT count(*) INTO new_count FROM the_realms WHERE kind = 'dungeon';
    SELECT count(*) INTO total FROM the_realms;
    RAISE NOTICE 'Dungeons migrated: % | Total the_realms: %', new_count, total;
END $$;

-- Breakdown by marker_type
DO $$ DECLARE r RECORD; BEGIN
    FOR r IN SELECT marker_type, count(*) as cnt FROM the_realms WHERE kind='dungeon' GROUP BY marker_type ORDER BY cnt DESC LOOP
        RAISE NOTICE '  %: %', r.marker_type, r.cnt;
    END LOOP;
END $$;

COMMIT;
