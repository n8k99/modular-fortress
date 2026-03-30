# Phase 26: Runtime Stability - Research

**Researched:** 2026-03-30
**Domain:** Common Lisp (SBCL) tick engine bug fixes and commit hygiene
**Confidence:** HIGH

## Summary

Phase 26 is a stabilization phase: fix one known paren scope bug (STAB-01) and commit 9 files of accumulated fixes (STAB-02) in the Noosphere Ghosts tick engine. All changes already exist as uncommitted modifications in `/opt/project-noosphere-ghosts/` on the `em-droplet` branch (39 commits ahead of origin). The work is surgical -- no new features, no new files, no architectural changes.

The paren bug in `execute-work-task` (lines 488-497 of `action-executor.lisp`) has been precisely identified: line 496 has one excess closing paren that prematurely terminates the `when` block, leaving the `handler-case` error clause as dead code. The fix is a single paren relocation. The remaining 8 files contain fixes for UTF-8 byte length, NULL handling, SQL formatting, type coercion, description column removal, and error handler wrapping.

**Primary recommendation:** Fix the paren bug first (STAB-01), then commit all 9 files in 4-5 logical atomic commits (STAB-02), then verify SBCL loads cleanly and a tick completes without errors.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: All 9 uncommitted modified files in /opt/project-noosphere-ghosts/ are in scope -- commit everything before building Phase 27+ on top
- D-02: The STAB-01 paren bug in execute-work-task (action-executor.lisp lines 471-612) must be fixed so the json-object result returns from the correct let* scope
- D-03: Specific fixes identified: UTF-8 pg-escape, description column removal, error handler-case wrapping, SQL formatting cleanup, changes in db-auxiliary, db-conversations, cognition-types, task-scheduler, packages
- D-04: Atomic commits per logical fix -- not one giant commit. Group related changes
- D-05: Follow existing commit message conventions: `fix(component): description`
- D-06: SBCL must load the full system without compile errors or warnings after all fixes
- D-07: A complete tick cycle must run on the live system without runtime errors
- D-08: Verification is against the live noosphere-ghosts process (PM2), not a test harness

### Claude's Discretion
- Exact commit grouping (how to batch the 9 files into logical commits)
- Order of fixes (which to commit first)
- Whether to add any defensive error handling beyond what's already in the uncommitted changes

### Deferred Ideas (OUT OF SCOPE)
None
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| STAB-01 | execute-work-task paren scope bug fixed -- return json-object executes in correct let* scope | Paren bug precisely located at line 496 (one excess close paren). Fix identified below in Architecture Patterns. |
| STAB-02 | All 7 tick engine fixes from 2026-03-29 session committed to project-noosphere-ghosts | All 9 modified files catalogued with exact diffs. Commit grouping recommended below. |
</phase_requirements>

## Standard Stack

Not applicable -- this phase modifies existing Common Lisp code in `/opt/project-noosphere-ghosts/`. No new dependencies. No package installations.

**Existing stack (for reference):**
- SBCL (Steel Bank Common Lisp) -- the runtime
- libpq.so.5 -- PostgreSQL C library via SB-ALIEN FFI
- PM2 -- process manager for noosphere-ghosts
- Git -- version control on `em-droplet` branch

## Architecture Patterns

### STAB-01: The Paren Scope Bug (CRITICAL)

**Location:** `lisp/runtime/action-executor.lisp`, lines 488-497

**What's wrong:** Line 496 has 6 closing parens but should have 5. The excess paren closes the `(when tool-results` block from line 485 prematurely, which means:

