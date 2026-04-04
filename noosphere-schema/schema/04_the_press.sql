-- 04_the_press.sql
-- Thought Police — editorials, publishing pipeline, executive blogging

CREATE TABLE the_press (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'draft',
    published_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_press IS 'Thought Police — editorials, research-backed publishing, hero images, executive blogging. Pipeline: research → editorial (Sylvia) → hero image (Vincent) → publish.';

-- Kind taxonomy:
-- editorial       Long-form editorial piece
-- blog_post       Executive blog post for em-site
-- newsletter      Newsletter edition
-- research_brief  Research backing for a post (consumed from The Commons)
-- pitch           Proposed article idea
-- hero_image_req  Request to Vincent for DPN-style hero image
-- published       Published article (final, on Ghost CMS or em-site)

-- meta for editorial:
--   author: [[Sylvia]]
--   artist: [[Vincent]]
--   research_refs: [[[research-slug-1]], [[research-slug-2]]]
--   publish_target: ghost_cms | em_site
--   pipeline_stage: draft | review | image | ready | published

CREATE INDEX idx_press_kind ON the_press(kind);
CREATE INDEX idx_press_status ON the_press(status);
CREATE INDEX idx_press_meta ON the_press USING gin(meta);
CREATE INDEX idx_press_published ON the_press(published_at) WHERE published_at IS NOT NULL;
CREATE INDEX idx_press_created ON the_press(created_at);
CREATE INDEX idx_press_slug_trgm ON the_press USING gin(slug gin_trgm_ops);
CREATE INDEX idx_press_title_trgm ON the_press USING gin(title gin_trgm_ops);

CREATE TRIGGER trg_press_updated_at
    BEFORE UPDATE ON the_press
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
