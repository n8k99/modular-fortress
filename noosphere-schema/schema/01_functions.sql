-- 01_functions.sql
-- Shared utility functions for the Nine Tables
-- Ghosts in the Noosphere v2.0

-- Auto-update updated_at on row modification
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Extract [[wikilinks]] from text content
-- Returns rows of (target, display_text)
-- Handles: [[Target]], [[Target|Display]], [[Target#Heading]], [[Target#Heading|Display]]
-- Also handles qualified: [[forge:Nova]], [[realms:Archonate]]
CREATE OR REPLACE FUNCTION extract_wikilinks(content TEXT)
RETURNS TABLE(target TEXT, display_text TEXT, qualifier TEXT) AS $$
BEGIN
    RETURN QUERY
    SELECT
        TRIM(m[1])::TEXT AS target,
        TRIM(NULLIF(m[3], ''))::TEXT AS display_text,
        TRIM(NULLIF(m[4], ''))::TEXT AS qualifier
    FROM regexp_matches(
        content,
        '\[\[(?:([a-z]+):)?([^\]#|]+)(?:#[^\]|]+)?(?:\|([^\]]+))?\]\]',
        'g'
    ) AS m;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Slugify a title into a URL-safe slug
CREATE OR REPLACE FUNCTION slugify(title TEXT)
RETURNS TEXT AS $$
BEGIN
    RETURN LOWER(
        REGEXP_REPLACE(
            REGEXP_REPLACE(
                TRIM(title),
                '[^a-zA-Z0-9\s-]', '', 'g'
            ),
            '\s+', '-', 'g'
        )
    );
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Extract all wikilinks from JSONB string values (recursive)
-- Walks all string values in a JSONB object/array and extracts [[wikilinks]]
CREATE OR REPLACE FUNCTION extract_wikilinks_from_jsonb(data JSONB)
RETURNS TABLE(target TEXT, field_name TEXT) AS $$
DECLARE
    key TEXT;
    val JSONB;
    elem JSONB;
    str_val TEXT;
    match_row RECORD;
BEGIN
    IF jsonb_typeof(data) = 'object' THEN
        FOR key, val IN SELECT * FROM jsonb_each(data) LOOP
            IF jsonb_typeof(val) = 'string' THEN
                str_val := val #>> '{}';
                FOR match_row IN SELECT * FROM extract_wikilinks(str_val) LOOP
                    target := match_row.target;
                    field_name := key;
                    RETURN NEXT;
                END LOOP;
            ELSIF jsonb_typeof(val) IN ('object', 'array') THEN
                FOR match_row IN SELECT * FROM extract_wikilinks_from_jsonb(val) LOOP
                    target := match_row.target;
                    field_name := key;
                    RETURN NEXT;
                END LOOP;
            END IF;
        END LOOP;
    ELSIF jsonb_typeof(data) = 'array' THEN
        FOR elem IN SELECT * FROM jsonb_array_elements(data) LOOP
            IF jsonb_typeof(elem) = 'string' THEN
                str_val := elem #>> '{}';
                FOR match_row IN SELECT * FROM extract_wikilinks(str_val) LOOP
                    target := match_row.target;
                    field_name := 'array_element';
                    RETURN NEXT;
                END LOOP;
            ELSIF jsonb_typeof(elem) IN ('object', 'array') THEN
                FOR match_row IN SELECT * FROM extract_wikilinks_from_jsonb(elem) LOOP
                    target := match_row.target;
                    field_name := match_row.field_name;
                    RETURN NEXT;
                END LOOP;
            END IF;
        END LOOP;
    END IF;
END;
$$ LANGUAGE plpgsql IMMUTABLE;
