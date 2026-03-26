-- Migration: Add wave advancement + project completion to on_task_completed_after trigger
-- Applied: 2026-03-26
-- Phase: 05-feedback-reporting, Plan 01, Task 1
--
-- This replaces the existing trigger function, preserving all original logic
-- (vault_notes daily note update, pg_notify task_completed) and adding:
--   1. Wave advancement: when all wave-N tasks complete, open wave N+1
--   2. Project completion: when all project tasks complete, mark project 'completed' + notify Nathan

CREATE OR REPLACE FUNCTION on_task_completed_after()
RETURNS trigger AS $$
DECLARE
    today_path text;
    agent_name text;
    wikilink text;
    completion_line text;
    rows_affected int;
    current_wave int;
    remaining int;
    next_wave int;
    advanced int;
    remaining_tasks int;
    proj_name varchar;
    proj_owner varchar;
BEGIN
    IF NEW.status NOT IN ('done', 'completed') THEN RETURN NEW; END IF;
    IF OLD.status = NEW.status THEN RETURN NEW; END IF;

    agent_name := COALESCE(NEW.assignee, NEW.assigned_to[1], 'unknown');
    wikilink := '[[' || replace(initcap(replace(agent_name, '_', ' ')), ' ', '') || ']]';
    completion_line := '- ✅ ' || NEW.text || ' — ' || wikilink;

    today_path := 'Areas/N8K99Notes/Daily Notes/' || to_char(NOW() AT TIME ZONE 'America/New_York', 'YYYY-MM-DD') || '.md';

    UPDATE vault_notes
    SET content = replace(
        content,
        E'## 🎯 What I Did Today\n\n',
        E'## 🎯 What I Did Today\n' || completion_line || E'\n\n'
    ),
    modified_at = NOW()
    WHERE path = today_path;

    GET DIAGNOSTICS rows_affected = ROW_COUNT;
    RAISE NOTICE 'Task completed trigger: updated % rows in vault_notes for path %', rows_affected, today_path;

    PERFORM pg_notify('task_completed', json_build_object(
        'task_id', NEW.id,
        'agent_id', agent_name,
        'text', NEW.text
    )::text);

    -- Wave advancement: when all tasks in wave N complete, open wave N+1
    IF NEW.context IS NOT NULL AND NEW.project_id IS NOT NULL THEN
        BEGIN
            current_wave := (NEW.context::jsonb->>'wave')::int;
        EXCEPTION WHEN OTHERS THEN
            current_wave := NULL;
        END;
        IF current_wave IS NOT NULL THEN
            SELECT COUNT(*) INTO remaining
            FROM tasks
            WHERE project_id = NEW.project_id
              AND id != NEW.id
              AND context IS NOT NULL
              AND (context::jsonb->>'wave')::int = current_wave
              AND status NOT IN ('done', 'completed');
            IF remaining = 0 THEN
                next_wave := current_wave + 1;
                UPDATE tasks
                SET status = 'open', updated_at = NOW()
                WHERE project_id = NEW.project_id
                  AND context IS NOT NULL
                  AND (context::jsonb->>'wave')::int = next_wave
                  AND status IN ('blocked', 'pending');
                GET DIAGNOSTICS advanced = ROW_COUNT;
                IF advanced > 0 THEN
                    PERFORM pg_notify('wave_advanced', json_build_object(
                        'project_id', NEW.project_id,
                        'completed_wave', current_wave,
                        'next_wave', next_wave,
                        'tasks_advanced', advanced
                    )::text);
                END IF;
            END IF;
        END IF;
    END IF;

    -- Project completion: update project status + notify Nathan
    IF NEW.project_id IS NOT NULL THEN
        SELECT COUNT(*) INTO remaining_tasks
        FROM tasks
        WHERE project_id = NEW.project_id
          AND id != NEW.id
          AND status NOT IN ('done', 'completed');
        IF remaining_tasks = 0 THEN
            SELECT name, owner INTO proj_name, proj_owner
            FROM projects WHERE id = NEW.project_id;
            UPDATE projects SET status = 'completed', updated_at = NOW()
            WHERE id = NEW.project_id;
            -- Notify Nathan via conversations table
            INSERT INTO conversations (from_agent, to_agent, message, channel, message_type, metadata)
            VALUES (
                COALESCE(proj_owner, 'system'),
                ARRAY['nathan'],
                '[Project Complete] ' || COALESCE(proj_name, 'Unknown') || ' -- all tasks done.',
                'noosphere',
                'project_complete',
                json_build_object('source', 'project_completion', 'project_id', NEW.project_id)::jsonb
            );
            PERFORM pg_notify('project_completed', json_build_object(
                'project_id', NEW.project_id,
                'name', proj_name
            )::text);
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
