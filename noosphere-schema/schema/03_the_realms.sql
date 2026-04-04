-- 03_the_realms.sql
-- Sovereign Realms of Orbis — worldbuilding, entities, ghost movement

CREATE TABLE the_realms (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_realms IS 'Sovereign Realms of Orbis — present-day worldbuilding, ghost movement, burg pipeline. Entity lifestage: Seed → Sapling → Tree.';

-- Kind taxonomy:
-- continent    Major landmass
-- state        Political state/nation
-- province     Province within a state
-- burg         Settlement (the core worldbuilding unit)
-- dungeon      Dungeon/underground complex
-- wilderness   Wild/untamed region
-- landmark     Notable location or feature
-- road         Trade route or road connecting burgs
-- river        Waterway
-- npc          Non-player character in Orbis
-- creature     Creature type or species
-- item         Notable item or artifact
-- culture      Cultural group or tradition
-- religion     Religious tradition
-- ghost_position  Ghost's current location on the Orbis map (Drunkard's Walk)

-- meta for burg/province/etc:
--   x, y (map coordinates)
--   latitude, longitude
--   parent_province: [[Province Name]]
--   parent_state: [[State Name]]
--   population, lifestage (seed/sapling/tree)
--   elevation, biome, features[]

-- meta for ghost_position:
--   agent_id, current_burg: [[Burg Name]]
--   trust, energy
--   travel_mode (road/river/flight/teleport)
--   biography: [[...]]

CREATE INDEX idx_realms_kind ON the_realms(kind);
CREATE INDEX idx_realms_status ON the_realms(status);
CREATE INDEX idx_realms_meta ON the_realms USING gin(meta);
CREATE INDEX idx_realms_created ON the_realms(created_at);
CREATE INDEX idx_realms_slug_trgm ON the_realms USING gin(slug gin_trgm_ops);
CREATE INDEX idx_realms_title_trgm ON the_realms USING gin(title gin_trgm_ops);
CREATE INDEX idx_realms_lifestage ON the_realms((meta->>'lifestage')) WHERE meta->>'lifestage' IS NOT NULL;
CREATE INDEX idx_realms_parent ON the_realms((meta->>'parent_province')) WHERE meta->>'parent_province' IS NOT NULL;

CREATE TRIGGER trg_realms_updated_at
    BEFORE UPDATE ON the_realms
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
