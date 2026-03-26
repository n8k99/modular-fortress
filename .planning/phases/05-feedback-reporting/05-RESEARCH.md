# Phase 5: Feedback & Reporting - Research

**Researched:** 2026-03-26
**Domain:** PostgreSQL triggers, Common Lisp action executor, conversation-based reporting
**Confidence:** HIGH

## Summary

Phase 5 closes the feedback loop in the Noosphere Dispatch Pipeline. The infrastructure is substantially built -- task completion, pipeline advancement, blocker detection, conversation posting, and energy rewards all exist in action-executor.lisp. The DB triggers (`on_task_completed_after`, `set_completed_date`, `notify_task_assigned`, `on_task_rejected`) are live on the tasks table.

The work divides into four areas: (1) enrich existing completion messages with GSD context (project name, must_haves, tool results), (2) add wave advancement logic to the `on_task_completed_after` DB trigger, (3) enhance blocker detection to escalate to the supervising executive via conversation, and (4) add Nathan-only notifications for unresolvable escalations and project completion.

**Primary recommendation:** Modify the existing `on_task_completed_after` PostgreSQL trigger to add wave advancement logic, enhance `apply-task-mutations` in action-executor.lisp to post enriched completion reports, and add blocker escalation routing to the existing `has-blocker` detection path. No new services, tables, or APIs required.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01 through D-06: Existing infrastructure confirmed -- parse-complete-lines, pipeline advancement, conversation posting, blocker detection, DB triggers, energy rewards all exist
- D-07: Enhance existing completion messages with GSD context: project name, must_have satisfied, tool results summary from stage_notes
- D-08: Completion reports posted to conversations table addressed to supervising executive (from staff, to executive)
- D-09: Wave advancement via DB trigger `on_task_completed_after()` -- when all sibling tasks in same wave complete, mark next-wave tasks as 'open'
- D-10: Wave number in task context JSON field (already populated by dispatch)
- D-11: Staff posts BLOCKED: to conversations addressed to supervising executive with elevated urgency (+50 for messages)
- D-12: No Nathan notification for routine blockers -- executives handle them
- D-13: Nathan receives conversation messages ONLY when: (1) ESCALATE: @nathan, (2) project completion
- D-14: Project completion detection -- when last task completes, post summary to Nathan

### Claude's Discretion
- Whether to add a /gsd:progress query command that reads project status from DB (or rely on dispatch --status)
- Format of completion summary message (markdown vs plain text)
- Whether wave advancement trigger should log to a separate audit table

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REPT-01 | Task completion posts a report to conversations table (from staff, to executive) | Existing `apply-task-mutations` marks tasks done but does NOT post a completion report to the executive. The pipeline `advance-pipeline` posts a message but only for pipeline-stage tasks, not GSD tasks. Need to add completion reporting in the non-pipeline COMPLETE: path. |
| REPT-02 | Project/task status in DB reflects actual execution state (open -> in_progress -> done) | `execute-work-task` already sets in-progress (line 358). COMPLETE: marks done (line 597). Wave advancement (D-09) handles open for next-wave tasks. Status flow is functional but needs wave gating. |
| REPT-03 | Wave advancement: when all tasks in wave N complete, wave N+1 tasks become perceivable | Requires new logic in `on_task_completed_after()` trigger. Context JSON `{"wave": N}` is populated by dispatch. Trigger must parse context, check sibling completion, advance next wave. |
| REPT-04 | Blocker escalation: staff ghost posts blocker to conversations, executive perceives with high urgency | `has-blocker` detection exists (line 929) but only writes to memory. Need to add conversation posting to the supervising executive. The `assigned_by` field identifies the executive. |
| REPT-05 | /gsd:progress (or dispatch --status) shows real execution state of dispatched projects | `dispatch_to_db.py --status` exists and shows plan/subtask counts. May enhance with wave-level progress. Claude's discretion item. |
| REPT-06 | Nathan only receives conversation notifications for blockers and strategic decisions | Need ESCALATE: @nathan pattern detection in action-executor.lisp, plus project-completion notification in the DB trigger or Lisp code. |
</phase_requirements>

## Standard Stack

### Core (No New Dependencies)

| Component | Location | Purpose | Why Standard |
|-----------|----------|---------|--------------|
| PostgreSQL trigger | `on_task_completed_after()` | Wave advancement + project completion detection | Already fires AFTER UPDATE on tasks; add logic here |
| action-executor.lisp | `/opt/project-noosphere-ghosts/lisp/runtime/` | Completion reporting, blocker escalation | All execution result handling lives here |
| dpn-api conversations | POST `/api/conversations` | Message bus for all reporting | Existing API, no changes needed |
| dispatch_to_db.py | `gotcha-workspace/tools/gsd/` | Status reporting enhancement | Existing --status command |

