-- Migration 009: Orbis content from document_versions → the_realms + the_chronicles
-- Source: ~3,243 unique entries (deduped by title) with [[wikilink]] type frontmatter
-- Target: the_realms (geography, NPCs, dragons, items, locations)
--         the_chronicles (chapters, adventures, epics, scenes, history)
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/009_orbis_content_to_realms_and_chronicles.sql

BEGIN;

-- ============================================================
-- Build deduped source: one row per unique title, newest version wins
-- ============================================================

CREATE TEMP TABLE orbis_source AS
SELECT DISTINCT ON (lower(title)) 
    id, title, version_content, version_frontmatter, created_at,
    (version_frontmatter::jsonb)->>'type' as fm_type,
    -- Extract common frontmatter fields
    (version_frontmatter::jsonb)->>'x' as x_coord,
    (version_frontmatter::jsonb)->>'y' as y_coord,
    (version_frontmatter::jsonb)->>'cell' as cell,
    (version_frontmatter::jsonb)->>'json_id' as json_id,
    (version_frontmatter::jsonb)->>'afwg_marker_id' as afwg_marker_id,
    (version_frontmatter::jsonb)->>'marker_type' as marker_type,
    (version_frontmatter::jsonb)->>'parent_province' as parent_province,
    (version_frontmatter::jsonb)->>'parent_state' as parent_state,
    (version_frontmatter::jsonb)->>'icon_type' as icon_type
FROM document_versions
WHERE version_frontmatter IS NOT NULL 
AND version_frontmatter NOT LIKE '-%'
AND version_frontmatter ~ '^\{.*\}$'
AND (version_frontmatter::jsonb)->>'type' LIKE '%[[%'
ORDER BY lower(title), version_date DESC NULLS LAST, id DESC;

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM orbis_source;
    RAISE NOTICE 'Orbis source (deduped): % entries', cnt;
END $$;

-- ============================================================
-- STEP 1: the_realms — Geography, NPCs, Dragons, Items, Locations
-- ============================================================

-- Map fm_type → kind for the_realms
INSERT INTO the_realms (
    slug, kind, title, body, status,
    x, y, cell, json_id, afwg_marker_id, marker_type,
    parent_province, parent_state,
    created_at, updated_at
)
SELECT
    'dv-' || os.id,
    CASE 
        -- Geographic features
        WHEN os.fm_type LIKE '%Necropolise%' THEN 'necropolis'
        WHEN os.fm_type LIKE '%Monument%' THEN 'monument'
        WHEN os.fm_type LIKE '%Mine%' THEN 'mine'
        WHEN os.fm_type LIKE '%Burial%' THEN 'burial'
        WHEN os.fm_type LIKE '%Ruins%' THEN 'ruins'
        WHEN os.fm_type LIKE '%Point of Interest%' THEN 'poi'
        WHEN os.fm_type LIKE '%Inns%' THEN 'inn'
        WHEN os.fm_type LIKE '%Hot Springs%' THEN 'hot_spring'
        WHEN os.fm_type LIKE '%Tower%' THEN 'tower'
        WHEN os.fm_type LIKE '%statues%' THEN 'statue'
        WHEN os.fm_type LIKE '%Planar%' THEN 'planar_geography'
        WHEN os.fm_type LIKE '%Geographic Feature%' THEN 'geographic_feature'
        -- Political/social
        WHEN os.fm_type LIKE '%Province%' THEN 'province'
        WHEN os.fm_type LIKE '%Burg%' THEN 'burg'
        WHEN os.fm_type LIKE '%District%' THEN 'district'
        WHEN os.fm_type LIKE '%Sovereign%' THEN 'state'
        WHEN os.fm_type LIKE '%Population Axis%' THEN 'population'
        WHEN os.fm_type LIKE '%Location Axis%' THEN 'location_axis'
        WHEN os.fm_type LIKE '%Government%' THEN 'government'
        WHEN os.fm_type LIKE '%Military%' THEN 'military'
        WHEN os.fm_type LIKE '%Intelligence%' THEN 'intelligence'
        WHEN os.fm_type LIKE '%Culture%' THEN 'culture'
        WHEN os.fm_type LIKE '%Religion%' THEN 'religion'
        WHEN os.fm_type LIKE '%Faction%' THEN 'faction'
        WHEN os.fm_type LIKE '%Historical Civilization%' THEN 'civilization'
        -- Entities
        WHEN os.fm_type LIKE '%NPC%' THEN 'npc'
        WHEN os.fm_type LIKE '%Dragon%' THEN 'dragon'
        WHEN os.fm_type LIKE '%Player%' THEN 'player_character'
        WHEN os.fm_type LIKE '%Actor%' THEN 'actor'
        -- Items/Loot
        WHEN os.fm_type LIKE '%Items%' THEN 'item'
        WHEN os.fm_type LIKE '%Treasure%' THEN 'treasure'
        WHEN os.fm_type LIKE '%Magic Item%' THEN 'magic_item'
        WHEN os.fm_type LIKE '%Legendary%' THEN 'legendary_artifact'
        WHEN os.fm_type LIKE '%Armor%' THEN 'armor'
        WHEN os.fm_type LIKE '%Loot%' THEN 'loot'
        -- Dungeons (additional ones not caught by migration 008)
        WHEN os.fm_type LIKE '%Dungeon%' THEN 'dungeon'
        -- Templates
        WHEN os.fm_type = '[[Templates]]' THEN 'template'
        WHEN os.fm_type LIKE '%Portfolio%' THEN 'portfolio'
        ELSE 'orbis_entry'
    END,
    os.title,
    os.version_content,
    'active',
    NULLIF(os.x_coord, '')::numeric,
    NULLIF(os.y_coord, '')::numeric,
    NULLIF(os.cell, '')::integer,
    NULLIF(os.json_id, '')::integer,
    NULLIF(os.afwg_marker_id, '')::integer,
    os.marker_type,
    os.parent_province,
    os.parent_state,
    os.created_at,
    os.created_at
