# Phase 3: Executive Cognition - Research

**Researched:** 2026-03-26
**Domain:** Common Lisp ghost runtime (action-planner + action-executor) and Rust dpn-api task endpoints
**Confidence:** HIGH

## Summary

Phase 3 fills a specific gap in an otherwise functional executive cognition system. The ghost tick engine already has `build-project-review-job` (creates LLM prompts for executives to review projects), `execute-project-review` (posts results to conversations and applies task mutations), and parsers for CLASSIFY/DELEGATE/COMPLETE commands. The critical missing piece is: the project review prompt tells executives to output `CREATE_TASK:` lines, but no parser or executor exists to handle them.

Three work areas: (1) implement `parse-create-task-lines` and wire it into `apply-task-mutations` in action-executor.lisp, (2) enrich `build-project-review-job` in action-planner.lisp to include GSD context (wave structure, must_haves, task details) and team roster, (3) extend the dpn-api `create_task` endpoint to accept `parent_id` and `source` fields (currently missing from `NewTask` struct).

**Primary recommendation:** Follow the existing parse-line pattern exactly (see `parse-delegate-lines` at line 512 of action-executor.lisp), add CREATE_TASK handling to `apply-task-mutations`, and fix the API endpoint to accept `parent_id`, `source`, and `task_id` fields.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01 through D-05: Executive cognition framework substantially exists; gap is CREATE_TASK parser only
- D-06: Use Lisp `api-post` to POST to dpn-api task creation endpoint
- D-07: CREATE_TASK format: `CREATE_TASK: <description> assignee=<agent_id>` -- single line creates AND assigns
- D-08: New function `parse-create-task-lines` alongside existing parsers in `apply-task-mutations`
- D-09: Created tasks get `parent_id` set to parent task being reviewed (if context available)
- D-10: Include FULL GSD context in project review prompt: wave structure, must_haves, phase goals, parent/subtask hierarchy
- D-11: Modify `build-project-review-job` to query task details for each project and include context JSON
- D-12: Include team roster in prompt so executives know who to delegate to
- D-13: Same-action creation + delegation: `CREATE_TASK: Build auth module assignee=casey`
- D-14: If no assignee specified, task created unassigned
- D-15: Proactive-when-idle monitoring already works -- no changes needed
- D-16: Existing `execute-project-review` already posts to conversations

