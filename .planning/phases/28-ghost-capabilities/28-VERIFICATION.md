---
phase: 28-ghost-capabilities
verified: 2026-03-30T19:00:00Z
status: human_needed
score: 5/5 must-haves verified
re_verification:
  previous_status: gaps_found
  previous_score: 4/5
  gaps_closed:
    - "execute-proactive-work now has extract-responsibility-mutations block (lines 1015-1026, commit b652365)"
    - "build-proactive-job now appends *capability-mutation-instructions* to system-prompt (line 797, commit b652365)"
  gaps_remaining: []
  regressions: []
human_verification:
  - test: "Start noosphere-ghosts with pm2 restart noosphere-ghosts, let a ghost with YAML tick, confirm log shows '[planner-debug] nova: 3 capabilities from YAML (source: yaml)'"
    expected: "Log line shows yaml source, not tool-registry, for a ghost that has a YAML file"
    why_human: "Requires a live tick cycle with the ghosts running and LLM API active"
  - test: "Send a task to nova or ethan_ng that asks them to add a new capability. Check config/agents/nova.yaml after tick for new entry."
    expected: "YAML file updated with the new responsibility expression after ghost cognition processes the task"
    why_human: "Requires live LLM cognition producing responsibility_mutations in output — cannot simulate programmatically"
  - test: "Send a task to an executive (eliana) asking her to add a capability to a staff agent. Check the staff agent's YAML file."
    expected: "Staff agent's YAML file updated with new responsibility after executive tick"
    why_human: "Requires live LLM output with target_agent field — executive CAP-06 path needs live testing"
---

# Phase 28: Ghost Capabilities Verification Report

