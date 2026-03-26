-- Migration: stage_notes TEXT -> JSONB
-- Phase 7, Plan 01: Structured Artifact Passing
-- Wraps existing freeform text in {legacy_text, schema_version: 0} per D-05
-- Preserves NULLs and empty strings as NULL

ALTER TABLE tasks
  ALTER COLUMN stage_notes TYPE JSONB
  USING CASE
    WHEN stage_notes IS NOT NULL AND stage_notes != ''
    THEN jsonb_build_object('legacy_text', stage_notes, 'schema_version', 0)
    ELSE NULL
  END;

COMMENT ON COLUMN tasks.stage_notes IS 'JSONB artifact: schema_version 0=legacy wrapped text, 1=structured artifact';