### No New Libraries Required

This phase modifies existing code only. No new Rust crates, Lisp libraries, Python packages, or npm modules.

## Architecture Patterns

### Existing Flow (What Works Today)

```
Ghost LLM output
  -> parse-complete-lines (extracts COMPLETE: #id)
  -> apply-task-mutations (PATCHes status=done via API)
  -> DB trigger fires: set_completed_date + on_task_completed_after
  -> on_task_completed_after: logs to vault_notes daily note, pg_notify

For pipeline tasks:
  -> validate-stage-output (checks quality)
  -> advance-pipeline (marks done, unblocks next stage, posts conversation, +15 energy)
```

### Enhanced Flow (What Phase 5 Adds)

```
Ghost LLM output (with COMPLETE: #id)
  -> apply-task-mutations marks done
  -> NEW: post enriched completion report to executive conversation
  -> DB trigger fires: set_completed_date + on_task_completed_after
  -> on_task_completed_after: existing vault_notes logic
  -> NEW: wave check — all siblings done? advance next wave tasks to 'open'
  -> NEW: project check — all tasks done? post to Nathan

Ghost LLM output (with BLOCKED: ...)
  -> has-blocker detection (line 929)
  -> existing: writes to memory
  -> NEW: post blocker to executive conversation (assigned_by field)
  -> Executive perceives with +50 urgency (already built into tick engine)

Executive LLM output (with ESCALATE: @nathan)
  -> NEW: parse ESCALATE: lines
  -> NEW: post to Nathan's conversation channel
```

### Key Integration Points

1. **Task context JSON** — `{"wave": 1, "must_haves": [...], "requirements": "...", "depends_on": "..."}`
   - Populated by dispatch_to_db.py (verified in code, line 237-242)
   - Available in Lisp via perception API's context field
   - Available in DB trigger via `NEW.context::jsonb`

2. **Executive identification** — `assigned_by` field on tasks table
   - Set by dispatch (value: 'gsd') for top-level tasks
   - Set by ghosts for delegated subtasks (value: executive agent-id)
   - For GSD tasks, project owner (from projects table) is the executive

3. **Nathan agent ID** — Use `'nathan'` as to_agent in conversations
   - Nathan perceives conversations via the same perception API
   - Messages to nathan get +50 urgency boost

### Recommended Modification Points

```
action-executor.lisp:
├── apply-task-mutations (line 572)     # Add completion report posting
├── execute-work-task (line 351)        # Enhance blocker handling
├── write-agent-memory (line 912)       # Blocker already detected here
└── NEW: parse-escalate-lines           # New parser for ESCALATE: @nathan

on_task_completed_after() trigger:
├── Existing vault_notes logic          # Keep as-is
├── NEW: wave advancement check         # Parse context->wave, check siblings
└── NEW: project completion check       # All tasks done? Notify Nathan

dispatch_to_db.py:
└── show_status()                       # Optionally add wave-level reporting
```

### Anti-Patterns to Avoid
- **Do NOT add wave advancement in Lisp** — The DB trigger fires atomically on every status change. Lisp tick engine may miss completions or race. The trigger is the correct place.
- **Do NOT create a new API endpoint for wave advancement** — The trigger handles it server-side, no round-trip needed.
- **Do NOT send messages to Nathan for routine blockers** — Executives handle blockers. Only ESCALATE: @nathan reaches Nathan.
- **Do NOT modify the conversation API** — The existing POST /api/conversations accepts all needed fields (from_agent, to_agent[], message, channel, metadata).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Wave sibling check | Lisp-side polling of all wave tasks | SQL in DB trigger: `SELECT COUNT(*) ... WHERE context::jsonb->>'wave' = X AND status != 'done'` | Atomic, no race conditions, fires on every status change |
| Executive lookup for task | Hardcoded agent mapping | `assigned_by` field or `projects.owner` via JOIN | Data already in DB |
| Message urgency boost | Custom urgency logic | Existing conversation-based +50 boost in tick engine | Already works for all messages |
| Project completion check | Periodic polling | SQL in DB trigger: `SELECT COUNT(*) FROM tasks WHERE project_id = X AND status NOT IN ('done','completed')` | Fires exactly when needed |

## Common Pitfalls

