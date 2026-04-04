-- 06_the_music.sql
-- Living Room Music — podcast, musicology, episodes, audio analysis

CREATE TABLE the_music (
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

COMMENT ON TABLE the_music IS 'Living Room Music — podcast, musicology, episodes, audio analysis. Owner: L.R. Morgenstern, Head of Musicology. Audio files live in The Commons, metadata here.';

-- Kind taxonomy:
-- episode         Podcast episode
-- track           Music track / composition
-- album           Album or collection
-- corpus_entry    LRM corpus entry (musicological analysis)
-- analysis        Audio analysis result (chroma, spectral)
-- show_note       Episode show notes
-- playlist        Curated playlist
-- wave_entry      Wave calendar entry
-- venue           Performance venue

-- meta for episode:
--   episode_number, season, duration, guests[], topics[]
--   audio_ref: [[commons:audio-slug]]  (points to file in The Commons)

-- meta for corpus_entry:
--   composer, genre, period, key, tempo, instrumentation[]

-- meta for analysis:
--   audio_ref, analysis_type (chroma/spectral/features)
--   features: {} (chroma vectors, spectral data as JSONB)

CREATE INDEX idx_music_kind ON the_music(kind);
CREATE INDEX idx_music_status ON the_music(status);
CREATE INDEX idx_music_meta ON the_music USING gin(meta);
CREATE INDEX idx_music_created ON the_music(created_at);
CREATE INDEX idx_music_slug_trgm ON the_music USING gin(slug gin_trgm_ops);
CREATE INDEX idx_music_title_trgm ON the_music USING gin(title gin_trgm_ops);

CREATE TRIGGER trg_music_updated_at
    BEFORE UPDATE ON the_music
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
