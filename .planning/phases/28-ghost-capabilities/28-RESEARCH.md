# Phase 28: Ghost Capabilities - Research

**Researched:** 2026-03-30
**Domain:** Common Lisp YAML parsing, tick engine capability injection, LLM cognition prompts, file-based agent configuration
**Confidence:** HIGH

## Summary

Phase 28 replaces the scope-based tool-registry.json lookup with per-ghost YAML files containing InnateScipt responsibility expressions. The implementation touches four Lisp source files: a new YAML parser utility, modifications to action-planner.lisp (capability injection into prompts), modifications to action-executor.lisp (responsibility mutation extraction), and new symbol exports in packages.lisp. The existing `validate-innate-expression` from innate-builder.lisp provides parse-round-trip validation.

The key technical challenge is YAML parsing in Common Lisp without Quicklisp (the project uses zero-deps, direct file loading with SB-ALIEN FFI). For the minimal YAML structure required (flat key + list of strings), a purpose-built parser of ~60 lines is the correct approach -- matching the project's existing patterns (json.lisp, pg.lisp are all hand-rolled).

**Primary recommendation:** Write a minimal YAML parser (~60 lines) in `lisp/util/yaml.lisp` that handles only the subset needed: scalar keys, string lists, and comments. Wire it into the existing action-planner prompt builders to replace `get-tools-for-agent` / `format-tools-for-prompt` with YAML-sourced capability declarations.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: Ghost YAML files at `/opt/project-noosphere-ghosts/config/agents/{agent_id}.yaml` -- one file per ghost
- D-02: YAML structure with `id:` and `responsibilities:` list of InnateScipt expression strings
- D-03: Each responsibility must pass parse-round-trip validation
- D-04: Only `responsibilities:` section in scope -- persona, department, etc. are out of scope
- D-05: Each responsibility is a valid InnateScipt expression string (action `![]`, bundle `{}`, reference `@()`)
- D-06: Expressions are NOT evaluated at YAML load time -- injected into LLM prompts as declarations
- D-07: Example responsibilities for key ghosts defined (EthanNg, Sylvia, Nova, Eliana)
- D-08: New function `load-ghost-capabilities` reads YAML file, returns list of expression strings
- D-09: Action planner replaces `get-tools-for-agent` with `load-ghost-capabilities` (for YAML-equipped ghosts)
- D-10: `format-tools-for-prompt` replaced with `format-capabilities-for-prompt`
- D-11: tool-registry.json continues as fallback -- ghosts WITHOUT YAML use old scope matching
- D-12: Cognition output includes `responsibility_add`, `responsibility_remove`, `responsibility_edit` mutations
- D-13: Mutations validated via InnateScipt parse-round-trip before writing
- D-14: Self-modification and executive modification use same validation path
- D-15: YAML writes are atomic -- read, modify, write entire file

### Claude's Discretion
- YAML parsing library choice (cl-yaml or custom parser)
- Exact cognition output format for responsibility mutations
- Which ghosts get initial YAML files
- Error handling when YAML file doesn't exist