### Pitfall 1: DB Trigger Cannot Call HTTP APIs
**What goes wrong:** The wave advancement trigger needs to mark next-wave tasks as 'open', which is fine (direct SQL). But project completion notification requires posting a conversation, and DB triggers cannot call external HTTP endpoints.
**Why it happens:** PostgreSQL triggers execute within the transaction context and cannot make network calls.
**How to avoid:** Two options: (1) Use `pg_notify('project_completed', ...)` in the trigger, then have a listener (Lisp or a small script) that posts the conversation. (2) Handle project completion detection in Lisp after the COMPLETE: path, not in the trigger.
**Recommendation:** Option 2 is simpler — add project completion check in `apply-task-mutations` after marking a task done. Query remaining tasks for that project. If zero open, post to Nathan. The trigger handles wave advancement (pure SQL), Lisp handles notifications (needs HTTP).

### Pitfall 2: Lisp JSON Underscore-to-Hyphen Conversion
**What goes wrong:** When reading task context JSON, the Lisp parser converts `wave` to `:WAVE`, `must_haves` to `:MUST-HAVES`, `project_id` to `:PROJECT-ID`.
**Why it happens:** Custom JSON parser in `util/json.lisp` does this automatically.
**How to avoid:** Always use hyphenated keywords in Lisp code: `(gethash :WAVE context-hash)`, `(gethash :MUST-HAVES context-hash)`.
**Warning signs:** nil values when you expect data — check keyword case/format.

### Pitfall 3: Context Field is TEXT, Not JSONB
**What goes wrong:** The `context` column in tasks is `text`, not `jsonb`. In the DB trigger, you must cast: `NEW.context::jsonb->>'wave'`.
**Why it happens:** Original schema used text for flexibility.
**How to avoid:** Always cast in SQL: `NEW.context::jsonb`. Handle NULL context gracefully (tasks without context JSON should be skipped).
**Warning signs:** "cannot extract element from a scalar" errors.

### Pitfall 4: Wave Numbers are Integers in JSON but Strings When Extracted
**What goes wrong:** `context::jsonb->>'wave'` returns the string `'1'`, not integer `1`. Comparison with integer fails.
**Why it happens:** The `->>` operator always returns text.
**How to avoid:** Use `(context::jsonb->>'wave')::int` for integer comparison, or compare as text consistently.

### Pitfall 5: GSD Tasks Use assigned_by='gsd', Not an Executive ID
**What goes wrong:** For blocker escalation, `assigned_by` is 'gsd' on dispatch-created tasks, not the executive agent-id.
**Why it happens:** dispatch_to_db.py sets `assigned_by='gsd'` (line 250).
**How to avoid:** For GSD tasks, look up the supervising executive via `projects.owner` (JOIN on project_id). For ghost-created subtasks, `assigned_by` IS the executive.

### Pitfall 6: AFTER UPDATE Trigger Returns Value is Ignored
**What goes wrong:** `on_task_completed_after` currently returns `NEW` but it's an AFTER trigger — the return value is ignored.
**Why it happens:** AFTER triggers fire after the row is committed; only BEFORE triggers can modify the row.
**How to avoid:** Wave advancement must use separate UPDATE statements, not modify NEW. This is already the correct approach since you're updating OTHER rows (next-wave tasks), not the triggering row.

## Code Examples

### Wave Advancement in DB Trigger (SQL)

```sql
-- Add to on_task_completed_after() function body
-- After existing vault_notes logic

-- Wave advancement: check if all sibling tasks in same wave are done
IF NEW.context IS NOT NULL AND NEW.project_id IS NOT NULL THEN
    DECLARE
        current_wave int;
        remaining int;
        next_wave int;
    BEGIN
        current_wave := (NEW.context::jsonb->>'wave')::int;

        IF current_wave IS NOT NULL THEN
            -- Count remaining incomplete tasks in this wave
            SELECT COUNT(*) INTO remaining
            FROM tasks
            WHERE project_id = NEW.project_id
              AND id != NEW.id
              AND context IS NOT NULL
              AND (context::jsonb->>'wave')::int = current_wave
              AND status NOT IN ('done', 'completed');

            IF remaining = 0 THEN
                next_wave := current_wave + 1;

                -- Advance next-wave tasks from their current status to 'open'
                UPDATE tasks
                SET status = 'open', updated_at = NOW()
                WHERE project_id = NEW.project_id
                  AND context IS NOT NULL
                  AND (context::jsonb->>'wave')::int = next_wave
                  AND status IN ('blocked', 'pending');

                -- Notify for logging
                PERFORM pg_notify('wave_advanced', json_build_object(
                    'project_id', NEW.project_id,
                    'completed_wave', current_wave,
                    'next_wave', next_wave
                )::text);
            END IF;
        END IF;
    END;
END IF;

-- Project completion check
IF NEW.project_id IS NOT NULL THEN
    DECLARE
        remaining_tasks int;
    BEGIN
        SELECT COUNT(*) INTO remaining_tasks
        FROM tasks
        WHERE project_id = NEW.project_id
          AND status NOT IN ('done', 'completed');

        IF remaining_tasks = 0 THEN
            -- Update project status
            UPDATE projects SET status = 'completed', updated_at = NOW()
            WHERE id = NEW.project_id;

            -- Notify for Nathan message (handled by Lisp listener or separate check)
            PERFORM pg_notify('project_completed', json_build_object(
                'project_id', NEW.project_id,
                'task_id', NEW.id
            )::text);
        END IF;
    END;
END IF;
```

