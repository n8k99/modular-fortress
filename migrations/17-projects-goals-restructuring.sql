-- Phase 17: Projects & Goals Restructuring Migration
-- Adds lifestage to projects, area_id FK to projects, project_id FK to goals
-- Executed against master_chronicle

BEGIN;

-- ============================================================
-- 1. LIFESTAGE COLUMN ON PROJECTS (SCHEMA-05, D-01)
-- ============================================================

ALTER TABLE projects ADD COLUMN IF NOT EXISTS lifestage VARCHAR(32);

ALTER TABLE projects ADD CONSTRAINT projects_lifestage_check
    CHECK (lifestage IN ('Seed', 'Sapling', 'Tree', 'Harvest'));

-- ============================================================
-- 2. BACKFILL LIFESTAGE (D-02)
-- ============================================================

UPDATE projects SET lifestage = 'Harvest' WHERE id = 1;         -- DragonPunk (completed)
UPDATE projects SET lifestage = 'Seed' WHERE id = 3;            -- Noosphere Ghosts (paused)
UPDATE projects SET lifestage = 'Tree' WHERE id IN (5,6,7,9,10,12,13,14,16,17,51);  -- long-running active
UPDATE projects SET lifestage = 'Sapling' WHERE id IN (56, 59); -- recent active

-- Verify all backfilled before making NOT NULL
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM projects WHERE lifestage IS NULL) THEN
        RAISE EXCEPTION 'Lifestage backfill incomplete: NULL lifestage found';
    END IF;
END $$;

ALTER TABLE projects ALTER COLUMN lifestage SET NOT NULL;

-- ============================================================
-- 3. FORWARD-ONLY TRIGGER (D-03) -- AFTER backfill to avoid Pitfall 4
-- ============================================================

CREATE OR REPLACE FUNCTION enforce_lifestage_forward_only() RETURNS TRIGGER AS $$
DECLARE
    old_rank INTEGER;
    new_rank INTEGER;
BEGIN
    old_rank := CASE OLD.lifestage
        WHEN 'Seed' THEN 1
        WHEN 'Sapling' THEN 2
        WHEN 'Tree' THEN 3
        WHEN 'Harvest' THEN 4
    END;
    new_rank := CASE NEW.lifestage
        WHEN 'Seed' THEN 1
        WHEN 'Sapling' THEN 2
        WHEN 'Tree' THEN 3
        WHEN 'Harvest' THEN 4
    END;

    IF new_rank < old_rank THEN
        RAISE EXCEPTION 'Lifestage transition not allowed: % -> % (forward only)',
            OLD.lifestage, NEW.lifestage;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_project_lifestage
    BEFORE UPDATE ON projects
    FOR EACH ROW
    WHEN (OLD.lifestage IS DISTINCT FROM NEW.lifestage)
    EXECUTE FUNCTION enforce_lifestage_forward_only();

-- ============================================================
-- 4. AREA_ID FK ON PROJECTS (SCHEMA-07, D-07)
-- ============================================================

ALTER TABLE projects ADD COLUMN IF NOT EXISTS area_id INTEGER REFERENCES areas(id);
CREATE INDEX IF NOT EXISTS idx_projects_area_id ON projects(area_id);

-- ============================================================
-- 5. BACKFILL AREA ASSIGNMENTS (D-08)
-- ============================================================

UPDATE projects SET area_id = 5 WHERE id IN (3, 51, 13, 5);    -- Infrastructure/Systems
UPDATE projects SET area_id = 1 WHERE id IN (10, 12, 14, 6);   -- EM Corp
UPDATE projects SET area_id = 2 WHERE id IN (16, 17, 9, 1);    -- Orbis
UPDATE projects SET area_id = 3 WHERE id = 7;                   -- Living Room Music
UPDATE projects SET area_id = 4 WHERE id IN (56, 59);           -- N8K99/Personal

-- ============================================================
-- 6. PROJECT_ID FK ON GOALS (SCHEMA-06, D-04)
-- ============================================================

ALTER TABLE goals ADD COLUMN IF NOT EXISTS project_id INTEGER REFERENCES projects(id);
CREATE INDEX IF NOT EXISTS idx_goals_project_id ON goals(project_id);

-- ============================================================
-- 7. MIGRATE WIKILINKS TO FK (D-05)
-- ============================================================

UPDATE goals SET project_id = 1
    WHERE project IN ('[[Project DragonPunk]]', '{{"Project DragonPunk"}}');
-- GOTCHA and Puppet Show: leave project_id NULL (no matching projects)

-- ============================================================
-- 8. VERIFICATION
-- ============================================================

DO $$ BEGIN
    IF (SELECT COUNT(*) FROM goals WHERE project LIKE '%DragonPunk%' AND project_id IS NULL) > 0 THEN
        RAISE EXCEPTION 'Goals migration incomplete: DragonPunk goals without project_id';
    END IF;
END $$;

DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM projects WHERE lifestage IS NULL) THEN
        RAISE EXCEPTION 'Post-migration check failed: NULL lifestage found';
    END IF;
END $$;

-- ============================================================
-- 9. UPDATED_AT TRIGGER ON PROJECTS (per research Open Question 2)
-- ============================================================

DO $$ BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'update_projects_updated_at'
        AND tgrelid = 'projects'::regclass
    ) THEN
        CREATE TRIGGER update_projects_updated_at
            BEFORE UPDATE ON projects
            FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
    END IF;
END $$;

COMMIT;
