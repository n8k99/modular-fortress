-- 00_extensions.sql
-- Required PostgreSQL extensions for the Nine Tables schema.

-- Trigram matching for fuzzy slug/title search
CREATE EXTENSION IF NOT EXISTS pg_trgm;