### Enriched Completion Report (Lisp)

```lisp
;; In apply-task-mutations, after marking task done (line 597)
;; Fetch task data to get context for enriched report
(let ((task-data (handler-case
                     (api-get (format nil "/api/af64/tasks/~a" task-id))
                   (error () nil))))
  (when (and task-data (hash-table-p task-data))
    (let* ((context-str (gethash :CONTEXT task-data))
           (context (when context-str
                      (handler-case (json-parse context-str)
                        (error () nil))))
           (project-id (gethash :PROJECT-ID task-data))
           (stage-notes (or (gethash :STAGE-NOTES task-data) ""))
           (must-haves (when (hash-table-p context)
                         (gethash :MUST-HAVES context)))
           ;; Look up project name
           (project-name (when project-id
                           (handler-case
                               (let ((proj (api-get (format nil "/api/projects/~a" project-id))))
                                 (when (hash-table-p proj) (gethash :NAME proj)))
                             (error () nil))))
           ;; Find supervising executive
           (executive (or (gethash :ASSIGNED-BY task-data)
                          (when project-id
                            (handler-case
                                (let ((proj (api-get (format nil "/api/projects/~a" project-id))))
                                  (when (hash-table-p proj) (gethash :OWNER proj)))
                              (error () nil))))))
      ;; Post completion report to executive
      (when executive
        (handler-case
            (api-post "/api/conversations"
                      (json-object
                       :from-agent agent-id
                       :to-agent (json-array executive)
                       :message (format nil "[Task Complete] ~a finished task #~a~%~%**Project:** ~a~%**Must-haves:** ~{~a~^, ~}~%**Summary:** ~a"
                                        agent-id task-id
                                        (or project-name "unknown")
                                        (or must-haves '("none"))
                                        (subseq stage-notes 0 (min 500 (length stage-notes))))
                       :channel "noosphere"
                       :metadata (json-object
                                  :source "task_completion"
                                  :task-id task-id
                                  :project-id project-id)))
          (error (e) (format t "  [completion-report-error] ~a: ~a~%" task-id e)))))))
```

### Blocker Escalation to Executive (Lisp)

```lisp
;; In write-agent-memory or execute-work-task, when has-blocker is true
;; Find the supervising executive and post blocker notification
(when has-blocker
  (let* ((task (gethash :task metadata))
         (task-id (when task (gethash :id task)))
         (project-id (when task (gethash :project-id task)))
         (executive (or (when task (gethash :assigned-by task))
                        ;; Fallback: look up project owner
                        (when project-id
                          (handler-case
                              (let ((proj (api-get (format nil "/api/projects/~a" project-id))))
                                (when (hash-table-p proj) (gethash :OWNER proj)))
                            (error () nil))))))
    (when (and executive (not (string-equal executive "gsd")))
      (handler-case
          (api-post "/api/conversations"
                    (json-object
                     :from-agent agent-id
                     :to-agent (json-array executive)
                     :message (format nil "[BLOCKED] ~a is blocked on task #~a~%~%~a"
                                      agent-id (or task-id "unknown") summary)
                     :channel "noosphere"
                     :metadata (json-object :source "blocker_escalation"
                                            :task-id task-id)))
        (error (e) (format t "  [blocker-escalation-error] ~a~%" e))))))
```

### Nathan Escalation Parser (Lisp)