**Phase Goal:** Ghosts declare what they can do as InnateScipt expressions in their YAML, replacing the static tool registry for capability discovery and cognition
**Verified:** 2026-03-30T19:00:00Z
**Status:** human_needed
**Re-verification:** Yes — after gap closure (plan 28-04, commit b652365)

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A ghost's YAML file contains a `responsibilities:` section with valid InnateScipt expressions | VERIFIED | All 9 YAML files in `config/agents/` have `responsibilities:` sections with double-quoted InnateScipt expressions. |
| 2 | The tick engine reads capabilities from ghost YAML instead of tool-registry.json | VERIFIED | `action-planner.lisp` has `LOAD-GHOST-CAPABILITIES` wired in all 4 prompt builders (lines 328, 497, 585, 768). YAML-first with tool-registry fallback. |
| 3 | The action planner includes ghost's InnateScipt capabilities in LLM cognition prompts | VERIFIED | `format-capabilities-for-prompt` called in all 4 builders: `build-message-job`, `build-pipeline-task-job`, `build-task-job`, `build-proactive-job`. |
| 4 | A ghost can add, edit, or remove its own responsibility expressions via cognition output, with parse-round-trip validation | VERIFIED | `extract-responsibility-mutations` now wired in all three cognition paths: `execute-work-task` (line 537), `execute-proactive-work` (lines 1015-1026), and `execute-project-review` (line 1100). `build-proactive-job` now appends `*capability-mutation-instructions*` at line 797. Gap closed by commit b652365. |
| 5 | An executive ghost can modify a subordinate's responsibility expressions | VERIFIED | `process-responsibility-mutations` checks `:TARGET-AGENT`, verifies executive authorization via `agent-is-executive-p` + `validate-executive-target`. `execute-project-review` is wired for mutation extraction. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/project-noosphere-ghosts/lisp/util/yaml.lisp` | YAML parser with `parse-simple-yaml` | VERIFIED | 101 lines. Implements `split-lines`, `unquote-yaml-string`, `parse-simple-yaml`, `serialize-simple-yaml`. |
| `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` | Capability loading, formatting, mutation processing | VERIFIED | 224 lines. Has `load-ghost-capabilities`, `format-capabilities-for-prompt`, `validate-capability-list`, `write-ghost-yaml`, `agent-is-executive-p`, `validate-executive-target`, `process-single-mutation`, `process-responsibility-mutations`. |
| `/opt/project-noosphere-ghosts/config/agents/nova.yaml` | Nova capability declaration | VERIFIED | Contains `![query_db]`, `![pipeline_status]`, `![claude_code]`. |
| `/opt/project-noosphere-ghosts/config/agents/ethan_ng.yaml` | EthanNg flagship capability declaration | VERIFIED (deviation noted) | Contains `![fundamentals.feeds]` (dot notation, not colon-qualified). Documented deviation — colon-qualified syntax unsupported by InnateScipt tokenizer. Dot notation is valid and passes parse-round-trip. |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Capability injection into all 4 prompt builders + mutation instructions in proactive-job | VERIFIED | 4x `LOAD-GHOST-CAPABILITIES` + 4x `FORMAT-CAPABILITIES-FOR-PROMPT`. `*capability-mutation-instructions*` defined (line 47) and injected into work-task (line 611), project-review (line 1093), and proactive-job (line 797). |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` | Mutation extraction wired in all three cognition output paths | VERIFIED | `extract-responsibility-mutations` wired in `execute-work-task` (line 537), `execute-proactive-work` (lines 1015-1026), and `execute-project-review` (line 1100). |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `ghost-capabilities.lisp` | `yaml.lisp` | `parse-simple-yaml` call | WIRED | Line 33: `(af64.utils.yaml:parse-simple-yaml content)` |
| `ghost-capabilities.lisp` | `innate-builder.lisp` | `validate-innate-expression` | WIRED | Lines 66, 168, 197: `(af64.runtime.innate-builder:validate-innate-expression ...)` |
| `launch.sh` | `ghost-capabilities.lisp` | load order | WIRED | `"util/yaml"` loaded before `"runtime/ghost-capabilities"` |
| `action-planner.lisp build-proactive-job` | `ghost-capabilities.lisp` | `LOAD-GHOST-CAPABILITIES` | WIRED | Line 768: cap-prompt injected into proactive system-prompt |
| `action-planner.lisp build-proactive-job` | `*capability-mutation-instructions*` | format nil append | WIRED | Line 797: `(format nil "~a~@[~%~%~a~]~a" base-prompt cap-prompt *capability-mutation-instructions*)` |
| `action-executor.lisp execute-proactive-work` | `ghost-capabilities.lisp process-responsibility-mutations` | `PROCESS-RESPONSIBILITY-MUTATIONS` find-symbol | WIRED | Lines 1015-1026: handler-case with extract+process pattern. Commit b652365. |
| `action-executor.lisp execute-work-task` | `ghost-capabilities.lisp process-responsibility-mutations` | `PROCESS-RESPONSIBILITY-MUTATIONS` | WIRED | Lines 537-548: handler-case with extract+process pattern |
| `action-executor.lisp execute-project-review` | `ghost-capabilities.lisp process-responsibility-mutations` | `PROCESS-RESPONSIBILITY-MUTATIONS` | WIRED | Lines 1100-1111: same pattern |
| `ghost-capabilities.lisp write-ghost-yaml` | `config/agents/*.yaml` | atomic `rename-file` | WIRED | Line 94: writes temp file then `(rename-file temp-path yaml-path)` |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `action-planner.lisp build-message-job` | `cap-prompt` | `load-ghost-capabilities` → reads `config/agents/{id}.yaml` | Yes | FLOWING |
| `action-planner.lisp build-proactive-job` | `cap-prompt` + mutation instructions | `load-ghost-capabilities` → YAML file + `*capability-mutation-instructions*` | Yes | FLOWING |
| `action-executor.lisp execute-proactive-work` | `mutations` | `extract-responsibility-mutations` → parses LLM output JSON | Yes — extracts from real LLM response | FLOWING |
| `action-executor.lisp execute-work-task` | `mutations` | `extract-responsibility-mutations` → parses LLM output JSON | Yes | FLOWING |
| `ghost-capabilities.lisp process-responsibility-mutations` | YAML write | `write-ghost-yaml` → temp + rename to `config/agents/{id}.yaml` | Yes — writes real YAML atomically | FLOWING |

### Behavioral Spot-Checks

Step 7b: SKIPPED for live tick cycle (requires running noosphere-ghosts + active LLM API). Static code checks performed instead.

| Behavior | Check | Result | Status |
|----------|-------|--------|--------|
| YAML files parseable | All 9 have `id:` and `responsibilities:` sections | 9/9 confirmed | PASS |
| `parse-simple-yaml` handles double-quoted InnateScipt chars | `unquote-yaml-string` strips outer quotes | Confirmed at lines 24-31 | PASS |
| `format-capabilities-for-prompt` returns nil for empty | Guard on length > 0 | Line 49 confirmed | PASS |
| Atomic YAML write | `rename-file` pattern from temp path | Line 94 confirmed | PASS |
| Mutation block in execute-proactive-work | `extract-responsibility-mutations` + `PROCESS-RESPONSIBILITY-MUTATIONS` find-symbol | Lines 1015-1026 confirmed, commit b652365 | PASS |
| Mutation instructions in build-proactive-job | `*capability-mutation-instructions*` in format nil at line 797 | Confirmed, commit b652365 | PASS |
| Executive authorization checked | `agent-is-executive-p` + `validate-executive-target` in `process-single-mutation` | Lines 157-163 confirmed | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|---------|
| CAP-01 | 28-01 | Ghost YAML has `responsibilities:` section with InnateScipt expressions | SATISFIED | 9 YAML files with `responsibilities:` — all 22 expressions validated |
| CAP-02 | 28-01, 28-02 | Tick engine evaluates ghost YAML capabilities instead of tool-registry.json | SATISFIED | 4 prompt builders in action-planner.lisp use YAML-first with tool-registry fallback |
| CAP-03 | 28-02 | Action planner injects ghost's InnateScipt capabilities into LLM cognition prompts | SATISFIED | All 4 builders confirmed: build-message-job, build-pipeline-task-job, build-task-job, build-proactive-job |
| CAP-04 | 28-03, 28-04 | Ghost can write new InnateScipt expressions to its own responsibilities via cognition output | SATISFIED | Wired in execute-work-task, execute-proactive-work (gap closed), and execute-project-review. All three cognition paths support self-modification. |
| CAP-05 | 28-03, 28-04 | Ghost can edit/remove its own responsibility expressions via cognition output | SATISFIED | Same as CAP-04 — all three paths wired. Gap from initial verification closed by commit b652365. |
| CAP-06 | 28-03 | Executive ghost can edit subordinate ghost responsibilities | SATISFIED | `validate-executive-target` implemented; `execute-project-review` wired for mutation extraction; `*capability-mutation-instructions*` injected in project-review prompts |
| CAP-07 | 28-01, 28-03 | Capability changes validated via InnateScipt parse-round-trip before persistence | SATISFIED | `validate-innate-expression` called in `process-single-mutation` before every `write-ghost-yaml` call |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `config/agents/ethan_ng.yaml` | 2 | `![fundamentals.feeds]` vs plan-required `![fundamentals:feeds]` | Info | Documented deviation — InnateScipt tokenizer does not support colon-qualified search syntax. Dot notation is semantically equivalent and passes parse-round-trip. No functional regression. |

No blocker anti-patterns remain. The two blockers identified in initial verification (missing mutation extraction in execute-proactive-work and missing capability-mutation-instructions in build-proactive-job) were resolved by commit b652365.

### Human Verification Required

#### 1. YAML Capability Source Confirmation

**Test:** Start noosphere-ghosts (`pm2 restart noosphere-ghosts`), wait for a tick, look at tick logs for a ghost with a YAML file (nova, eliana, etc.)
**Expected:** Log line `[planner-debug] nova: 3 capabilities from YAML (source: yaml)` confirming YAML path is taken, not tool-registry
**Why human:** Requires live tick cycle with LLM API active

#### 2. Ghost Self-Modification via Proactive-Work Path (CAP-04/05 gap closure verification)

**Test:** Observe a tick where a ghost with YAML runs proactive-work cognition. Check tick logs for `[cap-mutations] <agent-id>: N mutation(s) found` or `[cap-mutations] <agent-id>: N mutation(s) applied`.
**Expected:** Log line confirms the newly wired path is executing the mutation extraction block
**Why human:** Requires live LLM cognition producing `responsibility_mutations` JSON in proactive-work output

#### 3. Ghost Self-Modification via Work-Task

**Test:** Send a task to nova or ethan_ng asking them to add a new capability: "Add a new capability expression `![test_capability]` to your responsibilities." Check `config/agents/nova.yaml` after the tick.
**Expected:** nova.yaml updated with the new responsibility expression after cognition processes the task
**Why human:** Requires live LLM cognition producing `responsibility_mutations` JSON in output

#### 4. Executive Subordinate Modification (CAP-06)

**Test:** Send a message to eliana asking her to add capability `![build_rust]` to a staff agent's YAML. Monitor tick log for `[cap-mutation] eliana add:` log line. Check the target staff agent's YAML file.
**Expected:** Staff agent's YAML file updated with new responsibility; log confirms executive authorization path taken
**Why human:** Requires live LLM output with `target_agent` field — executive CAP-06 path needs live testing

### Re-verification Summary

Gap closure confirmed. Both fixes from plan 28-04 are present in commit b652365:

1. `execute-proactive-work` (action-executor.lisp lines 1015-1026): The `extract-responsibility-mutations` + `PROCESS-RESPONSIBILITY-MUTATIONS` handler-case block is present, matching the same pattern as `execute-work-task`. The comment `; Process responsibility mutations (Phase 28: CAP-04/05 gap closure)` confirms the intent.

2. `build-proactive-job` (action-planner.lisp line 797): The format string `(format nil "~a~@[~%~%~a~]~a" base-prompt cap-prompt *capability-mutation-instructions*)` now appends `*capability-mutation-instructions*` — matching the same pattern as the work-task and project-review builders.

All 7 requirements (CAP-01 through CAP-07) are now SATISFIED. The phase goal is achieved: ghosts declare capabilities via YAML, the tick engine reads and injects those capabilities into cognition prompts, and all three cognition paths (work-task, proactive-work, project-review) support ghost self-modification of responsibility expressions with parse-round-trip validation.

---

_Initially verified: 2026-03-30T18:00:00Z_
_Re-verified: 2026-03-30T19:00:00Z_
_Verifier: Claude (gsd-verifier)_
