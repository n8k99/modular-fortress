-- Phase 18-01: vault_notes -> memories migration
-- CRITICAL: Entire migration wrapped in single transaction for atomicity
-- Run as: sudo -u postgres psql -d master_chronicle -f /tmp/18-01-migration.sql

BEGIN;

-- Step 1: Add compression columns to vault_notes (before rename)
ALTER TABLE vault_notes ADD COLUMN compression_tier VARCHAR(32) NOT NULL DEFAULT 'daily'
    CHECK (compression_tier IN ('daily', 'weekly', 'monthly', 'quarterly', 'yearly'));
ALTER TABLE vault_notes ADD COLUMN compressed_from INTEGER[];

-- Step 2: Backfill compression_tier from note_type
UPDATE vault_notes SET compression_tier = CASE
    WHEN note_type = 'daily' THEN 'daily'
    WHEN note_type = 'weekly' THEN 'weekly'
    WHEN note_type = 'monthly' THEN 'monthly'
    WHEN note_type = 'quarterly' THEN 'quarterly'
    WHEN note_type = 'yearly' THEN 'yearly'
    WHEN note_type = 'agent_memory' THEN 'daily'
    WHEN note_type = 'freeform' THEN 'daily'
    WHEN note_type = 'zenwriter' THEN 'daily'
    WHEN note_type IS NULL THEN 'daily'
    ELSE 'daily'
END;

-- Step 3: Rename table
ALTER TABLE vault_notes RENAME TO memories;

-- Step 4: Rename indexes
ALTER INDEX vault_notes_pkey RENAME TO memories_pkey;
ALTER INDEX vault_notes_path_key RENAME TO memories_path_key;
ALTER INDEX idx_vault_notes_date RENAME TO idx_memories_date;
ALTER INDEX idx_vault_notes_type RENAME TO idx_memories_type;
ALTER INDEX idx_vault_notes_embedding RENAME TO idx_memories_embedding;

-- Step 5: Rename sequence
ALTER SEQUENCE vault_notes_id_seq RENAME TO memories_id_seq;

-- Step 6: Verify trigger survived rename (do NOT drop/recreate per Pitfall 4)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger WHERE tgrelid = 'memories'::regclass AND tgname = 'trg_sync_task_checkbox'
    ) THEN
        CREATE TRIGGER trg_sync_task_checkbox AFTER UPDATE ON memories FOR EACH ROW EXECUTE FUNCTION sync_task_checkbox();
    END IF;
END $$;

-- Step 7: Create vault_notes view
CREATE VIEW vault_notes AS SELECT * FROM memories;

-- Step 8: Create INSTEAD OF triggers on the view

-- INSERT trigger - explicit column list excluding id so DEFAULT nextval fires
-- Uses RETURNING id INTO + NEW.id assignment so RETURNING works through the view
CREATE OR REPLACE FUNCTION vault_notes_insert_fn() RETURNS trigger AS $$
DECLARE
    new_id INTEGER;