1. The `(error () nil)` on line 497 is NOT an error clause for handler-case -- it's a standalone expression evaluated in the enclosing `let` block (dead code returning nil)
2. handler-case on line 488 has NO error clause, so errors in tool-result DB writes propagate unhandled
3. The `json-object` return on lines 605-612 IS still reachable (it's in the outer `let*` from line 472), so the function doesn't crash -- but tool errors are unhandled

**Paren depth analysis (verified by counting):**

```
L488: +1  cumulative=1  | (handler-case
L489: +4  cumulative=5  |     (let ((results-text (with-output-to-string (s)
L490: +1  cumulative=6  |                           (dolist (r tool-results)
L491: -4  cumulative=2  |                             (format s ...)))))
L492: +0  cumulative=2  |       (setf content ...)
L493: +1  cumulative=3  |       (when (and task ...)
L494: +1  cumulative=4  |         (db-update-task ...
L495: +1  cumulative=5  |                        :stage-notes (json-object ...
L496: -6  cumulative=-1 |                                     :schema-version 0))))))  <-- BUG: -1 means one too many
L497: -2  cumulative=-3 |           (error () nil)))  <-- dead code, not handler-case clause
```

**The fix:** Remove one closing paren from line 496 (change 6 to 5), and ensure the `(error () nil)` on line 497 is properly inside handler-case. The corrected structure:

```lisp
        (handler-case
            (let ((results-text (with-output-to-string (s)
                                  (dolist (r tool-results)
                                    (format s "~%--- TOOL: ~a ---~%~a~%" (first r) (second r))))))
              (setf content (concatenate 'string content results-text))
              (when (and task (gethash :id task))
                (db-update-task (gethash :id task)
                               :stage-notes (json-object :legacy-text (subseq content 0 (min 4000 (length content)))
                                                         :schema-version 0))))
          (error () nil))))
```

Changes: line 496 loses one `)`, line 497 gains one `)` to close the `when` block properly.

### STAB-02: All 9 Uncommitted Files

**Complete inventory of changes (from `git diff`):**

| # | File | Change | Category |
|---|------|--------|----------|
| 1 | `lisp/util/pg.lisp` | `(length str-val)` changed to `(length (sb-ext:string-to-octets str-val :external-format :utf-8))` in pg-escape | UTF-8 fix |
| 2 | `lisp/runtime/db-tasks.lisp` | Removed `description` from SELECT column list | Schema fix |
| 3 | `lisp/runtime/db-client.lisp` | Removed tilde `~` line continuations from SQL strings (replaced with single-line strings) | SQL format fix |
| 4 | `lisp/runtime/cognition-types.lisp` | Added `(stringp text)` guard to `parse-iso8601` | NULL/type safety |
| 5 | `lisp/runtime/task-scheduler.lisp` | Added `(eq scheduled-at :null)` check in `task-ready-p` | NULL handling |
| 6 | `lisp/runtime/db-auxiliary.lisp` | Wrapped `db-execute` SQL in `(format nil ...)` for drives decay; added `db-coerce-row` calls in `db-get-drives` | SQL format + type coercion |
| 7 | `lisp/runtime/db-conversations.lisp` | Added `db-coerce-row` calls for `:id` and `:to-agent` fields | Type coercion |
| 8 | `lisp/runtime/action-executor.lisp` | Wrapped `execute-cognition-result` cond in handler-case; added handler-case around `write-agent-daily-memory` | Error resilience |
| 9 | `lisp/packages.lisp` | Added `:db-coerce-row` import to db-auxiliary and db-conversations packages; added `:parse-json` import to db-conversations | Package exports |

### Recommended Commit Grouping

**Commit 1: fix(pg): use UTF-8 byte length in pg-escape for multi-byte strings**
- `lisp/util/pg.lisp` (1 file)
- Rationale: This is the most impactful fix -- pg-escape is called by every DB operation. Standalone, no dependencies on other changes.

**Commit 2: fix(db): remove description column, fix SQL formatting, add NULL guards**
- `lisp/runtime/db-tasks.lisp` (description column removal)
- `lisp/runtime/db-client.lisp` (tilde SQL formatting)
- `lisp/runtime/cognition-types.lisp` (stringp guard in parse-iso8601)
- `lisp/runtime/task-scheduler.lisp` (null check for scheduled-at)
- Rationale: All are query/data-handling fixes that prevent runtime errors from unexpected DB values.

**Commit 3: fix(db): add type coercion to drives and conversations queries**
- `lisp/runtime/db-auxiliary.lisp` (format nil wrap + db-coerce-row)
- `lisp/runtime/db-conversations.lisp` (db-coerce-row)
- `lisp/packages.lisp` (import additions for db-coerce-row and parse-json)
- Rationale: These three are tightly coupled -- packages.lisp exports what db-auxiliary and db-conversations import.

**Commit 4: fix(action-executor): fix paren scope in execute-work-task, add error handlers**
- `lisp/runtime/action-executor.lisp` (paren fix + handler-case wrapping)
- Rationale: The STAB-01 fix plus the handler-case wrapping of execute-cognition-result are both in the same file and both improve error resilience.

### Load Order (from launch.sh)

Files load in this exact sequence -- changes must not break earlier loads:

```
1.  packages.lisp          (package definitions - modified)
2.  util/json.lisp
3.  util/pg.lisp           (modified - UTF-8 fix)
4.  util/http.lisp
5.  runtime/db-client.lisp (modified - SQL formatting)
6.  runtime/cognition-types.lisp (modified - stringp guard)
7.  runtime/db-auxiliary.lisp    (modified - format nil + coercion)
8.  runtime/db-conversations.lisp (modified - coercion)
9.  runtime/db-tasks.lisp        (modified - description column)
10. runtime/task-scheduler.lisp   (modified - null check)
11. runtime/action-executor.lisp  (modified - paren fix + handlers)
```

All modifications are self-contained within each file's scope. The packages.lisp changes add imports that the downstream files need, so packages.lisp must be committed alongside or before the files that use the new imports.

### Verification Approach

1. **SBCL load test:** Run launch.sh and check for clean load (no compile errors/warnings). The process will start ticking -- that's expected.
2. **Tick cycle test:** Let at least one full tick complete. Watch PM2 logs for `[tick-error]`, `[action-error]`, or other error markers.
3. **Specific checks:**
   - No `PQescapeLiteral failed` errors (UTF-8 fix working)
   - No `description` column errors (column removed from queries)
   - No type errors in parse-iso8601 (stringp guard working)
   - Tool execution in execute-work-task doesn't crash silently (paren fix)

```bash
# Start the ghosts
pm2 restart noosphere-ghosts

# Watch logs for errors (give it 2-3 minutes for a tick cycle)
pm2 logs noosphere-ghosts --lines 100 --nostream | grep -i "error\|warning\|failed"

# Or tail live
pm2 logs noosphere-ghosts --lines 50
```

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Paren matching verification | Manual counting | `sbcl --load` (compiler catches mismatched parens) | SBCL's reader is authoritative on paren matching |
| UTF-8 byte length | `(loop for c across str counting ...)` | `(length (sb-ext:string-to-octets str :external-format :utf-8))` | SBCL's built-in handles all edge cases |
| SQL string escaping | Manual quote doubling | `PQescapeLiteral` via FFI (already in pg-escape) | libpq handles all PostgreSQL escaping rules |

## Common Pitfalls

### Pitfall 1: Committing packages.lisp separately from files that need new imports
**What goes wrong:** If packages.lisp is committed alone and the system is loaded from that commit, the new `:db-coerce-row` import resolves to nothing (no error at package definition time), but the files that call `db-coerce-row` haven't been committed yet. Not a load-order error but confusing.
**How to avoid:** Commit packages.lisp together with db-auxiliary.lisp and db-conversations.lisp.

### Pitfall 2: Tilde in format strings vs SQL
**What goes wrong:** Common Lisp `(format nil "~\n...")` with tilde-newline is a line continuation directive. When used inside SQL strings passed to `db-execute` (not via format), the tilde is literal and gets sent to PostgreSQL, causing SQL syntax errors.
**How to avoid:** The fix in db-client.lisp already addresses this by making SQL strings single-line. The fix in db-auxiliary.lisp wraps the SQL in `(format nil ...)` so tilde-newline is properly consumed.

### Pitfall 3: Testing on stopped process
**What goes wrong:** noosphere-ghosts is currently stopped (PM2 status: stopped). Starting it will run live ticks against the production database with real agents.
**How to avoid:** This is expected and desired per D-08 -- verification IS against the live system. Just be aware that ghosts will start ticking and consuming Claude API budget (~$0.50/request) once started.

### Pitfall 4: The paren fix changes handler-case semantics
**What goes wrong:** Currently `(error () nil)` is dead code. After the fix, it becomes an active error handler that silently swallows errors. This is intentional (the function should be resilient) but means tool-result DB write failures will be silently ignored.
**How to avoid:** Acceptable -- the handler-case pattern with `(error () nil)` is used throughout the codebase for non-critical operations. Tool results are supplementary; the main work output is already saved before this block.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | SBCL native load + live tick cycle (no formal test framework) |
| Config file | `/opt/project-noosphere-ghosts/launch.sh` |
| Quick run command | `cd /opt/project-noosphere-ghosts/lisp && sbcl --noinform --non-interactive --eval '(require :asdf)' --eval '(dolist (f (list "/opt/innatescript/src/packages" "/opt/innatescript/src/types" "/opt/innatescript/src/conditions" "/opt/innatescript/src/parser/tokenizer" "/opt/innatescript/src/parser/parser" "/opt/innatescript/src/eval/resolver" "/opt/innatescript/src/eval/evaluator")) (load (format nil "~a.lisp" f)))' --eval '(load "packages.lisp")' --eval '(dolist (f (list "util/json" "util/pg" ...)) (load (format nil "~a.lisp" f)))' --eval '(format t "Load OK~%")'` |
| Full suite command | `pm2 restart noosphere-ghosts && sleep 120 && pm2 logs noosphere-ghosts --lines 200 --nostream` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| STAB-01 | execute-work-task returns json-object from correct scope | smoke (SBCL load + tick) | `pm2 restart noosphere-ghosts` then check logs | N/A (live system) |
| STAB-02 | All fixes committed and loadable | smoke (SBCL load) | SBCL load all files without error | N/A (live system) |

### Sampling Rate
- **Per task commit:** SBCL load test (quick -- just verify files parse)
- **Per wave merge:** N/A (single wave phase)
- **Phase gate:** Full tick cycle completes without runtime errors

### Wave 0 Gaps
None -- there is no formal test framework for the Lisp tick engine. Verification is via SBCL compilation (catches syntax errors like paren mismatches) and live tick execution (catches runtime errors). This is the established pattern per D-08.

## Sources

### Primary (HIGH confidence)
- Direct code inspection of all 9 modified files via `git diff` in `/opt/project-noosphere-ghosts/`
- Paren depth analysis via automated counting of `action-executor.lisp` lines 488-497
- Launch.sh load order verification
- PM2 process status check (noosphere-ghosts currently stopped)

### Secondary (MEDIUM confidence)
- CONTEXT.md decisions (user-provided, locked)
- REQUIREMENTS.md (STAB-01, STAB-02 definitions)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, existing codebase only
- Architecture: HIGH -- all changes directly inspected via git diff, paren bug verified by automated counting
- Pitfalls: HIGH -- based on direct code inspection and understanding of CL semantics

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable -- code fixes, not library versions)
