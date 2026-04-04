-- Search index triggers: populate the_index on INSERT/UPDATE of domain tables.
-- One unified search across the entire noosphere.

CREATE OR REPLACE FUNCTION trg_update_search_index()
RETURNS TRIGGER AS $$
DECLARE
    v_table     TEXT := TG_TABLE_NAME;
    v_id        BIGINT := NEW.id;
    v_slug      TEXT;
    v_kind      TEXT;
    v_title     TEXT;
    v_body      TEXT;
    v_desc      TEXT;
    v_vector    tsvector;
BEGIN
    -- Get core fields (all domain tables have these)
    EXECUTE format('SELECT ($1).slug')  INTO v_slug  USING NEW;
    EXECUTE format('SELECT ($1).title') INTO v_title USING NEW;

    -- Try to get optional fields (may not exist on all tables)
    BEGIN
        EXECUTE format('SELECT ($1).kind') INTO v_kind USING NEW;
    EXCEPTION WHEN undefined_column THEN
        v_kind := NULL;
    END;

    BEGIN
        EXECUTE format('SELECT ($1).body') INTO v_body USING NEW;
    EXCEPTION WHEN undefined_column THEN
        v_body := NULL;
    END;

    BEGIN
        EXECUTE format('SELECT ($1).description') INTO v_desc USING NEW;
    EXCEPTION WHEN undefined_column THEN
        v_desc := NULL;
    END;

    -- Build search vector: title weighted A, description B, body C
    v_vector :=
        setweight(to_tsvector('english', coalesce(v_title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(v_slug, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(v_desc, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(v_body, '')), 'C');

    -- Upsert into the_index
    INSERT INTO the_index (source_table, source_id, source_slug, source_kind, source_title, search_vector, updated_at)
    VALUES (v_table, v_id, COALESCE(v_slug, ''), v_kind, v_title, v_vector, NOW())
    ON CONFLICT (source_table, source_id)
    DO UPDATE SET
        source_slug   = EXCLUDED.source_slug,
        source_kind   = EXCLUDED.source_kind,
        source_title  = EXCLUDED.source_title,
        search_vector = EXCLUDED.search_vector,
        updated_at    = NOW();

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION trg_update_search_index IS 'Trigger: upsert row into the_index with weighted tsvector (title=A, description=B, body=C).';

-- Attach to all domain tables
DROP TRIGGER IF EXISTS trg_identity_search ON identity;
CREATE TRIGGER trg_identity_search
    AFTER INSERT OR UPDATE ON identity
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_temporal_search ON temporal;
CREATE TRIGGER trg_temporal_search
    AFTER INSERT OR UPDATE ON temporal
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_work_search ON the_work;
CREATE TRIGGER trg_work_search
    AFTER INSERT OR UPDATE ON the_work
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_commons_search ON the_commons;
CREATE TRIGGER trg_commons_search
    AFTER INSERT OR UPDATE ON the_commons
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_chronicles_search ON the_chronicles;
CREATE TRIGGER trg_chronicles_search
    AFTER INSERT OR UPDATE ON the_chronicles
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_realms_search ON the_realms;
CREATE TRIGGER trg_realms_search
    AFTER INSERT OR UPDATE ON the_realms
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_music_search ON the_music;
CREATE TRIGGER trg_music_search
    AFTER INSERT OR UPDATE ON the_music
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_post_search ON the_post;
CREATE TRIGGER trg_post_search
    AFTER INSERT OR UPDATE ON the_post
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_press_search ON the_press;
CREATE TRIGGER trg_press_search
    AFTER INSERT OR UPDATE ON the_press
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();

DROP TRIGGER IF EXISTS trg_markets_search ON the_markets;
CREATE TRIGGER trg_markets_search
    AFTER INSERT OR UPDATE ON the_markets
    FOR EACH ROW EXECUTE FUNCTION trg_update_search_index();