### Deferred Ideas (OUT OF SCOPE)
- Full tool-registry.json retirement -- Phase 31
- YAML sections beyond responsibilities -- future milestone
- Migrating DB frontmatter responsibilities to YAML
- Tool execution through InnateScipt commission -- Phase 31
- YAML-defined pipeline handoff chains -- Phase 30
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CAP-01 | Ghost YAML has responsibilities: section with InnateScipt expressions | Custom YAML parser reads `responsibilities:` list; `validate-innate-expression` validates each entry |
| CAP-02 | Tick engine evaluates ghost YAML capabilities instead of tool-registry.json | `load-ghost-capabilities` replaces `get-tools-for-agent` in action-planner.lisp lines 454-462 |
| CAP-03 | Action planner injects ghost's InnateScipt capabilities into LLM prompts | `format-capabilities-for-prompt` replaces `format-tools-for-prompt` in system prompt composition |
| CAP-04 | Ghost can write new InnateScipt expressions to own responsibilities | `responsibility_add` mutation parsed from cognition output, validated, written to YAML |
| CAP-05 | Ghost can edit/remove own responsibility expressions | `responsibility_edit`/`responsibility_remove` mutations with same validation path |
| CAP-06 | Executive can edit subordinate ghost responsibilities | Same mutation functions with target agent-id parameter; executive check via `agent-is-executive-p` |
| CAP-07 | Capability changes validated via InnateScipt parse-round-trip | Reuses `validate-innate-expression` from innate-builder.lisp (Phase 25) |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| SBCL | current | Common Lisp runtime | Project's only CL implementation |
| InnateScipt parser | v1.0 | Parse-round-trip validation | Already at `/opt/innatescript/`, 175/176 tests passing |
| innate-builder.lisp | Phase 25 | `validate-innate-expression` function | Proven parse-round-trip validation pattern |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| uiop | bundled | File I/O (`read-file-string`) | Reading/writing YAML files |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Custom YAML parser | cl-yaml (Quicklisp) | Project is zero-deps, no Quicklisp -- custom parser is the only viable choice |
| YAML files | JSON files | YAML is human-readable for Nathan to edit; JSON would require json.lisp but lose readability |

## Architecture Patterns

### Recommended Project Structure
```
config/
  agents/                    # NEW directory
    nova.yaml                # One file per ghost
    eliana.yaml
    kathryn.yaml
    ...
  tool-registry.json         # RETAINED as fallback (Phase 31 removes)
  provider-config.json       # Unchanged
lisp/
  util/
    yaml.lisp                # NEW: minimal YAML parser
  runtime/
    ghost-capabilities.lisp  # NEW: load-ghost-capabilities, format-capabilities-for-prompt, write mutations
    action-planner.lisp      # MODIFIED: replace tool injection with capability injection
    action-executor.lisp     # MODIFIED: extract responsibility mutations from cognition output
    tool-socket.lisp         # UNCHANGED (fallback path still works)
  packages.lisp              # MODIFIED: new package + exports
```

