# Phase 7: Structured Artifact Passing - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace freeform text stage_notes with typed, validated JSON artifacts so pipeline stage handoffs carry structured context. Migrate stage_notes to JSONB, define universal base + stage-specific extension schemas, update validation to enforce schema, and rewire action-planner to load predecessor output from DB instead of disk files. Final pipeline deliverables must be persisted to their appropriate DB tables (documents, vault_notes, etc.), not just stage_notes.

</domain>

<decisions>
## Implementation Decisions

### Schema Design
- **D-01:** Universal base schema + stage-specific extension fields. Base fields are required on every stage output; extensions are optional and vary by stage/pipeline type.
- **D-02:** Base schema fields: `{summary: string, key_outputs: [{name: string, content: string}], issues: [{severity: string, description: string}], metadata: {stage: string, agent_id: string, timestamp: string, duration_ms: number}}`.
- **D-03:** Stage-specific extensions are defined per-stage but validated only if present (not required). This keeps schemas manageable across 20+ stages and 4 pipeline types.

### Storage Location
- **D-04:** Migrate `stage_notes` column from TEXT to JSONB. Structured output lives in the DB where perception can query it directly. Respects DB-is-the-OS principle.
- **D-05:** Existing freeform text data wrapped during migration: `{"legacy_text": "<original content>", "schema_version": 0}`. No data loss.
- **D-06:** Disk file loading pattern (`~/gotcha-workspace/tools/{tool}/{STAGE}.md`) is deprecated. New stages read/write structured output from stage_notes JSONB only.
- **D-07:** Final pipeline results (the completed pipeline's deliverable) must be persisted to the appropriate DB table — documents, vault_notes, decisions, etc. — depending on what was produced. stage_notes JSONB is for in-flight handoff between stages only.

### Validation Strictness
- **D-08:** Required base fields + optional extensions. Base fields (summary, key_outputs, metadata) MUST be present and valid JSON. Stage-specific extension fields validated if present but not required.
- **D-09:** Existing rejection/retry pattern preserved (3 attempts, then task blocked). Rejection now fires on: not valid JSON, missing base fields, or base field type mismatch.
- **D-10:** Replace existing keyword + length checks in `validate-stage-output` with JSON schema validation. Keep minimum length check on `summary` field.

### Context Handoff
- **D-11:** action-planner reads predecessor task's `stage_notes` JSONB from DB and injects structured output into the LLM prompt. Replaces the `load-previous-stage-output` disk-file function.
- **D-12:** Hard prompts stay in documents table (separate from stage schema). Instructions are not artifacts — clean separation of concerns. Note: hard prompts often come from the `Templates/` virtual directory in the documents table — researcher should investigate this pattern.

### Claude's Discretion
- Whether to add a `schema_version` field to the base schema for future evolution
- Exact JSON structure of stage-specific extensions (researcher determines based on what each stage currently validates for)
- Whether `advance-pipeline` should validate the JSON before storing or rely on `validate-stage-output` alone
- How to handle the transition period where some tasks have legacy text and others have structured JSON

### Carried From Prior Phases
- Phase 4: Tool results stored in stage_notes (currently freeform text)
- Phase 5: Completion reports reference stage_notes content
- Phase 6: Dependency chains ensure pipeline stages execute in order

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Pipeline Stage Definitions & Advancement
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Key: `*pipeline-advancement*` (lines 42-75), `advance-pipeline` (lines 197-318), `validate-stage-output` (lines 78-195), stage_notes writes (lines 209, 374-376, 439-443)

### Action Planner (Context Building)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` — Key: `build-pipeline-task-job` (lines 310-397), `load-previous-stage-output` (disk file loading, to be replaced), hard prompt loading, stage_notes extraction (line 315)

### Task API (stage_notes Handling)
- `/opt/dpn-api/src/handlers/af64_tasks.rs` — Task structs, create/update handlers for stage_notes field

### Perception (Task Context)
- `/opt/dpn-api/src/handlers/af64_perception.rs` — How tasks are returned to ghosts, stage_notes in response

### Schema Migration Pattern
- `/root/.planning/phases/06-task-dependency-chains/migrations/` — Phase 6 migration pattern (ALTER COLUMN with data preservation)

### Task Table Schema
- `/root/gotcha-workspace/tools/db/migrate_to_postgres.sql` — Current tasks table definition

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `validate-stage-output` — existing per-stage validation function. Refactor to check JSON schema instead of keyword matching.
- `advance-pipeline` — stage advancement function already handles stage_notes writes. Update to write structured JSON.
- `load-previous-stage-output` — disk-file loader to be replaced with DB query. Same call site, different data source.
- Phase 6 migration pattern — ALTER COLUMN with data preservation via CASE WHEN.
- `json-object` / Lisp JSON construction — already used throughout for API calls.

### Established Patterns
- Stage completion: content → validate → store in stage_notes → advance to next stage → notify next assignee
- Rejection: validate fails → increment count → store feedback in stage_notes → reopen task
- Pipeline advancement: current task done → find next stage/assignee → unblock next task → post conversation
- Energy rewards: +15 per stage, +30 for pipeline completion

### Integration Points
- `action-executor.lisp` `validate-stage-output` — rewrite validation logic for JSON schema
- `action-executor.lisp` `advance-pipeline` — rewrite stage_notes writes to produce structured JSON
- `action-planner.lisp` `build-pipeline-task-job` — replace disk-file loading with DB query for predecessor stage_notes
- `af64_tasks.rs` — update stage_notes field type from String/Text to serde_json::Value (JSONB)
- `af64_perception.rs` — ensure stage_notes JSONB is properly serialized in task response
- DB migration — ALTER stage_notes from TEXT to JSONB with data wrapping

</code_context>

<specifics>
## Specific Ideas

- Final pipeline deliverables must land in the correct DB table (documents for content, vault_notes for operational docs, decisions for strategic choices) — not just sit in stage_notes forever
- The 4 pipeline types (engineering, investment, editorial, modular fortress) each have different final stages that produce different deliverable types
- Schema version field allows gradual evolution without breaking existing tasks

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 07-structured-artifact-passing*
*Context gathered: 2026-03-26*