FROM orbis_source os
WHERE os.fm_type NOT IN (
    -- These go to the_chronicles instead
    '[[Chapter]]', '[[Adventure]]', '[[Grand Epic]]', '[[Scene]]', 
    '[[Moment]]', '[[History]]', '[[Historical Records]]',
    '[[Narrative Arc]]', '[[Scene Note]]',
    -- Temporal entries
    '[[Weekly Note]]', '[[Yearly Note]]', '[[Quarterly Note]]', '[[Temporal Axis]]'
)
AND NOT EXISTS (SELECT 1 FROM the_realms r WHERE lower(r.title) = lower(os.title))
AND NOT EXISTS (SELECT 1 FROM the_realms r WHERE r.slug = 'dv-' || os.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_realms WHERE slug LIKE 'dv-%';
    RAISE NOTICE 'New the_realms entries (dv- prefix): %', cnt;
END $$;

-- ============================================================
-- STEP 2: the_chronicles — Narrative content
-- ============================================================

INSERT INTO the_chronicles (
    slug, kind, title, body, status, created_at, updated_at
)
SELECT
    'dv-' || os.id,
    CASE 
        WHEN os.fm_type LIKE '%Chapter%' THEN 'chapter'
        WHEN os.fm_type LIKE '%Adventure%' THEN 'adventure'
        WHEN os.fm_type LIKE '%Grand Epic%' THEN 'epic'
        WHEN os.fm_type LIKE '%Scene%' THEN 'scene'
        WHEN os.fm_type LIKE '%Moment%' THEN 'moment'
        WHEN os.fm_type LIKE '%History%' THEN 'history'
        WHEN os.fm_type LIKE '%Historical Records%' THEN 'historical_record'
        ELSE 'narrative'
    END,
    os.title,
    os.version_content,
    'active',
    os.created_at,
    os.created_at
FROM orbis_source os
WHERE os.fm_type IN (
    '[[Chapter]]', '[[Adventure]]', '[[Grand Epic]]', '[[Scene]]', 
    '[[Moment]]', '[[History]]', '[[Historical Records]]'
)
AND NOT EXISTS (SELECT 1 FROM the_chronicles c WHERE lower(c.title) = lower(os.title))
AND NOT EXISTS (SELECT 1 FROM the_chronicles c WHERE c.slug = 'dv-' || os.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_chronicles WHERE slug LIKE 'dv-%';
    RAISE NOTICE 'New the_chronicles entries (dv- prefix): %', cnt;
END $$;

DROP TABLE orbis_source;

-- ============================================================
-- FINAL: Summary by kind
-- ============================================================

DO $$ DECLARE r RECORD; BEGIN
    RAISE NOTICE '--- the_realms by kind ---';
    FOR r IN SELECT kind, count(*) as cnt FROM the_realms GROUP BY kind ORDER BY cnt DESC LOOP
        RAISE NOTICE '  %: %', r.kind, r.cnt;
    END LOOP;
    RAISE NOTICE '--- the_chronicles by kind ---';
    FOR r IN SELECT kind, count(*) as cnt FROM the_chronicles GROUP BY kind ORDER BY cnt DESC LOOP
        RAISE NOTICE '  %: %', r.kind, r.cnt;
    END LOOP;
END $$;

COMMIT;