### Claude's Discretion
- Whether CREATE_TASK should support additional fields beyond description and assignee (e.g., priority, stage, due_date)
- Whether to add UPDATE_GOAL parsing (mentioned in prompt but like CREATE_TASK, has no executor)
- Error handling when executive references nonexistent staff agents
- Whether to add ESCALATE: parsing (posts to Nathan's conversation channel)

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| EXEC-01 | Executive ghost receives dispatched project and uses LLM cognition to decompose into staff-suitable tasks | `build-project-review-job` already creates the cognition job; enriching prompt with GSD context (D-10/D-11) enables informed decomposition; CREATE_TASK parser (D-08) enables output execution |
| EXEC-02 | Executive task breakdown respects wave ordering from GSD dispatch context | GSD context JSON in task `context` field contains wave numbers; prompt enrichment (D-10) makes wave structure visible to LLM |
| EXEC-03 | Executive assigns decomposed tasks to staff ghosts based on domain expertise and tool_scope | Team roster in prompt (D-12) gives exec agent/skill awareness; `assignee=<agent_id>` in CREATE_TASK format (D-07/D-13) enables assignment |
| EXEC-04 | Decomposed subtasks are written to tasks table via API with project_id linkage | `api-post "/api/af64/tasks"` with project_id and parent_id; requires API endpoint fix (NewTask struct needs parent_id and source fields) |
| EXEC-05 | Executive monitors progress of delegated tasks across ticks and re-prioritizes as needed | Already works via `build-project-review-job` firing when idle (D-15); project perception includes open/completed task counts |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Stack**: Common Lisp (SBCL) for ghost runtime, Rust (Axum) for dpn-api, PostgreSQL for state -- no new languages
- **DB is the OS**: All state in master_chronicle, not files
- **Lisp JSON quirk**: Parser converts underscores to hyphens (`:is-error` not `:is_error`)
- **Lisp conventions**: `kebab-case` functions, `handler-case` for error wrapping, `(format t ...)` for logging
- **Rust conventions**: `snake_case`, `anyhow::Result`, serde for serialization
- **Ghost LLM**: Claude Code CLI, $0.50/request budget
- **UTF-8 Rule**: Never mix character positions with byte indices in Rust

## Architecture Patterns

### Existing Pattern: LLM Command Output -> Parse -> Apply via API

This is the core pattern used throughout the executor. The LLM produces structured text commands, parsers extract them, and API calls apply mutations.

```
LLM output text
  -> parse-classify-lines()   -> api-patch /api/af64/tasks/:id  {department, assignee}
  -> parse-delegate-lines()   -> api-patch /api/af64/tasks/:id  {assignee}
  -> parse-complete-lines()   -> api-patch /api/af64/tasks/:id  {status: "done"}
  -> parse-create-task-lines() [NEW] -> api-post /api/af64/tasks {text, assignee, ...}
```

All parsers follow identical structure:
1. Split content by newline
2. Trim each line
3. Search for prefix string (e.g., "DELEGATE:")
4. Extract fields after prefix using `search` and `subseq`
5. Return list of extracted tuples

### Pattern: apply-task-mutations dispatch point

`apply-task-mutations` (action-executor.lisp:548) is the single integration point. It calls all parsers and applies results. Adding CREATE_TASK means:
1. Call `parse-create-task-lines` to get list of (description assignee)
2. For each result, `api-post "/api/af64/tasks"` with appropriate fields
3. Wrap in `handler-case` for error resilience
4. Increment mutation count

### Pattern: Prompt Construction in build-project-review-job

The project review prompt (action-planner.lisp:665) currently receives:
- Executive persona
- Project summaries (name, ID, status, open/completed task counts, goals, blockers, current_context)
- Generic action list: CREATE_TASK | ESCALATE | DELEGATE | UPDATE_GOAL

Enrichment means: for each project, query its tasks via API and format the GSD context (wave numbers, must_haves) into readable text within the prompt.

### Recommended Code Structure (changes only)

```
/opt/project-noosphere-ghosts/lisp/runtime/
  action-executor.lisp    # + parse-create-task-lines, extend apply-task-mutations
  action-planner.lisp     # + GSD context formatting, team roster in build-project-review-job

/opt/dpn-api/src/handlers/
  af64_tasks.rs           # + parent_id, source, task_id in NewTask struct + INSERT
```

### Anti-Patterns to Avoid
- **Hardcoded team rosters**: The engineering-work-prompt (line 500) hardcodes Eliana's team. Do NOT replicate this -- query `/api/agents` or `/api/af64/agents` filtered by department for dynamic rosters
- **Mixing byte and character operations**: In any Rust changes, use `.chars().take(N)` not byte slicing
- **Direct DB access from Lisp**: All DB operations go through dpn-api REST endpoints. Never connect to PostgreSQL from Lisp.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Task ID generation | Custom ID scheme | UUID or `ghost-{agent_id}-{timestamp}` format | Must be unique per `tasks_task_id_unique` constraint |
| Team roster lookup | Hardcoded agent lists | `api-get "/api/af64/agents"` with department filter | Roster changes without code deploys |
| JSON construction | String concatenation | Existing `json-object` / `json-array` utilities | Already used everywhere in the codebase |
| Error handling | Custom retry logic | `handler-case` wrapping (existing pattern) | Tick engine continues on error |

## Common Pitfalls

### Pitfall 1: task_id NOT NULL Constraint
**What goes wrong:** The `create_task` API endpoint (af64_tasks.rs:161) does NOT set `task_id` in its INSERT statement, but the column is `NOT NULL` with no default and has a unique constraint (`tasks_task_id_unique`). Any POST to `/api/af64/tasks` will fail with a database error.
**Why it happens:** The endpoint was likely written but never tested for ghost-created tasks.
**How to avoid:** Add `task_id` to both the `NewTask` struct and the INSERT query. Generate a unique task_id in the Rust handler (e.g., `format!("ghost-{}", Uuid::new_v4())`) or accept it from the Lisp caller.
**Warning signs:** Database error on first CREATE_TASK attempt; "NOT NULL violation" in API error response.

### Pitfall 2: Missing parent_id and source in NewTask
**What goes wrong:** D-09 requires `parent_id` for hierarchy linking and the CONTEXT.md specifies `source='ghost'` for ghost-created tasks. Neither field exists in the `NewTask` struct.
**Why it happens:** The endpoint was built for basic task creation, not ghost-initiated creation.
**How to avoid:** Extend `NewTask` with `parent_id: Option<i32>` and `source: Option<String>`, add them to the INSERT query. Default source to `"ghost"` if not provided.
**Warning signs:** Tasks appear in DB without hierarchy or source tracking.

### Pitfall 3: Lisp JSON Key Hyphenation
**What goes wrong:** The Lisp JSON parser converts underscores to hyphens. When Lisp sends `(json-object :parent-id 123)`, it serializes as `{"parent-id": 123}` -- but the Rust API expects `{"parent_id": 123}`.
**Why it happens:** This is documented in CLAUDE.md as a known quirk.
**How to avoid:** Use the `keyword->json-key` function in the JSON encoder which should convert hyphens back to underscores. Verify by checking how existing `api-post` calls work (e.g., `:project-id` in json-object).
**Warning signs:** API returns 422 or ignores fields silently because key names don't match.

### Pitfall 4: Project Context Not Reaching Prompt
**What goes wrong:** Perception returns project summaries with only counts (open_tasks, completed_tasks), not individual task details. The prompt won't have wave structure or must_haves unless we query tasks separately.
**Why it happens:** Perception was designed for summary-level awareness, not detailed project planning.
**How to avoid:** In `build-project-review-job`, after getting projects from perception, make a supplementary `api-get "/api/af64/tasks"` call with `project_id` filter for each project to get task details including `context` field (which contains GSD wave/must_haves JSON).
**Warning signs:** Executive LLM output ignores wave ordering because it never saw wave data.

### Pitfall 5: Prompt Token Budget
**What goes wrong:** Including full GSD context + team roster + all project details may exceed the cognition broker's cost estimate (currently 5 tokens for project_review). Large prompts consume more API budget.
**Why it happens:** Each project may have 10+ tasks with context JSON.
**How to avoid:** Limit task detail expansion to active/open tasks only. Truncate context JSON to key fields (wave, must_haves). Keep cost-estimate reasonable or adjust it.
**Warning signs:** Cognition broker rejects jobs as too expensive; LLM responses are truncated.

## Code Examples

### Pattern: parse-delegate-lines (template for parse-create-task-lines)

```lisp
;; Source: /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp:512
(defun parse-delegate-lines (content)
  "Extract DELEGATE: #id assignee=agent lines. Returns list of (id assignee)."
  (let ((results '())
        (lines (uiop:split-string content :separator '(#\Newline))))
    (dolist (line lines)
      (let ((trimmed (string-trim '(#\Space #\Tab #\Return) line)))
        (when (and (> (length trimmed) 10) (search "DELEGATE:" trimmed))
          (let* ((after (subseq trimmed (+ (search "DELEGATE:" trimmed) 9)))
                 (id-match ...)
                 (assignee-match ...))
            (when (and id-match assignee-match)
              (push (list id-match assignee-match) results))))))
    (nreverse results)))
```

### Pattern: api-post for task creation

```lisp
;; Source: /opt/project-noosphere-ghosts/lisp/runtime/api-client.lisp:66
;; api-post takes a path and a json-object payload
(api-post "/api/af64/tasks"
          (json-object
           :text "Build auth module"
           :assignee "casey"
           :department "Engineering"
           :project-id 42
           :parent-id 100
           :source "ghost"))
```

### Pattern: apply-task-mutations integration point

```lisp
;; Source: /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp:548
(defun apply-task-mutations (agent-id content)
  "Parse CLASSIFY, DELEGATE, and COMPLETE lines from content and apply via API."
  (let ((classified (parse-classify-lines content))
        (delegated (parse-delegate-lines content))
        (completed (parse-complete-lines content))
        ;; ADD: (created (parse-create-task-lines content))
        (count 0))
    ;; ... existing handlers ...
    ;; ADD: dolist over created, api-post for each
    count))
```

### Pattern: Current project review prompt

```lisp
;; Source: /opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp:665
;; Current prompt includes: CREATE_TASK: <description> | ESCALATE: <issue> | DELEGATE: <agent> <work> | UPDATE_GOAL: <project_id> <new_goal>
;; The LLM is already told to output CREATE_TASK -- we just need to parse it
```

### Existing NewTask struct (needs extension)

```rust
// Source: /opt/dpn-api/src/handlers/af64_tasks.rs:142
#[derive(Deserialize)]
pub struct NewTask {
    pub text: String,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub department: Option<String>,
    pub doc_path: Option<String>,
    pub due_date: Option<String>,
    pub project_id: Option<i32>,
    // MISSING: parent_id, source, task_id
}
```

## Critical Finding: API Endpoint Gaps

The `POST /api/af64/tasks` endpoint has three gaps that MUST be fixed before Lisp can create tasks:

| Gap | Current State | Required State | Impact |
|-----|---------------|----------------|--------|
| `task_id` not in INSERT | Column is NOT NULL, no default, unique constraint | Generate UUID-based task_id in handler | **INSERT will fail** |
| `parent_id` not accepted | Not in NewTask struct or INSERT | Add to struct and INSERT for hierarchy linking | No parent-child relationships |
| `source` not accepted | Not in NewTask struct or INSERT | Add to struct and INSERT, default to "ghost" | Can't distinguish ghost-created tasks |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Hardcoded team roster (Eliana only) | Query agents table by department | Phase 3 | Dynamic roster for all executives |
| Project review shows counts only | Full GSD context in review prompt | Phase 3 | Executives see wave structure and must_haves |
| CREATE_TASK in prompt but unimplemented | Full parse + execute pipeline | Phase 3 | Executives can actually create subtasks |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual integration testing via PM2 + tick engine |
| Config file | None (Lisp has no automated test suite; Rust uses `cargo test`) |
| Quick run command | `cargo test --manifest-path /opt/dpn-api/Cargo.toml` (Rust API only) |
| Full suite command | Manual: start ghosts, dispatch test project, observe tick logs |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| EXEC-01 | Executive decomposes project into tasks | integration/manual | Dispatch test project, observe LLM output in conversations | N/A |
| EXEC-02 | Task breakdown respects wave ordering | integration/manual | Verify created tasks reference correct wave numbers | N/A |
| EXEC-03 | Executive assigns tasks to staff | integration/manual | Check assignee field on ghost-created tasks | N/A |
| EXEC-04 | Subtasks written to DB with project_id | unit+integration | `cargo test` for API endpoint; manual for Lisp integration | Partial (Rust) |
| EXEC-05 | Executive monitors delegated tasks | integration/manual | Already works; verify no regression in tick logs | N/A |

### Sampling Rate
- **Per task commit:** `cargo test --manifest-path /opt/dpn-api/Cargo.toml` (for Rust changes)
- **Per wave merge:** Manual tick test: start ghosts with test project, observe 2-3 ticks
- **Phase gate:** Full integration test: dispatch GSD project, verify executive creates tasks, tasks appear in DB with correct fields

### Wave 0 Gaps
- [ ] Test project in DB: Create a minimal GSD-dispatched project owned by an executive (e.g., Eliana) with 2-3 tasks across 2 waves
- [ ] API endpoint test: `curl -X POST http://localhost:8080/api/af64/tasks -H "X-API-Key: dpn-nova-2026" -H "Content-Type: application/json" -d '{"text":"test task"}'` -- this will FAIL currently, confirming Pitfall 1

## Discretion Recommendations

### Additional CREATE_TASK fields
**Recommendation: Keep minimal.** Only `description` and `assignee` for v1. Priority, stage, and due_date add parsing complexity with little value -- executives can always update tasks in follow-up ticks. The Lisp `api-post` can easily be extended later.

### UPDATE_GOAL parsing
**Recommendation: Skip for Phase 3.** UPDATE_GOAL would need a separate API endpoint on the projects table. Lower value than CREATE_TASK. Can be added in a follow-up if executives naturally try to use it.

### Error handling for nonexistent agents
**Recommendation: Log and continue.** If `assignee=nonexistent_agent`, the task gets created with that assignee string but nobody will perceive it. Add a `(format t "  [create-task-warn] unknown assignee: ~a~%" assignee)` log line. Validation against the agents table is a v2 concern.

### ESCALATE parsing
**Recommendation: Implement as a lightweight addition.** ESCALATE is simple: parse `ESCALATE: <message>` and post a conversation to Nathan. This is 10-15 lines of Lisp and gives executives a critical communication channel. Include it if time allows.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- full source read, parse patterns verified at lines 488-583, execute-project-review at 643-690
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- full source read, build-project-review-job at 626-675, default-job-builder priority chain at 677-687
- `/opt/project-noosphere-ghosts/lisp/runtime/api-client.lisp` -- full source read, api-post/api-get patterns
- `/opt/dpn-api/src/handlers/af64_tasks.rs` -- full source read, NewTask struct at 142-151, create_task at 153-166
- `/opt/dpn-api/src/handlers/af64_perception.rs` -- task queries at 95-131, project queries at 337-362
- Live DB schema: `\d tasks` and `\d agents` verified against running master_chronicle

### Secondary (MEDIUM confidence)
- Agent roster: queried from live DB (8 executives confirmed)
- Task sources: queried from live DB (7 distinct sources, no "ghost" source yet)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code is in-repo, read directly, patterns verified
- Architecture: HIGH -- existing patterns documented with line numbers, integration points identified
- Pitfalls: HIGH -- task_id NOT NULL gap verified against live DB schema; JSON key quirk documented in CLAUDE.md
- API gaps: HIGH -- verified by reading NewTask struct and INSERT query directly

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable codebase, no external dependency drift)