```lisp
(defun parse-escalate-lines (content)
  "Extract ESCALATE: @nathan reason lines."
  (let ((results '())
        (lines (uiop:split-string content :separator '(#\Newline))))
    (dolist (line lines)
      (let ((trimmed (string-trim '(#\Space #\Tab #\Return) line)))
        (when (search "ESCALATE:" trimmed)
          (let ((after (string-trim '(#\Space #\Tab)
                        (subseq trimmed (+ (search "ESCALATE:" trimmed) 9)))))
            (when (search "@nathan" after)
              (push after results))))))
    (nreverse results)))
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Hardcoded pipeline advancement | Per-task stage tracking | Phase 3-4 | Pipeline works for fixed pipelines but not GSD waves |
| Blocker written to memory only | Need to also post to executive | Phase 5 | Executives can act on blockers without checking memory |
| No wave gating | Wave N+1 blocked until wave N completes | Phase 5 | Proper dependency ordering for GSD plans |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual verification via DB queries + PM2 logs |
| Config file | None -- Lisp and SQL changes verified empirically |
| Quick run command | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT proname, prosrc FROM pg_proc WHERE proname='on_task_completed_after'"` |
| Full suite command | Manual: create test tasks, mark done, verify wave advancement + conversations |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| REPT-01 | Completion report to conversations | smoke | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT id,from_agent,to_agent,message FROM conversations WHERE metadata->>'source'='task_completion' ORDER BY created_at DESC LIMIT 1"` | N/A (DB query) |
| REPT-02 | Status reflects execution state | smoke | `python3 gotcha-workspace/tools/gsd/dispatch_to_db.py --status` | Exists |
| REPT-03 | Wave advancement | integration | Create 2 wave-1 tasks, mark both done, verify wave-2 tasks become 'open' | Manual |
| REPT-04 | Blocker escalation | smoke | `PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -c "SELECT id,from_agent,to_agent,message FROM conversations WHERE metadata->>'source'='blocker_escalation' ORDER BY created_at DESC LIMIT 1"` | N/A |
| REPT-05 | Progress reporting | smoke | `python3 gotcha-workspace/tools/gsd/dispatch_to_db.py --status` | Exists |
| REPT-06 | Nathan-only notifications | smoke | Query conversations to_agent containing 'nathan' with source='escalation' or 'project_complete' | N/A |

### Sampling Rate
- **Per task commit:** Verify DB trigger function source + test with manual task status update
- **Per wave merge:** Full end-to-end: dispatch test project, mark tasks done, verify conversations + wave advancement
- **Phase gate:** All 6 REPT requirements verified via DB queries

### Wave 0 Gaps
- [ ] Test data: create a test project with 2 waves (2 tasks each) for wave advancement verification
- [ ] Verification script: SQL script that checks all REPT requirements against DB state

## Open Questions

1. **Project completion notification: trigger vs Lisp?**
   - What we know: DB triggers cannot call HTTP APIs. Wave advancement is pure SQL (fine in trigger). Project completion notification needs a conversation INSERT.
   - What's unclear: Should we INSERT the conversation directly in the trigger (it IS a local table, not HTTP), or handle in Lisp?
   - Recommendation: INSERT directly in the trigger. The conversations table is in the same database -- no HTTP needed. The trigger can `INSERT INTO conversations (from_agent, to_agent, message, channel, metadata) VALUES ('system', ARRAY['nathan'], ...)` directly. This is atomic and guaranteed to fire.

2. **dispatch --status wave enhancement**
   - What we know: `show_status()` already shows plan/subtask counts
   - What's unclear: How much detail is useful? Wave-by-wave breakdown?
   - Recommendation: Add wave-level grouping to --status output. Simple enhancement, low effort. Skip /gsd:progress command -- dispatch --status is sufficient.

3. **Audit table for wave advancement**
   - What we know: pg_notify already fires for logging
   - Recommendation: Skip separate audit table. The pg_notify + existing vault_notes logging is sufficient. Wave advancement events are also visible in task updated_at timestamps.

## Environment Availability

Step 2.6: SKIPPED (no external dependencies identified). All changes are to existing PostgreSQL triggers, Common Lisp code, and Python script on the same droplet.

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` — Read lines 13-958, all execution paths verified
- `on_task_completed_after()` trigger — queried from `pg_proc`, full source reviewed
- `/root/gotcha-workspace/tools/gsd/dispatch_to_db.py` — Full file read, context JSON format verified (line 237-242)
- `/opt/dpn-api/src/handlers/af64_conversations.rs` — Full file read, POST endpoint schema confirmed
- Tasks table schema — `\d tasks` verified all columns including context (text), project_id, assigned_by, stage_notes
- Conversations table schema — `\d conversations` verified from_agent, to_agent[], message, channel, metadata (jsonb)
- Projects table schema — `\d projects` verified owner, status, status check constraint

### Secondary (MEDIUM confidence)
- Lisp JSON parser underscore-to-hyphen behavior — documented in CLAUDE.md, confirmed by code patterns in action-executor.lisp

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new dependencies, all existing code verified
- Architecture: HIGH - all integration points read and understood, flows traced end-to-end
- Pitfalls: HIGH - identified from actual code review (context text vs jsonb, assigned_by='gsd', trigger limitations)

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable infrastructure, no external dependency drift)
