-- 07_the_forge.sql
-- THE META-DOMAIN — ghost agents, memories, tick engine, infrastructure,
-- N8K99 vault notes, pipelines, projects, decisions, Innate templates.
-- Where Nathan works AND where ghosts live.

CREATE TABLE the_forge (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL,
    kind        TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    agent_id    TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE the_forge IS 'Digital Sovereignty — the meta-domain. Ghost agents, memories, tick engine state, infrastructure, N8K99 vault notes, pipeline definitions, projects, decisions, Innate templates. AGPL — no enclosure.';

-- Kind taxonomy:
--
-- AGENT IDENTITY & STATE (one row per ghost)
-- agent            Ghost identity + runtime state. meta contains:
--                  full_name, role, department, reports_to: [[X]], mentor: [[X]],
--                  collaborators: [[[X]], [[Y]]], responsibilities[], goals[],
--                  energy, tier, agent_tier, ticks_alive, ticks_at_current_tier,
--                  last_tick_at, tool_scope[], has_tools, avatar,
--                  drives: { curiosity: {satisfaction, pressure, frustration, decay_rate}, ... }
--
-- TICK ENGINE
-- cognition_job    Pending/completed reasoning request
-- (tick_log and tick_report moved to the_ledger — invisible infrastructure)
--
-- MEMORY
-- memory_daily     Daily memory per agent. meta: actions_taken, decisions_made,
--                  knowledge_gained, blockers, handoffs, plan_tomorrow
-- memory_entry     Long-term agent memory (semantic)
-- vault_note       N8K99 operational note (Nathan's daily/weekly notes)
--
-- DECISIONS & EVOLUTION
-- decision         Strategic decision. meta: rationale, stakeholders, department, project_slug
-- fitness_event    Agent fitness score event. meta: outcome, score, context
-- persona_mutation Ghost persona change record
-- metamorphosis    Ghost evolution event
--
-- INFRASTRUCTURE
-- project          Project registry. meta: owner, goals[], blockers[], current_context,
--                  schedule, lifestage, area_slug, open_tasks (computed)
-- area             Organizational area (EM Corp, Orbis, etc.)
-- pipeline         Pipeline definition. meta: stages[] (ordered slug list),
--                  description, owner, active
-- pipeline_stage   Stage within a pipeline. meta: order, pipeline_slug,
--                  assigned: [[Ghost]], next_stage, tools_required, description
-- codebase_scan    Infrastructure scan result
-- config           System configuration entry

-- Slug uniqueness: only for named entities, not append-only kinds
CREATE UNIQUE INDEX idx_forge_slug_unique
    ON the_forge(slug)
    WHERE kind NOT IN ('fitness_event', 'memory_daily', 'cognition_job');

-- Core indexes
CREATE INDEX idx_forge_kind ON the_forge(kind);
CREATE INDEX idx_forge_status ON the_forge(status);
CREATE INDEX idx_forge_agent ON the_forge(agent_id) WHERE agent_id IS NOT NULL;
CREATE INDEX idx_forge_meta ON the_forge USING gin(meta);
CREATE INDEX idx_forge_created ON the_forge(created_at);

-- Composite indexes for perception hot paths
CREATE INDEX idx_forge_kind_agent ON the_forge(kind, agent_id);
CREATE INDEX idx_forge_kind_agent_created ON the_forge(kind, agent_id, created_at DESC);
CREATE INDEX idx_forge_kind_status ON the_forge(kind, status);

-- Agent-specific lookups
CREATE INDEX idx_forge_agents ON the_forge(agent_id)
    WHERE kind = 'agent';
CREATE INDEX idx_forge_agent_dept ON the_forge((meta->>'department'))
    WHERE kind = 'agent';

-- Project lookups
CREATE INDEX idx_forge_project_owner ON the_forge((meta->>'owner'))
    WHERE kind = 'project';

-- Pipeline lookups
CREATE INDEX idx_forge_pipeline ON the_forge((meta->>'pipeline_slug'))
    WHERE kind = 'pipeline_stage';

-- Trigram for search
CREATE INDEX idx_forge_slug_trgm ON the_forge USING gin(slug gin_trgm_ops);
CREATE INDEX idx_forge_title_trgm ON the_forge USING gin(title gin_trgm_ops);

-- Conditional updated_at: skip for append-only kinds
CREATE OR REPLACE FUNCTION forge_conditional_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.kind NOT IN ('fitness_event', 'cognition_job') THEN
        NEW.updated_at = NOW();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_forge_updated_at
    BEFORE UPDATE ON the_forge
    FOR EACH ROW EXECUTE FUNCTION forge_conditional_updated_at();
