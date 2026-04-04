-- the_realms: Geography (State→Province→Burg→District→POI) + Entities (NPCs, Creatures, Dragons, Factions)
-- Source: Azgaar Fantasy World Generator data + Orbis worldbuilding docs
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.
-- 70+ fields, massively sparse across geographic and entity kinds.

CREATE TABLE IF NOT EXISTS the_realms (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- 'state','province','burg','district','poi','river','route','npc','creature','dragon','faction'
    title           TEXT NOT NULL,
    body            TEXT,
    name            TEXT,           -- alternate name field from Azgaar
    type            TEXT,
    icon            TEXT,
    lifestage       TEXT CHECK (lifestage IN ('🌱 Seed', '🌿 Sapling', '🌳 Tree')),
    status          TEXT,

    -- Geographic hierarchy (wikilinks)
    parent_state    TEXT,
    parent_province TEXT,
    parent_burg     TEXT,
    parent_district TEXT,
    state_full      TEXT,
    province        TEXT,
    burg            TEXT,
    district        TEXT,
    poi             TEXT,

    -- Coordinates & geography
    latitude        NUMERIC,
    longitude       NUMERIC,
    x               NUMERIC,
    y               NUMERIC,
    cell            INTEGER,
    elevation_ft    NUMERIC,
    population      INTEGER,
    total_population INTEGER,
    population_current INTEGER,
    population_density NUMERIC,
    area_mi2        NUMERIC,
    biome           TEXT,
    climate         TEXT,
    terrain_dominant_biome TEXT,
    terrain_dominant_biome_id INTEGER,
    terrain_difficulty TEXT,
    terrain_biome_diversity TEXT,
    terrain_elevation_avg NUMERIC,
    terrain_elevation_max NUMERIC,
    terrain_elevation_min NUMERIC,
    terrain_temperature_avg NUMERIC,
    terrain_temperature_max NUMERIC,
    terrain_temperature_min NUMERIC,
    precipitation   TEXT,
    temperature     TEXT,
    temperature_likeness TEXT,
    seasonal_variations TEXT,
    natural_hazards TEXT,

    -- State fields
    state_name      TEXT,
    state_form      TEXT,
    state_form_name TEXT,
    state_type      TEXT,
    state_color     TEXT,
    state_expansionism NUMERIC,
    state_burgs_count INTEGER,
    state_cells     INTEGER,
    state_population_rural INTEGER,
    state_population_urban INTEGER,
    state_population_total INTEGER,
    state_population_density NUMERIC,
    state_urban_percentage NUMERIC,
    state_urbanization NUMERIC,
    state_area      NUMERIC,
    state_pole_x    NUMERIC,
    state_pole_y    NUMERIC,
    state_alert_level TEXT,
    state_location  TEXT,
    government_type TEXT,
    provinces       TEXT,           -- wikilink list
    neighbor_states TEXT,           -- wikilink list

    -- Burg/District/POI
    capital         TEXT,
    capital_burg    TEXT,
    citadel         TEXT,
    culture         TEXT,
    religion        TEXT,
    emblem          TEXT,
    emblem_path     TEXT,
    banner          TEXT,
    coa_description TEXT,
    coa_primary_color TEXT,
    coa_shield      TEXT,
    founding_date   TEXT,
    founding_era    TEXT,
    founding_moment TEXT,           -- wikilink to the_chronicles moment
    destruction_moment TEXT,        -- wikilink to the_chronicles moment
    settlement_types TEXT,
    neighbor_burgs  TEXT,
    notable_builders TEXT,
    architectural_materials TEXT,

    -- Burg infrastructure
    burg_capital_level INTEGER,
    burg_citadel_level INTEGER,
    burg_has_citadel BOOLEAN,
    burg_has_plaza  BOOLEAN,
    burg_has_shanty BOOLEAN,
    burg_has_temple BOOLEAN,
    burg_has_walls  BOOLEAN,
    burg_is_capital BOOLEAN,
    burg_is_port    BOOLEAN,
    burg_name       TEXT,
    burg_cell       INTEGER,
    burg_population INTEGER,
    burg_port_id    INTEGER,
    burg_plaza_level INTEGER,
    burg_shanty_level INTEGER,
    burg_temple_level INTEGER,
    burg_type       TEXT,
    burg_walls_level INTEGER,
    plaza           TEXT,
    temple          TEXT,
    walls           TEXT,
    port            TEXT,

    -- River fields
    river_name      TEXT,
    river_type      TEXT,
    river_length    NUMERIC,
    river_discharge NUMERIC,
    river_width     NUMERIC,
    river_source_cell INTEGER,
    river_mouth_cell INTEGER,
    river_waypoints_count INTEGER,
    river_basin     TEXT,
    river_parent    TEXT,

    -- Route fields
    route_name      TEXT,
    route_group     TEXT,
    route_length    NUMERIC,
    route_waypoints_count INTEGER,
    route_feature   TEXT,

    -- POI marker fields
    marker_type     TEXT,
    icon_type       TEXT,
    notes           TEXT,
    afwg_marker_id  INTEGER,

    -- NPC/Creature/Dragon fields
    ac              INTEGER,
    hp              TEXT,
    cr              TEXT,
    level           INTEGER,
    class           TEXT,
    race            TEXT,
    alignment       TEXT,
    subtype         TEXT,
    abilities       TEXT,
    traits          TEXT,
    skills          TEXT,
    languages       TEXT,
    saves           TEXT,
    senses          TEXT,
    speed           TEXT,
    hit_dice        TEXT,
    lineage         TEXT,
    background      TEXT,
    origin_story    TEXT,
    hometown        TEXT,

    -- Dragon-specific
    dragon          TEXT,
    dragon_type     TEXT,
    dragon_relations TEXT,
    lair            TEXT,

    -- Faction fields
    faction         TEXT,           -- wikilink
    faction_link    TEXT,
    faction_relationships TEXT,
    public_affiliation TEXT,
    secret_affiliation TEXT,
    diplomatic_status TEXT,
    overall_stance  TEXT,
    diplomacy_allies TEXT,
    diplomacy_enemies TEXT,
    diplomacy_neutral TEXT,
    diplomacy_suspicion TEXT,
    diplomacy_suzerain TEXT,
    diplomacy_unknown TEXT,
    diplomacy_vassal TEXT,

    -- NPC lifecycle (wikilinks to the_chronicles)
    birth_moment    TEXT,
    death_moment    TEXT,

    -- Military (state-level)
    military_strength TEXT,
    military_total_units INTEGER,
    military_regiments_count INTEGER,
    military_infantry INTEGER,
    military_archers INTEGER,
    military_cavalry INTEGER,
    military_artillery INTEGER,
    military_fleet  INTEGER,
    captain         TEXT,

    -- Resources & trade
    primary_resources TEXT,
    secondary_resources TEXT,
    trade_goods     TEXT,
    common_fauna    TEXT,
    common_flora    TEXT,
    survival_challenges TEXT,
    travel_methods  TEXT,
    primary_threat_level TEXT,
    cross_realm_relations TEXT,

    -- Map references
    json_id         INTEGER,
    json_state_id   INTEGER,
    json_province_id INTEGER,
    json_feature_id INTEGER,
    map_width_x     NUMERIC,
    map_height_y    NUMERIC,
    scale_pixels    NUMERIC,
    scale_pixels_range TEXT,
    watabou_seed    TEXT,
    city_generator_link TEXT,
    foundry_link    TEXT,
    dnd_link        TEXT,

    -- Meta
    aliases         TEXT,
    tags            TEXT,
    description     TEXT,
    ai_summary      TEXT,
    significance    TEXT,
    cultural_impact TEXT,
    historical_context TEXT,
    tasks_description TEXT,
    sources         TEXT,
    ceo             TEXT,
    department_head TEXT,
    geographic_feature TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_realms IS 'Geography (State→Province→Burg→District→POI) + Entities (NPCs, Creatures, Dragons, Factions). Azgaar data + Orbis worldbuilding.';

-- Indexes
CREATE INDEX idx_realms_kind            ON the_realms(kind);
CREATE INDEX idx_realms_status          ON the_realms(status);
CREATE INDEX idx_realms_parent_state    ON the_realms(parent_state);
CREATE INDEX idx_realms_parent_province ON the_realms(parent_province);
CREATE INDEX idx_realms_parent_burg     ON the_realms(parent_burg);
CREATE INDEX idx_realms_state_full      ON the_realms(state_full);
CREATE INDEX idx_realms_culture         ON the_realms(culture);
CREATE INDEX idx_realms_religion        ON the_realms(religion);
CREATE INDEX idx_realms_faction         ON the_realms(faction);
CREATE INDEX idx_realms_race            ON the_realms(race);
CREATE INDEX idx_realms_json_id         ON the_realms(json_id);
CREATE INDEX idx_realms_json_state_id   ON the_realms(json_state_id);
CREATE INDEX idx_realms_cell            ON the_realms(cell);

CREATE INDEX idx_realms_fts ON the_realms USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(description, '') || ' ' ||
        coalesce(origin_story, '')
    )
);
