---
phase: 28-ghost-capabilities
verified: 2026-03-30T18:00:00Z
status: gaps_found
score: 4/5 must-haves verified
gaps:
  - truth: "A ghost can add, edit, or remove its own responsibility expressions via cognition output, with parse-round-trip validation"
    status: partial
    reason: "Mutation extraction is wired in execute-work-task but NOT in execute-proactive-work. Capability mutation instructions are NOT injected into build-proactive-job prompts. Ghosts in the proactive-work path cannot self-modify capabilities."
    artifacts:
      - path: "/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp"
        issue: "execute-proactive-work (line 986) has no extract-responsibility-mutations block. Only execute-work-task (line 537) and execute-project-review (line 1100) are wired."
      - path: "/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp"
        issue: "build-proactive-job (line 752) does not append *capability-mutation-instructions* to the system-prompt. The proactive_work cognition prompt does not tell ghosts they can output mutations."
    missing:
      - "Add extract-responsibility-mutations block to execute-proactive-work in action-executor.lisp (same pattern as execute-work-task line 537-548)"
      - "Append *capability-mutation-instructions* to the system-prompt in build-proactive-job in action-planner.lisp (same pattern as work-task prompt on line 611)"
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
**Verified:** 2026-03-30T18:00:00Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A ghost's YAML file contains a `responsibilities:` section with valid InnateScipt expressions | VERIFIED | All 9 YAML files in `config/agents/` have `responsibilities:` sections with double-quoted InnateScipt expressions. Expressions use validated dot-notation syntax (e.g., `![fundamentals.feeds]` not `![fundamentals:feeds]`). |
| 2 | The tick engine reads capabilities from ghost YAML instead of tool-registry.json | VERIFIED | `action-planner.lisp` has `LOAD-GHOST-CAPABILITIES` wired in all 4 prompt builders (lines 328, 497, 585, 768). YAML-first with tool-registry fallback when YAML returns nil. Debug log shows "yaml" vs "tool-registry" source. |
| 3 | The action planner includes ghost's InnateScipt capabilities in LLM cognition prompts | VERIFIED | `format-capabilities-for-prompt` called in all 4 builders: `build-message-job` (line 333), `build-pipeline-task-job` (line 502), `build-task-job` (line 590), `build-proactive-job` (line 773). Capability list formatted as "YOUR CAPABILITIES (InnateScipt expressions you can use):" section. |
| 4 | A ghost can add, edit, or remove its own responsibility expressions via cognition output, with parse-round-trip validation | PARTIAL | `extract-responsibility-mutations` wired in `execute-work-task` (line 537) and `execute-project-review` (line 1100) — but NOT in `execute-proactive-work`. `build-proactive-job` does NOT inject `*capability-mutation-instructions*`, so proactive-work prompts never tell ghosts to output mutations. The work-task and project-review paths are fully wired. |
| 5 | An executive ghost can modify a subordinate's responsibility expressions | VERIFIED | `process-responsibility-mutations` checks `:TARGET-AGENT`, verifies executive authorization via `agent-is-executive-p` + `validate-executive-target`, allows mutation if executive's department matches or if nova/sarah. `execute-project-review` (the executive path) is wired for mutation extraction. `*capability-mutation-instructions*` injected in project-review prompt (action-planner.lisp line 1095). |

