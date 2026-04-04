-- Migration 010: Dragon lairs + AI chat logs + narrative arcs from document_versions
--
-- Dragon lairs/content (689) → the_realms
-- AI Chat logs (938) → the_post  
-- Narrative arcs (227) → the_chronicles
-- Items/Loot (150) → the_realms
-- Geography dupes (100) → the_realms
-- People/Staff (120) → identity or the_realms
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/010_dragons_chats_narratives.sql

BEGIN;

-- Deduped source of all remaining unmigrated document_versions
CREATE TEMP TABLE remaining_dv AS
SELECT DISTINCT ON (lower(title))
    id, title, version_content, version_frontmatter, source_location, created_at
FROM document_versions dv
WHERE NOT EXISTS (SELECT 1 FROM the_realms r WHERE lower(r.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM the_chronicles c WHERE lower(c.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM identity i WHERE lower(i.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM the_work w WHERE lower(w.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM the_post p WHERE lower(p.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM the_commons c WHERE lower(c.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM the_music m WHERE lower(m.title) = lower(dv.title))
AND NOT EXISTS (SELECT 1 FROM the_press pr WHERE lower(pr.title) = lower(dv.title))
ORDER BY lower(title), id DESC;

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM remaining_dv;
    RAISE NOTICE 'Remaining unique unmigrated: %', cnt;
END $$;

-- ============================================================
-- STEP 1: Dragons + Dragon Lairs → the_realms
-- ============================================================

INSERT INTO the_realms (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'dvr-' || r.id,
    CASE 
        WHEN r.title LIKE '%Lair' THEN 'dragon_lair'
        WHEN r.source_location LIKE '%/dragon/%' THEN 'dragon_statblock'
        ELSE 'dragon'
    END,
    r.title,
    r.version_content,
    'active',
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE (r.source_location LIKE '%/Dragons/%' OR r.source_location LIKE '%/dragon/%' OR r.title LIKE '%Lair')
AND r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_realms tr WHERE tr.slug = 'dvr-' || r.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_realms WHERE slug LIKE 'dvr-%';
    RAISE NOTICE 'Dragons/lairs added to the_realms: %', cnt;
END $$;

-- ============================================================
-- STEP 2: Items/Loot/Geography/People → the_realms
-- ============================================================

INSERT INTO the_realms (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'dvr2-' || r.id,
    CASE 
        WHEN r.source_location LIKE '%/Loot/%' OR r.source_location LIKE '%/Treasure/%' OR r.source_location LIKE '%/items/%' THEN 'item'
        WHEN r.source_location LIKE '%/Burgs/%' THEN 'burg'
        WHEN r.source_location LIKE '%/States/%' THEN 'state'
        WHEN r.source_location LIKE '%/Province%' THEN 'province'
        WHEN r.source_location LIKE '%/Geography/%' THEN 'geography'
        WHEN r.source_location LIKE '%/NPCs/%' OR r.source_location LIKE '%/NPC%' THEN 'npc'
        WHEN r.source_location LIKE '%/deities/%' OR r.source_location LIKE '%ReligiousOrders/%' THEN 'deity'
        WHEN r.source_location LIKE '%/Factions%' THEN 'faction'
        WHEN r.source_location LIKE '%/Rivers/%' THEN 'river'
        WHEN r.source_location LIKE '%/battlefields/%' OR r.source_location LIKE '%/battlefield%' THEN 'battlefield'
        ELSE 'orbis_misc'
    END,
    r.title,
    r.version_content,
    'active',
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE (
    r.source_location LIKE '%/Loot/%' OR r.source_location LIKE '%/Treasure/%' OR r.source_location LIKE '%/items/%'
    OR r.source_location LIKE '%/Burgs/%' OR r.source_location LIKE '%/States/%' OR r.source_location LIKE '%/Province%'
    OR r.source_location LIKE '%/Geography/%'
    OR r.source_location LIKE '%/NPCs/%' OR r.source_location LIKE '%/NPC%'
    OR r.source_location LIKE '%/deities/%' OR r.source_location LIKE '%ReligiousOrders/%'
    OR r.source_location LIKE '%/Factions%'
    OR r.source_location LIKE '%/Rivers/%'
    OR r.source_location LIKE '%/battlefields/%'
)
AND r.source_location NOT LIKE '%/Dragons/%'
AND r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_realms tr WHERE tr.slug = 'dvr2-' || r.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_realms WHERE slug LIKE 'dvr2-%';
    RAISE NOTICE 'Items/geography/people added to the_realms: %', cnt;
END $$;

-- ============================================================
-- STEP 3: AI Chat Logs → the_post
-- ============================================================

INSERT INTO the_post (slug, kind, title, body, status, protocol, from_identity, created_at, updated_at)
SELECT
    'chat-' || r.id,
    'ai_chat',
    r.title,
    r.version_content,
    'archived',
    'nexus',
    CASE 
        WHEN r.title LIKE '%T.A.S.K.S.%' THEN 'TASKS'
        ELSE 'NathanEckenrode'
    END,
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE r.source_location LIKE '%Nexus AI Chat%'
AND r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_post tp WHERE tp.slug = 'chat-' || r.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_post WHERE kind = 'ai_chat';
    RAISE NOTICE 'AI chat logs added to the_post: %', cnt;
END $$;

-- ============================================================
-- STEP 4: Narrative Arcs → the_chronicles
-- ============================================================

INSERT INTO the_chronicles (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'dvc-' || r.id,
    CASE
        WHEN r.source_location LIKE '%/Epics/%' THEN 'epic_content'
        WHEN r.source_location LIKE '%NarrativeArcs/%' THEN 'narrative_arc'
        WHEN r.source_location LIKE '%Narrative%' THEN 'narrative'
        ELSE 'chronicle'
    END,
    r.title,
    r.version_content,
    'active',
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE (r.source_location LIKE '%NarrativeArcs/%' OR r.source_location LIKE '%Narrative%' OR r.source_location LIKE '%/Epics/%')
AND r.source_location NOT LIKE '%/Dragons/%'
AND r.source_location NOT LIKE '%Nexus AI Chat%'
AND r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_chronicles tc WHERE tc.slug = 'dvc-' || r.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_chronicles WHERE slug LIKE 'dvc-%';
    RAISE NOTICE 'Narratives added to the_chronicles: %', cnt;
END $$;

-- ============================================================
-- STEP 5: D&D SRD Reference Material → the_commons
-- ============================================================

INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'srd-' || r.id,
    CASE
        WHEN r.source_location LIKE '%/Spells/%' OR r.source_location LIKE '%/spells/%' THEN 'spell'
        WHEN r.source_location LIKE '%bestiary/%' OR r.source_location LIKE '%Monsters/%' OR r.source_location LIKE '%MonsterManual%' THEN 'monster'
        WHEN r.source_location LIKE '%/classes/%' THEN 'class'
        WHEN r.source_location LIKE '%/rules/%' OR r.source_location LIKE '%/Rules/%' THEN 'rule'
        WHEN r.source_location LIKE '%DM%Guide%' THEN 'dm_guide'
        WHEN r.source_location LIKE '%compendium/%' THEN 'compendium'
        WHEN r.source_location LIKE '%foundry-data/%' THEN 'foundry_data'
        WHEN r.source_location LIKE '%DND.SRD%' OR r.source_location LIKE '%dnd5e%' OR r.source_location LIKE '%5e-markdown%' THEN 'srd_reference'
        ELSE 'reference'
    END,
    r.title,
    r.version_content,
    'active',
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE (
    r.source_location LIKE '%/Spells/%' OR r.source_location LIKE '%/spells/%'
    OR r.source_location LIKE '%bestiary/%' OR r.source_location LIKE '%Monsters/%' OR r.source_location LIKE '%MonsterManual%'
    OR r.source_location LIKE '%/classes/%'
    OR r.source_location LIKE '%/rules/%' OR r.source_location LIKE '%/Rules/%'
    OR r.source_location LIKE '%DM%Guide%'
    OR r.source_location LIKE '%compendium/%'
    OR r.source_location LIKE '%foundry-data/%'
    OR r.source_location LIKE '%DND.SRD%' OR r.source_location LIKE '%dnd5e%' OR r.source_location LIKE '%5e-markdown%'
)
AND r.source_location NOT LIKE '%/Dragons/%'
AND r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_commons tc WHERE tc.slug = 'srd-' || r.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_commons WHERE slug LIKE 'srd-%';
    RAISE NOTICE 'D&D SRD content added to the_commons: %', cnt;
END $$;

-- ============================================================  
-- STEP 6: Themes/Values → the_commons (agent personality reference)
-- ============================================================

INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'theme-' || r.id,
    'theme',
    r.title,
    r.version_content,
    'active',
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE r.source_location LIKE '%ThemesandValues%'
AND r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_commons tc WHERE tc.slug = 'theme-' || r.id);

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_commons WHERE kind = 'theme';
    RAISE NOTICE 'Themes added to the_commons: %', cnt;
END $$;

-- ============================================================
-- STEP 7: Everything else → the_commons as 'archive' kind
-- Converted notes, project docs, templates, uncategorized
-- ============================================================

INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'arch-' || r.id,
    CASE
        WHEN r.source_location LIKE '%converted-notes%' OR r.source_location LIKE '%scripts/%' THEN 'converted_note'
        WHEN r.source_location LIKE '%Projects/%' THEN 'project_doc'
        WHEN r.source_location LIKE 'Templates/%' THEN 'template'
        WHEN r.source_location LIKE '%EM Staff%' OR r.source_location LIKE '%PeopleDirectory/%' OR r.source_location LIKE '%People Directory%' THEN 'people_doc'
        WHEN r.source_location LIKE '%puppet-show%' OR r.source_location LIKE '%node_modules%' THEN 'junk'
        ELSE 'uncategorized'
    END,
    r.title,
    r.version_content,
    'archived',
    r.created_at,
    r.created_at
FROM remaining_dv r
WHERE r.title NOT LIKE '{{%'
AND NOT EXISTS (SELECT 1 FROM the_realms tr WHERE tr.slug IN ('dvr-' || r.id, 'dvr2-' || r.id))
AND NOT EXISTS (SELECT 1 FROM the_post tp WHERE tp.slug = 'chat-' || r.id)
AND NOT EXISTS (SELECT 1 FROM the_chronicles tc WHERE tc.slug = 'dvc-' || r.id)
AND NOT EXISTS (SELECT 1 FROM the_commons tc WHERE tc.slug IN ('srd-' || r.id, 'theme-' || r.id, 'arch-' || r.id))
AND r.source_location NOT LIKE '%/Dragons/%'
AND r.source_location NOT LIKE '%Nexus AI Chat%'
AND r.source_location NOT LIKE '%NarrativeArcs/%'
AND r.source_location NOT LIKE '%Narrative%'
AND r.source_location NOT LIKE '%/Epics/%'
AND r.source_location NOT LIKE '%/Spells/%' AND r.source_location NOT LIKE '%/spells/%'
AND r.source_location NOT LIKE '%bestiary/%' AND r.source_location NOT LIKE '%Monsters/%' AND r.source_location NOT LIKE '%MonsterManual%'
AND r.source_location NOT LIKE '%/classes/%'
AND r.source_location NOT LIKE '%/rules/%' AND r.source_location NOT LIKE '%/Rules/%'
AND r.source_location NOT LIKE '%DM%Guide%'
AND r.source_location NOT LIKE '%compendium/%'
AND r.source_location NOT LIKE '%foundry-data/%'
AND r.source_location NOT LIKE '%DND.SRD%' AND r.source_location NOT LIKE '%dnd5e%' AND r.source_location NOT LIKE '%5e-markdown%'
AND r.source_location NOT LIKE '%ThemesandValues%'
AND r.source_location NOT LIKE '%/Loot/%' AND r.source_location NOT LIKE '%/Treasure/%' AND r.source_location NOT LIKE '%/items/%'
AND r.source_location NOT LIKE '%/Burgs/%' AND r.source_location NOT LIKE '%/States/%' AND r.source_location NOT LIKE '%/Province%'
AND r.source_location NOT LIKE '%/Geography/%'
AND r.source_location NOT LIKE '%/NPCs/%' AND r.source_location NOT LIKE '%/NPC%'
AND r.source_location NOT LIKE '%/deities/%' AND r.source_location NOT LIKE '%ReligiousOrders/%'
AND r.source_location NOT LIKE '%/Factions%'
AND r.source_location NOT LIKE '%/Rivers/%'
AND r.source_location NOT LIKE '%/battlefields/%';

DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(*) INTO cnt FROM the_commons WHERE slug LIKE 'arch-%';
    RAISE NOTICE 'Archive/uncategorized added to the_commons: %', cnt;
END $$;

DROP TABLE remaining_dv;

-- ============================================================
-- FINAL TALLY
-- ============================================================

DO $$ DECLARE r RECORD; BEGIN
    RAISE NOTICE '=== FINAL NINE TABLES STATE ===';
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
        RAISE NOTICE '  % = %', r.tbl, r.cnt;
    END LOOP;
END $$;

-- Check: how many document_versions titles are STILL unmigrated?
DO $$ DECLARE cnt INTEGER; BEGIN
    SELECT count(DISTINCT lower(title)) INTO cnt
    FROM document_versions dv
    WHERE NOT EXISTS (SELECT 1 FROM the_realms r WHERE lower(r.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM the_chronicles c WHERE lower(c.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM identity i WHERE lower(i.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM the_work w WHERE lower(w.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM the_post p WHERE lower(p.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM the_commons c WHERE lower(c.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM the_music m WHERE lower(m.title) = lower(dv.title))
    AND NOT EXISTS (SELECT 1 FROM the_press pr WHERE lower(pr.title) = lower(dv.title));
    RAISE NOTICE 'Remaining unmigrated unique titles: %', cnt;
END $$;

COMMIT;
