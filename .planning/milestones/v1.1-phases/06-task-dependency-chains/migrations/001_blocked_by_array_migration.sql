-- Migration: blocked_by INTEGER -> INTEGER[] (per D-01, D-02)
-- Preserves existing data: single INTEGER values become single-element arrays

ALTER TABLE tasks
  ALTER COLUMN blocked_by TYPE INTEGER[]
  USING CASE WHEN blocked_by IS NOT NULL THEN ARRAY[blocked_by] ELSE NULL END;

CREATE INDEX idx_tasks_blocked_by ON tasks USING GIN (blocked_by);
