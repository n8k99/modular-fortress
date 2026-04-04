-- 09_the_work.sql
-- @Tasks — connective tissue. Every task points at a domain. Flat and universal.

CREATE TABLE the_work (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'open',
    domain      TEXT,
    assigned    TEXT[],
    priority    TEXT NOT NULL DEFAULT 'normal',
    deadline    TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_work IS '@Tasks — connective tissue. Every task points at a domain. Flat and universal. Ghosts perceive work from here.';

-- Kind taxonomy:
-- task         Work item. meta: department, stage, pipeline_slug, pipeline_order,
--              project_slug, source, context, parent_slug, blocked_by[] (slug array),
--              stage_notes (JSONB), scheduled_at, recurrence, completed_at
-- goal         Strategic goal. meta: owner, target_date, progress_pct, milestones[]
-- routine      Recurring routine. meta: schedule (cron), last_run, next_run, owner
-- issue        Bug or problem. meta: severity (critical/high/medium/low), reporter,
--              affected_system, resolution
-- request      Agent request. meta: from_agent, request_type, context

-- status values: open, pending, in-progress, blocked, done, completed, cancelled

-- domain column: which sovereign domain this work belongs to
-- e.g., domain='the_markets' for a trading task
-- Tasks without a domain are infrastructure/cross-cutting

-- assigned column: TEXT[] of agent_ids
-- Supports multi-agent assignment
-- GIN indexed for "find tasks assigned to agent X"

-- meta.blocked_by: JSONB array of task slugs that must complete first
-- Resolution: JOIN the_work ON slug to check if blockers are done

CREATE INDEX idx_work_kind ON the_work(kind);
CREATE INDEX idx_work_status ON the_work(status);
CREATE INDEX idx_work_domain ON the_work(domain) WHERE domain IS NOT NULL;
CREATE INDEX idx_work_assigned ON the_work USING gin(assigned);
CREATE INDEX idx_work_priority ON the_work(priority);
CREATE INDEX idx_work_deadline ON the_work(deadline) WHERE deadline IS NOT NULL;
CREATE INDEX idx_work_meta ON the_work USING gin(meta);
CREATE INDEX idx_work_created ON the_work(created_at);

-- Perception hot paths
CREATE INDEX idx_work_active ON the_work(kind, status)
    WHERE status IN ('open', 'pending', 'in-progress', 'blocked');
CREATE INDEX idx_work_dept ON the_work((meta->>'department'))
    WHERE meta->>'department' IS NOT NULL;
CREATE INDEX idx_work_project ON the_work((meta->>'project_slug'))
    WHERE meta->>'project_slug' IS NOT NULL;
CREATE INDEX idx_work_pipeline ON the_work((meta->>'pipeline_slug'))
    WHERE meta->>'pipeline_slug' IS NOT NULL;

CREATE INDEX idx_work_slug_trgm ON the_work USING gin(slug gin_trgm_ops);
CREATE INDEX idx_work_title_trgm ON the_work USING gin(title gin_trgm_ops);

CREATE TRIGGER trg_work_updated_at
    BEFORE UPDATE ON the_work
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