### Pattern 1: Minimal YAML Parser
**What:** A ~60 line parser handling only: comments (#), scalar values (`key: value`), and string lists (`- "value"`).
**When to use:** When the YAML structure is fixed and simple -- `id:` scalar + `responsibilities:` list of quoted strings.
**Example:**
```lisp
;; Parses YAML into hash-table with string keys
;; "id" -> "nova"
;; "responsibilities" -> ("![query_db]" "![pipeline_status]" "![claude_code]")
(defun parse-simple-yaml (text)
  "Parse a minimal YAML document. Supports: comments, scalars, string lists."
  (let ((result (make-hash-table :test #'equal))
        (current-key nil)
        (current-list nil))
    (dolist (raw-line (split-lines text))
      (let ((line (string-trim '(#\Space #\Tab #\Return) raw-line)))
        (cond
          ;; Skip empty lines and comments
          ((or (= (length line) 0) (char= (char line 0) #\#))
           nil)
          ;; List item: "  - "value""
          ((and current-key (>= (length line) 2)
                (char= (char line 0) #\-)
                (char= (char line 1) #\Space))
           (push (unquote-yaml-string (string-trim '(#\Space) (subseq line 2)))
                 current-list))
          ;; Key: value
          ((let ((colon-pos (position #\: line)))
             (when colon-pos
               ;; Flush previous list
               (when (and current-key current-list)
                 (setf (gethash current-key result) (nreverse current-list)))
               (let ((key (subseq line 0 colon-pos))
                     (val (string-trim '(#\Space) (subseq line (1+ colon-pos)))))
                 (setf current-key key)
                 (setf current-list nil)
                 (when (> (length val) 0)
                   (setf (gethash key result) (unquote-yaml-string val))
                   (setf current-key nil))
                 t)))))))
    ;; Flush final list
    (when (and current-key current-list)
      (setf (gethash current-key result) (nreverse current-list)))
    result))
```

### Pattern 2: Capability Injection into Prompts
**What:** Replace tool-registry scope matching with YAML-sourced InnateScipt expressions in LLM system prompts.
**When to use:** Every cognition job that currently calls `get-tools-for-agent` / `format-tools-for-prompt`.
**Example:**
```lisp
(defun format-capabilities-for-prompt (capabilities)
  "Format ghost's InnateScipt capabilities as a prompt section."
  (when capabilities
    (with-output-to-string (s)
      (format s "YOUR CAPABILITIES (InnateScipt expressions you can use):~%")
      (dolist (cap capabilities)
        (format s "  - ~a~%" cap))
      (format s "~%These define what you can do. When relevant, use these capabilities in your work.~%"))))
```

### Pattern 3: Responsibility Mutation Extraction
**What:** Parse `responsibility_add`/`responsibility_edit`/`responsibility_remove` from cognition output, similar to existing `extract-innate-expressions` pattern.
**When to use:** In `execute-work-task` and `execute-proactive-work`, after existing innate expression processing.
**Example:**
```lisp
;; Cognition output format (JSON block in LLM response):
;; "responsibility_mutations": [
;;   {"action": "add", "expression": "![new_capability]"},
;;   {"action": "remove", "expression": "![old_capability]"},
;;   {"action": "edit", "old": "![old_expr]", "new": "![updated_expr]"},
;;   {"action": "add", "expression": "![delegated_cap]", "target_agent": "ethan_ng"}
;; ]
```

### Pattern 4: Atomic YAML Write
**What:** Read entire YAML file, modify the responsibilities list in memory, write entire file back atomically.
**When to use:** All responsibility mutations (add/edit/remove).
**Example:**
```lisp
(defun write-ghost-yaml (agent-id yaml-data)
  "Atomically write ghost YAML file. Writes to temp file, then renames."
  (let* ((yaml-path (ghost-yaml-path agent-id))
         (temp-path (format nil "~a.tmp" yaml-path)))
    (with-open-file (s temp-path :direction :output :if-exists :supersede)
      (format s "id: ~a~%" agent-id)
      (format s "responsibilities:~%")
      (dolist (r (gethash "responsibilities" yaml-data))
        (format s "  - \"~a\"~%" r)))
    (rename-file temp-path yaml-path)))
```

### Anti-Patterns to Avoid
- **Evaluating expressions at load time:** D-06 explicitly says expressions are NOT evaluated -- they're string declarations for LLM prompts
- **Modifying tool-socket.lisp for YAML:** Keep tool-socket.lisp as the fallback path; new capability code goes in a separate ghost-capabilities.lisp
- **Using `*tool-registry*` hash-table for capabilities:** Capabilities are per-ghost YAML files, not a global registry

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| InnateScipt validation | Custom expression validator | `validate-innate-expression` from innate-builder.lisp | Already handles tokenize -> parse -> round-trip with error reporting |
| Executive check | Custom role lookup | `agent-is-executive-p` from action-planner.lisp | Already checks tool_scope for "decision" membership |
| DB agent lookup | Custom SQL | `db-get-agent-by-id` from db-auxiliary.lisp | Returns agent data including department, tool_scope |
| JSON parsing of mutations | Custom extractor | Pattern from `extract-innate-expressions` in action-executor.lisp | Same JSON-block-in-LLM-output extraction pattern |

## Common Pitfalls

### Pitfall 1: JSON Parser Underscore-to-Hyphen Conversion
**What goes wrong:** The AF64 JSON parser converts underscores to hyphens and upcases keywords. So `responsibility_add` becomes `:RESPONSIBILITY-ADD`.
**Why it happens:** `json.lisp` normalizes all JSON keys to CL keyword convention.
**How to avoid:** Always use hyphenated, uppercased keywords when accessing parsed JSON: `:RESPONSIBILITY-ADD`, `:RESPONSIBILITY-REMOVE`, `:RESPONSIBILITY-EDIT`, `:TARGET-AGENT`.
**Warning signs:** Hash-table lookups returning nil unexpectedly.

### Pitfall 2: YAML String Quoting
**What goes wrong:** InnateScipt expressions contain special YAML characters: `!`, `{`, `}`, `[`, `]`, `@`, `(`, `)`.
**Why it happens:** Unquoted YAML strings with these characters cause parse ambiguity. `![query_db]` starts with `!` (YAML tag indicator) and contains `[]` (flow sequence).
**How to avoid:** All responsibility strings MUST be double-quoted in YAML. The parser must handle quoted strings. The writer must always emit quoted strings.
**Warning signs:** Expressions parsed as YAML tags or sequences instead of strings.

### Pitfall 3: Race Condition on YAML Write
**What goes wrong:** Two ghosts (or a ghost and its executive) modify the same YAML file simultaneously.
**Why it happens:** Tick engine processes multiple agents per tick.
**How to avoid:** Use atomic write (write to temp file, rename). Since SBCL is single-threaded in this tick engine, concurrent modification within a tick is unlikely -- but atomic writes protect against crashes mid-write.
**Warning signs:** Truncated or corrupt YAML files.

### Pitfall 4: Missing YAML Fallback
**What goes wrong:** Ghost without a YAML file gets no capabilities at all (nil prompt section).
**Why it happens:** D-11 requires fallback to tool-registry.json for ghosts without YAML.
**How to avoid:** `load-ghost-capabilities` must check for YAML file existence first, fall back to `get-tools-for-agent` if missing.
**Warning signs:** Ghosts suddenly losing tool access after Phase 28 deployment.

### Pitfall 5: Package Export Ordering
**What goes wrong:** New package `af64.runtime.ghost-capabilities` not available when action-planner tries to import from it.
**Why it happens:** launch.sh loads files in explicit order. New file must be inserted correctly.
**How to avoid:** Add ghost-capabilities.lisp to launch.sh load sequence AFTER innate-builder (needs `validate-innate-expression`) and BEFORE action-planner (which calls the new functions).
**Warning signs:** SBCL "package not found" or "symbol not found" errors at startup.

## Code Examples

### Current Tool Injection (Being Replaced)
```lisp
;; action-planner.lisp lines 454-462 (inside build-pipeline-task-job)
(agent-tools (handler-case
                  (let ((fn (find-symbol "GET-TOOLS-FOR-AGENT" :af64.runtime.action-executor)))
                    (if fn (funcall fn agent-id) '()))
                (error () '())))
(tools-prompt (handler-case
                   (let ((fn (find-symbol "FORMAT-TOOLS-FOR-PROMPT" :af64.runtime.action-executor)))
                     (if fn (funcall fn agent-tools) nil))
                 (error () nil)))
```

### Current Innate Expression Extraction (Pattern to Follow)
```lisp
;; action-executor.lisp lines 35-56
(defun extract-innate-expressions (content)
  "Extract innate_expressions JSON array from LLM output content."
  (handler-case
      (let ((key-pos (search "\"innate_expressions\"" content)))
        (when key-pos
          (let ((arr-start (position #\[ content :start key-pos)))
            (when arr-start
              ;; ... bracket matching, JSON parsing ...
              ))))
    (error () nil)))
```

### Current DB Responsibilities (Stays as Perception Source)
```lisp
;; db-client.lisp lines 398-426
(defun db-perceive-responsibilities (pool agent-id)
  "Fetch responsibilities, goals, content-focus, department, role from agent document frontmatter."
  ;; Returns hash-table with :responsibilities :goals :content-focus :department :role
  ;; This STAYS -- provides goals/role/department for proactive prompts
  ;; YAML capabilities AUGMENT this, not replace it
  )
```

### Where Tools Are Injected (All Modification Points)
```
1. build-pipeline-task-job (line 454-462): GET-TOOLS-FOR-AGENT + FORMAT-TOOLS-FOR-PROMPT
   -> Replace with load-ghost-capabilities + format-capabilities-for-prompt

2. build-task-job (line 542): Only appends *innate-generation-instructions*
   -> Add capability injection here too

3. build-proactive-job (line 683-739): Uses responsibilities from DB perception
   -> Merge YAML capabilities into the prompt alongside DB responsibilities

4. build-message-job (line 316): Has tools in system prompt for respond_message
   -> Add capabilities here for tool-aware message responses
```

## Existing Integration Points Detail

### action-planner.lisp Modification Points
| Line | Function | Current Behavior | Phase 28 Change |
|------|----------|-----------------|-----------------|
| 454-462 | build-pipeline-task-job | `find-symbol GET-TOOLS-FOR-AGENT` | Replace with `load-ghost-capabilities` + fallback |
| 542 | build-task-job | Appends innate instructions only | Add capability prompt injection |
| 683-739 | build-proactive-job | Uses DB responsibilities string | Merge YAML capabilities |
| 316 | build-message-job | No capability awareness | Add capabilities for tool-aware responses |

### action-executor.lisp Modification Points
| Line | Function | Current Behavior | Phase 28 Change |
|------|----------|-----------------|-----------------|
| 502-508 | execute-work-task | Processes innate expressions | Add responsibility mutation processing |
| 624 | execute-proactive-work | Similar to work-task | Add responsibility mutation processing |

### packages.lisp Additions
New package `af64.runtime.ghost-capabilities`:
- Imports: `validate-innate-expression` from innate-builder, `uiop:read-file-string`, `db-get-agent-by-id`
- Exports: `load-ghost-capabilities`, `format-capabilities-for-prompt`, `process-responsibility-mutations`, `write-ghost-yaml`, `ghost-yaml-path`

### launch.sh Addition
Insert `"runtime/ghost-capabilities"` in the load sequence between `"runtime/innate-builder"` and `"runtime/provider-adapters"` (or anywhere before `"runtime/action-planner"`).

## Agent YAML File Inventory

### Recommended Initial Set (All 8 Executives + EthanNg)
| Agent ID | Department | Example Responsibilities |
|----------|-----------|------------------------|
| nova | Systems | `![query_db]`, `![pipeline_status]`, `![claude_code]` |
| eliana | Engineering | `![build_tool]`, `![claude_code]`, `![query_db]` |
| kathryn | Executive | `![market_scanner]`, `{em.strategy.portfolio}` |
| sylvia | Content & Brand | `![write_document]`, `{em.content.blog}`, `{em.content.thought-police}` |
| vincent | Creative | `![write_document]`, `{em.creative.covers}` |
| jmax | Legal | `![query_db]`, `{em.legal.compliance}` |
| lrm | Research | `![query_db]`, `{em.music.archive}` |
| sarah | Office of CEO | `![query_db]` |
| ethan_ng | Strategic Office | `![fundamentals:feeds]`, `![technicals:oanda_api[pairs: major+minor]]`, `{em.content.podcast}` |

Total agent count in DB: 64 (8 executives + 56 staff). Phase 28 creates YAML for 9 agents; remaining 55 use tool-registry.json fallback per D-11.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual SBCL REPL validation (no automated test framework in noosphere-ghosts) |
| Config file | None -- project uses manual testing |
| Quick run command | `sbcl --load test-snippet.lisp` |
| Full suite command | Restart `pm2 restart noosphere-ghosts` and observe tick logs |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CAP-01 | YAML has responsibilities section | manual | Verify YAML files parse correctly via REPL | No -- Wave 0 |
| CAP-02 | Tick engine reads YAML instead of tool-registry | smoke | Start tick engine, verify log shows "capabilities from YAML" | No -- Wave 0 |
| CAP-03 | Capabilities injected into LLM prompts | smoke | Check tick log for capability text in prompt | No -- Wave 0 |
| CAP-04 | Ghost adds responsibility via cognition | integration | Send test message triggering responsibility_add | No -- manual |
| CAP-05 | Ghost edits/removes responsibility | integration | Send test message triggering edit/remove | No -- manual |
| CAP-06 | Executive modifies subordinate capabilities | integration | Send message to executive targeting subordinate | No -- manual |
| CAP-07 | Parse-round-trip validation on mutations | unit | REPL test: `(validate-innate-expression "![test]")` | Exists in innate-builder |

### Sampling Rate
- **Per task commit:** Manual REPL validation of new functions
- **Per wave merge:** `pm2 restart noosphere-ghosts` -- watch 2-3 tick cycles for errors
- **Phase gate:** Full tick cycle with at least one ghost loading capabilities from YAML

### Wave 0 Gaps
- [ ] No automated test suite for ghost-capabilities.lisp -- test via REPL
- [ ] yaml.lisp parser needs REPL-level verification against sample YAML files
- [ ] Need to verify YAML file permissions allow SBCL process to read/write `config/agents/`

## Open Questions

1. **Which tool expressions do 55 staff agents need?**
   - What we know: 8 executives + ethan_ng get initial YAML files per discussion
   - What's unclear: Whether any staff need YAML immediately (e.g., lara/sarah for triage)
   - Recommendation: Start with 9 per discussion; add staff YAML files incrementally as needed

2. **Should capabilities appear in respond_message prompts?**
   - What we know: Currently tools ARE injected in build-pipeline-task-job. build-message-job has tool_call format in system prompt (line 316) but no actual tool list.
   - What's unclear: Whether message responses need capability awareness
   - Recommendation: Include capabilities in message prompts -- ghosts should know what they can do when responding to messages

3. **Mutation target validation for executive editing subordinates**
   - What we know: D-14 says same validation path, D-06 specifies executive = edits subordinate's YAML
   - What's unclear: Whether to validate that the executive actually supervises the target agent
   - Recommendation: Check that target agent's department matches executive's department OR executive has "all" scope (Nova)

## Sources

### Primary (HIGH confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` -- Full source of `get-tools-for-agent`, `format-tools-for-prompt`, `load-tool-registry` (324 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` -- All prompt builder functions, tool injection points (1051 lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` -- `execute-work-task`, `extract-innate-expressions`, `process-innate-expressions` (625+ lines)
- `/opt/project-noosphere-ghosts/lisp/runtime/innate-builder.lisp` -- `validate-innate-expression`, template CRUD (123 lines)
- `/opt/project-noosphere-ghosts/lisp/packages.lisp` -- All package definitions, export lists, import chains (369 lines)
- `/opt/project-noosphere-ghosts/launch.sh` -- Load order for all Lisp files
- `/opt/project-noosphere-ghosts/config/tool-registry.json` -- 1264 lines, scope-based tool definitions
- master_chronicle `agents` table -- 64 agents (8 exec, 56 staff) with tool_scope, department

### Secondary (MEDIUM confidence)
- `/opt/project-noosphere-ghosts/lisp/runtime/db-client.lisp` -- `db-perceive-responsibilities` (reads frontmatter, stays as perception source)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all code is in-repo, fully inspected
- Architecture: HIGH -- modification points precisely identified with line numbers
- Pitfalls: HIGH -- JSON parser quirk, YAML quoting, and load order are all proven issues in this codebase
- YAML parser approach: HIGH -- zero-deps pattern matches json.lisp, pg.lisp, http.lisp precedent

**Research date:** 2026-03-30
**Valid until:** 2026-04-30 (stable -- internal codebase, no external dependency churn)
