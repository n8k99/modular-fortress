-- Wikilink extraction: the ligaments of the noosphere
-- Scans every TEXT column in a row for [[slug]] and [[path|display]] patterns,
-- writes each found link to the_links table.
-- Triggers fire on INSERT and UPDATE for every domain table.

-- =========================================================================
-- Core extraction function: given a table name, row id, column name, and
-- text value, extract all [[...]] patterns and INSERT into the_links.
-- =========================================================================
CREATE OR REPLACE FUNCTION extract_wikilinks(
    p_source_table TEXT,
    p_source_id    BIGINT,
    p_source_field TEXT,
    p_text         TEXT
) RETURNS INTEGER AS $$
DECLARE
    v_match     TEXT;
    v_slug      TEXT;
    v_display   TEXT;
    v_count     INTEGER := 0;
BEGIN
    IF p_text IS NULL THEN
        RETURN 0;
    END IF;

    -- Find all [[...]] patterns
    FOR v_match IN
        SELECT (regexp_matches(p_text, '\[\[([^\]]+)\]\]', 'g'))[1]
    LOOP
        -- Handle [[path|display]] syntax
        IF position('|' in v_match) > 0 THEN
            v_slug    := split_part(v_match, '|', 1);
            v_display := split_part(v_match, '|', 2);
        ELSE
            v_slug    := v_match;
            v_display := NULL;
        END IF;

        -- Strip any path prefix (Orbis/ReligiousOrders/deities/greek-zeus → greek-zeus)
        -- But keep the full path in link_text for context
        IF position('/' in v_slug) > 0 THEN
            v_display := COALESCE(v_display, v_slug);  -- preserve full path as display
            v_slug    := split_part(v_slug, '/', -1);   -- use last segment as slug
            -- PostgreSQL split_part doesn't support negative index, use reverse approach
            v_slug    := reverse(split_part(reverse(v_match), '/', 1));
            IF position('|' in v_slug) > 0 THEN
                v_slug := split_part(v_slug, '|', 1);
            END IF;
        END IF;

        -- Clean slug: trim whitespace
        v_slug := trim(v_slug);

        IF v_slug != '' THEN
            INSERT INTO the_links (source_table, source_id, source_field, target_slug, link_text)
            VALUES (p_source_table, p_source_id, p_source_field, v_slug, v_display);
            v_count := v_count + 1;
        END IF;
    END LOOP;

    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION extract_wikilinks IS 'Extract [[wikilinks]] from a text value and INSERT into the_links. Handles [[slug]], [[path|display]], and [[path/to/slug]] patterns.';


-- =========================================================================
-- Generic trigger function: scans ALL text columns of the triggering row,
-- calls extract_wikilinks for each non-null text column.
-- =========================================================================
CREATE OR REPLACE FUNCTION trg_extract_wikilinks()
RETURNS TRIGGER AS $$
DECLARE
    v_col       RECORD;
    v_value     TEXT;
    v_table     TEXT := TG_TABLE_NAME;
    v_id        BIGINT;
    v_total     INTEGER := 0;
BEGIN
    -- Get the row id
    v_id := NEW.id;

    -- On UPDATE, clear old links for this row first
    IF TG_OP = 'UPDATE' THEN
        DELETE FROM the_links
        WHERE source_table = v_table AND source_id = v_id;
    END IF;

    -- Iterate over all TEXT columns in this table
    FOR v_col IN
        SELECT column_name
        FROM information_schema.columns
        WHERE table_name = v_table
          AND table_schema = 'public'
          AND data_type = 'text'
          AND column_name NOT IN ('created_at', 'updated_at')  -- skip timestamp-like
        ORDER BY ordinal_position
    LOOP
        -- Dynamically get the column value from NEW
        EXECUTE format('SELECT ($1).%I', v_col.column_name)
            INTO v_value
            USING NEW;

        IF v_value IS NOT NULL AND v_value LIKE '%[[%]]%' THEN
            v_total := v_total + extract_wikilinks(v_table, v_id, v_col.column_name, v_value);
        END IF;
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION trg_extract_wikilinks IS 'Trigger function: scans all TEXT columns for [[wikilinks]], writes to the_links. Clears old links on UPDATE.';


-- =========================================================================
-- Attach triggers to ALL domain tables
-- =========================================================================

-- Identity
DROP TRIGGER IF EXISTS trg_identity_wikilinks ON identity;
CREATE TRIGGER trg_identity_wikilinks
    AFTER INSERT OR UPDATE ON identity
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- Temporal
DROP TRIGGER IF EXISTS trg_temporal_wikilinks ON temporal;
CREATE TRIGGER trg_temporal_wikilinks
    AFTER INSERT OR UPDATE ON temporal
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_work
DROP TRIGGER IF EXISTS trg_work_wikilinks ON the_work;
CREATE TRIGGER trg_work_wikilinks
    AFTER INSERT OR UPDATE ON the_work
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_commons
DROP TRIGGER IF EXISTS trg_commons_wikilinks ON the_commons;
CREATE TRIGGER trg_commons_wikilinks
    AFTER INSERT OR UPDATE ON the_commons
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_chronicles
DROP TRIGGER IF EXISTS trg_chronicles_wikilinks ON the_chronicles;
CREATE TRIGGER trg_chronicles_wikilinks
    AFTER INSERT OR UPDATE ON the_chronicles
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_realms
DROP TRIGGER IF EXISTS trg_realms_wikilinks ON the_realms;
CREATE TRIGGER trg_realms_wikilinks
    AFTER INSERT OR UPDATE ON the_realms
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_music
DROP TRIGGER IF EXISTS trg_music_wikilinks ON the_music;
CREATE TRIGGER trg_music_wikilinks
    AFTER INSERT OR UPDATE ON the_music
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_post
DROP TRIGGER IF EXISTS trg_post_wikilinks ON the_post;
CREATE TRIGGER trg_post_wikilinks
    AFTER INSERT OR UPDATE ON the_post
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_press
DROP TRIGGER IF EXISTS trg_press_wikilinks ON the_press;
CREATE TRIGGER trg_press_wikilinks
    AFTER INSERT OR UPDATE ON the_press
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();

-- the_markets
DROP TRIGGER IF EXISTS trg_markets_wikilinks ON the_markets;
CREATE TRIGGER trg_markets_wikilinks
    AFTER INSERT OR UPDATE ON the_markets
    FOR EACH ROW
    EXECUTE FUNCTION trg_extract_wikilinks();