BEGIN
    INSERT INTO memories (
        path, title, frontmatter, content, size_bytes, note_type, note_date,
        embedding, created_at, modified_at, compression_tier, compressed_from,
        eliana_memories, alex_torres_memories, amara_shah_memories, amelia_frost_memories,
        ava_orozco_memories, carla_velasquez_memories, carmen_delgado_memories, casey_memories,
        daniel_cho_memories, danielle_memories, devin_memories, elise_memories,
        emilio_torres_memories, emma_rios_memories, ethan_ng_memories, evelyn_woods_memories,
        felix_wu_memories, fiona_carter_memories, harper_liu_memories, ibrahim_hassan_memories,
        isaac_memories, jasper_li_memories, jay_harper_memories, jmax_memories,
        jonah_klein_memories, jordan_blake_memories, julian_weber_memories, kai_nakamoto_memories,
        kathryn_memories, lara_memories, lena_morris_memories, leo_martin_memories,
        liam_rivera_memories, lily_nakamura_memories, lrm_memories, lucas_bryant_memories,
        mara_ellison_memories, marcello_ruiz_memories, miguel_sanchez_memories, milo_gaines_memories,
        morgan_memories, nadia_sorenson_memories, nina_castillo_memories, oliver_grant_memories,
        oscar_diaz_memories, priya_patel_memories, renee_chang_memories, samantha_yu_memories,
        samir_memories, sanjay_memories, sarah_memories, simon_beck_memories,
        sofia_lake_memories, sophie_lee_memories, sylvia_memories, tara_bennett_memories,
        tina_gray_memories, tobias_kim_memories, vincent_memories, vivian_hart_memories,
        zara_khan_memories, zoey_chen_memories, nova_memories
    ) VALUES (
        NEW.path, NEW.title, NEW.frontmatter, NEW.content, NEW.size_bytes, NEW.note_type, NEW.note_date,
        NEW.embedding, NEW.created_at, NEW.modified_at, COALESCE(NEW.compression_tier, 'daily'), NEW.compressed_from,
        NEW.eliana_memories, NEW.alex_torres_memories, NEW.amara_shah_memories, NEW.amelia_frost_memories,
        NEW.ava_orozco_memories, NEW.carla_velasquez_memories, NEW.carmen_delgado_memories, NEW.casey_memories,
        NEW.daniel_cho_memories, NEW.danielle_memories, NEW.devin_memories, NEW.elise_memories,
        NEW.emilio_torres_memories, NEW.emma_rios_memories, NEW.ethan_ng_memories, NEW.evelyn_woods_memories,
        NEW.felix_wu_memories, NEW.fiona_carter_memories, NEW.harper_liu_memories, NEW.ibrahim_hassan_memories,
        NEW.isaac_memories, NEW.jasper_li_memories, NEW.jay_harper_memories, NEW.jmax_memories,
        NEW.jonah_klein_memories, NEW.jordan_blake_memories, NEW.julian_weber_memories, NEW.kai_nakamoto_memories,
        NEW.kathryn_memories, NEW.lara_memories, NEW.lena_morris_memories, NEW.leo_martin_memories,
        NEW.liam_rivera_memories, NEW.lily_nakamura_memories, NEW.lrm_memories, NEW.lucas_bryant_memories,
        NEW.mara_ellison_memories, NEW.marcello_ruiz_memories, NEW.miguel_sanchez_memories, NEW.milo_gaines_memories,
        NEW.morgan_memories, NEW.nadia_sorenson_memories, NEW.nina_castillo_memories, NEW.oliver_grant_memories,
        NEW.oscar_diaz_memories, NEW.priya_patel_memories, NEW.renee_chang_memories, NEW.samantha_yu_memories,
        NEW.samir_memories, NEW.sanjay_memories, NEW.sarah_memories, NEW.simon_beck_memories,
        NEW.sofia_lake_memories, NEW.sophie_lee_memories, NEW.sylvia_memories, NEW.tara_bennett_memories,
        NEW.tina_gray_memories, NEW.tobias_kim_memories, NEW.vincent_memories, NEW.vivian_hart_memories,
        NEW.zara_khan_memories, NEW.zoey_chen_memories, NEW.nova_memories
    ) RETURNING id INTO new_id;
    NEW.id := new_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER vault_notes_insert
    INSTEAD OF INSERT ON vault_notes
    FOR EACH ROW EXECUTE FUNCTION vault_notes_insert_fn();

-- DELETE trigger
CREATE OR REPLACE FUNCTION vault_notes_delete_fn() RETURNS trigger AS $$
BEGIN
    DELETE FROM memories WHERE id = OLD.id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER vault_notes_delete
    INSTEAD OF DELETE ON vault_notes
    FOR EACH ROW EXECUTE FUNCTION vault_notes_delete_fn();

