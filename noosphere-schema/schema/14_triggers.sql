-- 14_triggers.sql
-- Wikilink extraction triggers for all 9 domain tables.
-- On INSERT/UPDATE, extracts [[wikilinks]] from body and meta,
-- populates the_links table for instant backlink queries.

-- Generic trigger function: extracts wikilinks and populates the_links
CREATE OR REPLACE FUNCTION sync_wikilinks()
RETURNS TRIGGER AS $$
DECLARE
    tbl TEXT := TG_TABLE_NAME;
    link_row RECORD;
BEGIN
    -- Delete existing links for this source
    DELETE FROM the_links
    WHERE source_table = tbl AND source_id = NEW.id;

    -- Extract from body text
    IF NEW.body IS NOT NULL AND NEW.body != '' THEN
        FOR link_row IN
            SELECT target, display_text, qualifier
            FROM extract_wikilinks(NEW.body)
        LOOP
            INSERT INTO the_links (source_table, source_id, source_slug, target_slug,
                                   link_context, qualifier, display_text)
            VALUES (tbl, NEW.id, NEW.slug, slugify(link_row.target),
                    'body', link_row.qualifier, link_row.display_text);
        END LOOP;
    END IF;

    -- Extract from meta JSONB
    IF NEW.meta IS NOT NULL AND NEW.meta != '{}'::jsonb THEN
        FOR link_row IN
            SELECT target, field_name
            FROM extract_wikilinks_from_jsonb(NEW.meta)
        LOOP
            INSERT INTO the_links (source_table, source_id, source_slug, target_slug,
                                   link_context)
            VALUES (tbl, NEW.id, NEW.slug, slugify(link_row.target),
                    link_row.field_name);
        END LOOP;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to all 9 domain tables
-- Note: the_post is append-only for messages, but we still extract links
-- from reports and handoffs that may contain [[wikilinks]]

CREATE TRIGGER trg_chronicles_links
    AFTER INSERT OR UPDATE ON the_chronicles
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

CREATE TRIGGER trg_realms_links
    AFTER INSERT OR UPDATE ON the_realms
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

CREATE TRIGGER trg_press_links
    AFTER INSERT OR UPDATE ON the_press
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

CREATE TRIGGER trg_markets_links
    AFTER INSERT OR UPDATE ON the_markets
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

CREATE TRIGGER trg_music_links
    AFTER INSERT OR UPDATE ON the_music
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

-- the_forge: skip link extraction for append-only high-volume kinds
CREATE OR REPLACE FUNCTION sync_wikilinks_forge()
RETURNS TRIGGER AS $$
BEGIN
    -- Skip link extraction for high-volume append-only kinds
    IF NEW.kind IN ('fitness_event', 'cognition_job') THEN
        RETURN NEW;
    END IF;

    -- Delegate to generic function
    DELETE FROM the_links
    WHERE source_table = 'the_forge' AND source_id = NEW.id;

    IF NEW.body IS NOT NULL AND NEW.body != '' THEN
        INSERT INTO the_links (source_table, source_id, source_slug, target_slug,
                               link_context, qualifier, display_text)
        SELECT 'the_forge', NEW.id, NEW.slug, slugify(target),
               'body', qualifier, display_text
        FROM extract_wikilinks(NEW.body);
    END IF;

    IF NEW.meta IS NOT NULL AND NEW.meta != '{}'::jsonb THEN
        INSERT INTO the_links (source_table, source_id, source_slug, target_slug,
                               link_context)
        SELECT 'the_forge', NEW.id, NEW.slug, slugify(target),
               field_name
        FROM extract_wikilinks_from_jsonb(NEW.meta);
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_forge_links
    AFTER INSERT OR UPDATE ON the_forge
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks_forge();

CREATE TRIGGER trg_commons_links
    AFTER INSERT OR UPDATE ON the_commons
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

CREATE TRIGGER trg_work_links
    AFTER INSERT OR UPDATE ON the_work
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks();

-- the_post: only extract links from reports and handoffs
CREATE OR REPLACE FUNCTION sync_wikilinks_post()
RETURNS TRIGGER AS $$
BEGIN
    -- Skip link extraction for short chat messages
    IF NEW.kind = 'chat' THEN
        RETURN NEW;
    END IF;

    DELETE FROM the_links
    WHERE source_table = 'the_post' AND source_id = NEW.id;

    IF NEW.body IS NOT NULL AND NEW.body != '' THEN
        INSERT INTO the_links (source_table, source_id, source_slug, target_slug,
                               link_context, qualifier, display_text)
        SELECT 'the_post', NEW.id, NEW.slug, slugify(target),
               'body', qualifier, display_text
        FROM extract_wikilinks(NEW.body);
    END IF;

    IF NEW.meta IS NOT NULL AND NEW.meta != '{}'::jsonb THEN
        INSERT INTO the_links (source_table, source_id, source_slug, target_slug,
                               link_context)
        SELECT 'the_post', NEW.id, NEW.slug, slugify(target),
               field_name
        FROM extract_wikilinks_from_jsonb(NEW.meta);
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_post_links
    AFTER INSERT OR UPDATE ON the_post
    FOR EACH ROW EXECUTE FUNCTION sync_wikilinks_post();