**Score:** 4/5 truths verified (1 partial)

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/project-noosphere-ghosts/lisp/util/yaml.lisp` | YAML parser with `parse-simple-yaml` | VERIFIED | 101 lines. Implements `split-lines`, `unquote-yaml-string`, `parse-simple-yaml`, `serialize-simple-yaml`. Handles double-quoted strings with InnateScipt special chars. |
| `/opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp` | Capability loading, formatting, mutation processing | VERIFIED | 224 lines. Has `load-ghost-capabilities`, `format-capabilities-for-prompt`, `validate-capability-list`, `write-ghost-yaml`, `agent-is-executive-p`, `validate-executive-target`, `process-single-mutation`, `process-responsibility-mutations`. |
| `/opt/project-noosphere-ghosts/config/agents/nova.yaml` | Nova capability declaration | VERIFIED | Contains `![query_db]`, `![pipeline_status]`, `![claude_code]` per D-07. |
| `/opt/project-noosphere-ghosts/config/agents/ethan_ng.yaml` | EthanNg flagship capability declaration | PARTIAL | Contains `![fundamentals.feeds]` (not `![fundamentals:feeds]` as plan specified). Deviation documented in 28-01-SUMMARY.md: colon-qualified syntax unsupported by InnateScipt tokenizer. Dot notation is valid and passes parse-round-trip. Semantically equivalent. |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | Capability injection into all 4 prompt builders | VERIFIED | 4x `LOAD-GHOST-CAPABILITIES` + 4x `FORMAT-CAPABILITIES-FOR-PROMPT`. `*capability-mutation-instructions*` defined (line 47) and injected into work-task and project-review prompts. |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp` | Mutation extraction wired in cognition output processing | PARTIAL | `extract-responsibility-mutations` (line 63) exists and is wired in `execute-work-task` (line 537) and `execute-project-review` (line 1100). NOT wired in `execute-proactive-work` (line 986). |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `ghost-capabilities.lisp` | `yaml.lisp` | `parse-simple-yaml` call | WIRED | Line 33 of ghost-capabilities.lisp: `(af64.utils.yaml:parse-simple-yaml content)` |
| `ghost-capabilities.lisp` | `innate-builder.lisp` | `validate-innate-expression` | WIRED | Lines 66, 168, 197 of ghost-capabilities.lisp: `(af64.runtime.innate-builder:validate-innate-expression ...)` |
| `launch.sh` | `ghost-capabilities.lisp` | load order | WIRED | `"util/yaml"` loaded after `"util/http"`, `"runtime/ghost-capabilities"` loaded after `"runtime/innate-builder"` and before `"runtime/provider-adapters"` |
| `action-planner.lisp build-pipeline-task-job` | `ghost-capabilities.lisp` | `LOAD-GHOST-CAPABILITIES` | WIRED | Line 497: find-symbol pattern with tool-registry fallback |
| `action-planner.lisp build-task-job` | `ghost-capabilities.lisp` | `FORMAT-CAPABILITIES-FOR-PROMPT` | WIRED | Line 590: cap-prompt injected into system-prompt |
| `action-planner.lisp build-proactive-job` | `ghost-capabilities.lisp` | `LOAD-GHOST-CAPABILITIES` | WIRED | Line 768: cap-prompt injected into proactive system-prompt |
| `action-planner.lisp build-message-job` | `ghost-capabilities.lisp` | `FORMAT-CAPABILITIES-FOR-PROMPT` | WIRED | Line 333: cap-prompt in message system-prompt |
| `action-executor.lisp execute-work-task` | `ghost-capabilities.lisp process-responsibility-mutations` | `PROCESS-RESPONSIBILITY-MUTATIONS` find-symbol | WIRED | Lines 537-548: handler-case with extract+process pattern |
| `action-executor.lisp execute-proactive-work` | `ghost-capabilities.lisp process-responsibility-mutations` | `PROCESS-RESPONSIBILITY-MUTATIONS` | NOT WIRED | `execute-proactive-work` (line 986) has no mutation extraction block |
| `action-executor.lisp execute-project-review` | `ghost-capabilities.lisp process-responsibility-mutations` | `PROCESS-RESPONSIBILITY-MUTATIONS` | WIRED | Lines 1100-1111: same pattern as execute-work-task |
| `ghost-capabilities.lisp write-ghost-yaml` | `config/agents/*.yaml` | atomic `rename-file` | WIRED | Line 94: writes temp file then `(rename-file temp-path yaml-path)` |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `action-planner.lisp build-message-job` | `cap-prompt` | `load-ghost-capabilities` → reads `config/agents/{id}.yaml` | Yes — reads actual YAML file from disk | FLOWING |
| `action-planner.lisp build-proactive-job` | `cap-prompt` | `load-ghost-capabilities` → reads `config/agents/{id}.yaml` | Yes — reads actual YAML file from disk | FLOWING |
| `action-executor.lisp execute-work-task` | `mutations` | `extract-responsibility-mutations` → parses LLM output JSON | Yes — extracts from real LLM response content | FLOWING |
| `ghost-capabilities.lisp process-responsibility-mutations` | YAML write | `write-ghost-yaml` → temp + rename to `config/agents/{id}.yaml` | Yes — writes real YAML to disk atomically | FLOWING |

### Behavioral Spot-Checks

Step 7b: SKIPPED for live tick cycle (requires running noosphere-ghosts + active LLM API). Static code checks performed instead.