-- UPDATE trigger - all 75 columns explicitly listed
CREATE OR REPLACE FUNCTION vault_notes_update_fn() RETURNS trigger AS $$
BEGIN
    UPDATE memories SET
        path = NEW.path,
        title = NEW.title,
        frontmatter = NEW.frontmatter,
        content = NEW.content,
        size_bytes = NEW.size_bytes,
        note_type = NEW.note_type,
        note_date = NEW.note_date,
        embedding = NEW.embedding,
        created_at = NEW.created_at,
        modified_at = NEW.modified_at,
        compression_tier = NEW.compression_tier,
        compressed_from = NEW.compressed_from,
        eliana_memories = NEW.eliana_memories,
        alex_torres_memories = NEW.alex_torres_memories,
        amara_shah_memories = NEW.amara_shah_memories,
        amelia_frost_memories = NEW.amelia_frost_memories,
        ava_orozco_memories = NEW.ava_orozco_memories,
        carla_velasquez_memories = NEW.carla_velasquez_memories,
        carmen_delgado_memories = NEW.carmen_delgado_memories,
        casey_memories = NEW.casey_memories,
        daniel_cho_memories = NEW.daniel_cho_memories,
        danielle_memories = NEW.danielle_memories,
        devin_memories = NEW.devin_memories,
        elise_memories = NEW.elise_memories,
        emilio_torres_memories = NEW.emilio_torres_memories,
        emma_rios_memories = NEW.emma_rios_memories,
        ethan_ng_memories = NEW.ethan_ng_memories,
        evelyn_woods_memories = NEW.evelyn_woods_memories,
        felix_wu_memories = NEW.felix_wu_memories,
        fiona_carter_memories = NEW.fiona_carter_memories,
        harper_liu_memories = NEW.harper_liu_memories,
        ibrahim_hassan_memories = NEW.ibrahim_hassan_memories,
        isaac_memories = NEW.isaac_memories,
        jasper_li_memories = NEW.jasper_li_memories,
        jay_harper_memories = NEW.jay_harper_memories,
        jmax_memories = NEW.jmax_memories,
        jonah_klein_memories = NEW.jonah_klein_memories,
        jordan_blake_memories = NEW.jordan_blake_memories,
        julian_weber_memories = NEW.julian_weber_memories,
        kai_nakamoto_memories = NEW.kai_nakamoto_memories,
        kathryn_memories = NEW.kathryn_memories,
        lara_memories = NEW.lara_memories,
        lena_morris_memories = NEW.lena_morris_memories,
        leo_martin_memories = NEW.leo_martin_memories,
        liam_rivera_memories = NEW.liam_rivera_memories,
        lily_nakamura_memories = NEW.lily_nakamura_memories,
        lrm_memories = NEW.lrm_memories,
        lucas_bryant_memories = NEW.lucas_bryant_memories,
        mara_ellison_memories = NEW.mara_ellison_memories,
        marcello_ruiz_memories = NEW.marcello_ruiz_memories,
        miguel_sanchez_memories = NEW.miguel_sanchez_memories,
        milo_gaines_memories = NEW.milo_gaines_memories,
        morgan_memories = NEW.morgan_memories,
        nadia_sorenson_memories = NEW.nadia_sorenson_memories,
        nina_castillo_memories = NEW.nina_castillo_memories,
        oliver_grant_memories = NEW.oliver_grant_memories,
        oscar_diaz_memories = NEW.oscar_diaz_memories,
        priya_patel_memories = NEW.priya_patel_memories,
        renee_chang_memories = NEW.renee_chang_memories,
        samantha_yu_memories = NEW.samantha_yu_memories,
        samir_memories = NEW.samir_memories,
        sanjay_memories = NEW.sanjay_memories,
        sarah_memories = NEW.sarah_memories,
        simon_beck_memories = NEW.simon_beck_memories,
        sofia_lake_memories = NEW.sofia_lake_memories,
        sophie_lee_memories = NEW.sophie_lee_memories,
        sylvia_memories = NEW.sylvia_memories,
        tara_bennett_memories = NEW.tara_bennett_memories,
        tina_gray_memories = NEW.tina_gray_memories,
        tobias_kim_memories = NEW.tobias_kim_memories,
        vincent_memories = NEW.vincent_memories,
        vivian_hart_memories = NEW.vivian_hart_memories,
        zara_khan_memories = NEW.zara_khan_memories,
        zoey_chen_memories = NEW.zoey_chen_memories,
        nova_memories = NEW.nova_memories
    WHERE id = OLD.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER vault_notes_update
    INSTEAD OF UPDATE ON vault_notes
    FOR EACH ROW EXECUTE FUNCTION vault_notes_update_fn();

-- Step 9: Create departments table
CREATE TABLE departments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) UNIQUE NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Step 10: Seed 8 canonical departments
INSERT INTO departments (name, slug) VALUES
    ('Operations', 'operations'),
    ('Engineering', 'engineering'),
    ('Content & Brand', 'content-brand'),
    ('Creative', 'creative'),
    ('Legal', 'legal'),
    ('Music', 'music'),
    ('Strategy', 'strategy'),
    ('Executive', 'executive');

-- Step 11: Add department_id FK to agents
ALTER TABLE agents ADD COLUMN department_id INTEGER REFERENCES departments(id);

-- Step 12: Backfill department_id from existing department text
UPDATE agents SET department_id = (
    SELECT id FROM departments WHERE slug = CASE agents.department
        WHEN 'Operations' THEN 'operations'
        WHEN 'Systems' THEN 'operations'
        WHEN 'support' THEN 'operations'
        WHEN 'Engineering' THEN 'engineering'
        WHEN 'Content & Brand' THEN 'content-brand'
        WHEN 'content_brand' THEN 'content-brand'
        WHEN 'Creative' THEN 'creative'
        WHEN 'art' THEN 'creative'
        WHEN 'Legal' THEN 'legal'
        WHEN 'legal' THEN 'legal'
        WHEN 'music' THEN 'music'
        WHEN 'Research' THEN 'music'
        WHEN 'strategic_office' THEN 'strategy'
        WHEN 'audience_experience' THEN 'strategy'
        WHEN 'social_impact' THEN 'strategy'
        WHEN 'digital_partnership' THEN 'strategy'
        WHEN 'Executive' THEN 'executive'
        WHEN 'Office of the CEO' THEN 'executive'
        WHEN 'cross_functional' THEN 'executive'
    END
);

-- Step 13: Grant privileges to chronicle user
GRANT SELECT, INSERT, UPDATE, DELETE ON departments TO chronicle;
GRANT USAGE, SELECT ON SEQUENCE departments_id_seq TO chronicle;
GRANT SELECT, INSERT, UPDATE, DELETE ON vault_notes TO chronicle;

COMMIT;