| Behavior | Check | Result | Status |
|----------|-------|--------|--------|
| YAML files parseable | All 9 have `id:` and `responsibilities:` sections | 9/9 confirmed | PASS |
| `parse-simple-yaml` handles double-quoted InnateScipt chars | Implementation uses `unquote-yaml-string` stripping outer quotes | Lines 24-31 confirmed | PASS |
| `format-capabilities-for-prompt` returns nil for empty | Guard `(when (and capabilities (listp ...) (> (length ..) 0)))` | Line 49 confirmed | PASS |
| Atomic YAML write | `rename-file` pattern from temp path | Line 94 confirmed | PASS |
| `load-ghost-capabilities` returns nil on missing file | `(when (probe-file path) ...)` handler-case | Lines 31-38 confirmed | PASS |
| Mutation validation before write | `validate-innate-expression` called before `write-ghost-yaml` in add (line 168) and edit (line 197) | Confirmed | PASS |
| Executive authorization checked | `agent-is-executive-p` + `validate-executive-target` in `process-single-mutation` | Lines 157-163 confirmed | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|---------|
| CAP-01 | 28-01 | Ghost YAML has `responsibilities:` section with InnateScipt expressions | SATISFIED | 9 YAML files with `responsibilities:` — all 22 expressions validated |
| CAP-02 | 28-01, 28-02 | Tick engine evaluates ghost YAML capabilities instead of tool-registry.json | SATISFIED | 4 prompt builders in action-planner.lisp use YAML-first with tool-registry fallback |
| CAP-03 | 28-02 | Action planner injects ghost's InnateScipt capabilities into LLM cognition prompts | SATISFIED | All 4 builders confirmed: build-message-job, build-pipeline-task-job, build-task-job, build-proactive-job |
| CAP-04 | 28-03 | Ghost can write new InnateScipt expressions to its own responsibilities via cognition output | PARTIAL | Wired in execute-work-task. NOT wired in execute-proactive-work. Work-task is the primary task-execution path so most ghost work can trigger self-modification, but proactive-work path is missing. |
| CAP-05 | 28-03 | Ghost can edit/remove its own responsibility expressions via cognition output | PARTIAL | Same gap as CAP-04 — wired in work-task and project-review, not proactive-work |
| CAP-06 | 28-03 | Executive ghost can edit subordinate ghost responsibilities | SATISFIED | `validate-executive-target` implemented; `execute-project-review` (the executive cognition path) is wired for mutation extraction; `*capability-mutation-instructions*` injected in project-review prompts |
| CAP-07 | 28-01, 28-03 | Capability changes validated via InnateScipt parse-round-trip before persistence | SATISFIED | `validate-innate-expression` called in `process-single-mutation` before every `write-ghost-yaml` call; `validate-capability-list` available for bulk validation |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `config/agents/ethan_ng.yaml` | 2 | `![fundamentals.feeds]` vs plan-required `![fundamentals:feeds]` | Info | Documented deviation — InnateScipt tokenizer does not support colon-qualified search syntax. Dot notation is semantically equivalent and passes parse-round-trip. No functional regression. |
| `action-executor.lisp execute-proactive-work` | 986-1044 | No `extract-responsibility-mutations` block | Blocker | Proactive-work path cannot trigger ghost self-modification. Staff ghosts doing autonomous/proactive work cannot evolve their own capabilities via this path. |
| `action-planner.lisp build-proactive-job` | 752-821 | `*capability-mutation-instructions*` NOT appended to system-prompt | Blocker | Proactive-work prompts never instruct ghosts that they can output `responsibility_mutations`. Even if extraction were added to execute-proactive-work, ghosts wouldn't know to produce mutations. |

### Human Verification Required

#### 1. YAML Capability Source Confirmation

**Test:** Start noosphere-ghosts (`pm2 restart noosphere-ghosts`), wait for a tick, look at tick logs for a ghost with a YAML file (nova, eliana, etc.)
**Expected:** Log line `[planner-debug] nova: 3 capabilities from YAML (source: yaml)` confirming YAML path is taken, not tool-registry
**Why human:** Requires live tick cycle with LLM API active

#### 2. Ghost Self-Modification via Work-Task

**Test:** Send a task to nova or ethan_ng asking them to add a new capability: "Add a new capability expression `![test_capability]` to your responsibilities." Check `config/agents/nova.yaml` after the tick.
**Expected:** nova.yaml updated with the new responsibility expression after cognition processes the task
**Why human:** Requires live LLM cognition producing `responsibility_mutations` JSON in output

#### 3. Executive Subordinate Modification (CAP-06)

**Test:** Send a message to eliana asking her to add capability `![build_rust]` to a staff agent's YAML. Monitor tick log for `[cap-mutation] eliana add:` log line. Check the target staff agent's YAML file.
**Expected:** Staff agent's YAML file updated with new responsibility; log confirms executive authorization path taken
**Why human:** Requires live LLM output with `target_agent` field — executive CAP-06 path needs live testing

### Gaps Summary

One functional gap blocks full goal achievement:

**Proactive-work path missing mutation support (CAP-04/05 partial)**

The `execute-proactive-work` function in `action-executor.lisp` does not extract or process `responsibility_mutations` from LLM output. Similarly, `build-proactive-job` in `action-planner.lisp` does not include `*capability-mutation-instructions*` in the system prompt. This means ghosts in the proactive-work cognition path (staff doing autonomous departmental work) cannot self-modify their capabilities via this path.

The work-task path (`execute-work-task`) and the project-review path (`execute-project-review`) are both fully wired. The executive CAP-06 path is satisfied via project-review. CAP-04 and CAP-05 are satisfied for the work-task path (the primary staff task execution path) but incomplete for proactive-work.

**Root cause:** The plan's task 2 required wiring into both `execute-work-task` AND `execute-proactive-work`. The second wiring landed in `execute-project-review` instead — a plausible executive-path choice but leaving proactive-work uncovered.

**Severity:** Medium. The critical paths (task execution and executive review) are wired. Proactive-work self-modification is a less common path. The phase goal (ghosts declare capabilities via YAML, capabilities injected in prompts) is substantially achieved; only the self-modification feedback loop for the proactive-work path is incomplete.

**Fix required:** Add `extract-responsibility-mutations` block to `execute-proactive-work` (same 12-line pattern as lines 537-548 in execute-work-task) and append `*capability-mutation-instructions*` to the system-prompt string in `build-proactive-job` (same append pattern as work-task line 611).

---

_Verified: 2026-03-30T18:00:00Z_
_Verifier: Claude (gsd-verifier)_
