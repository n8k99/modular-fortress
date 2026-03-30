---
phase: 24-template-evaluation-execution
verified: 2026-03-29T20:15:00Z
status: gaps_found
score: 3/5 success criteria verified
gaps:
  - truth: "(sarah_lin){sync_calendar} in a Daily Note template triggers the calendar sync tool invocation during ghost operations, producing real output"
    status: partial
    reason: "Commission conversations ARE created with channel=commission targeting sarah/kathryn, but find-unanswered-message in build-message-job filters them out because from_agent='system' is not in message-from-human-p and the message body 'sync_calendar' does not match handoff keywords. The commission never becomes a cognition job. Only half the pipeline is wired."
    artifacts:
      - path: "/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp"
        issue: "find-unanswered-message at line 227 skips messages from 'system' that are not handoffs. Commission messages inserted by deliver-commission (from_agent='system') are dropped here and never produce a cognition job."
    missing:
      - "find-unanswered-message must recognize channel=commission messages as actionable (check gethash :channel msg for 'commission')"
      - "OR build-commission-job function that handles commission perception as a distinct job type before build-message-job in the priority chain"
  - truth: "(kathryn){finance_positions} triggers the trading positions tool, with output attributed to Kathryn in the conversations table"
    status: partial
    reason: "Same root cause as SC3: commission conversation for kathryn IS created in DB (verified at id=9650), but find-unanswered-message filters it from cognition job creation. No tool invocation occurs."
    artifacts:
      - path: "/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp"
        issue: "Same filter as SC3 — from='system' and message='finance_positions' does not match any bypass condition in find-unanswered-message."
    missing:
      - "Same fix as SC3 — commission channel bypass in find-unanswered-message or dedicated commission job builder"
human_verification:
  - test: "After fixing find-unanswered-message commission bypass: restart noosphere-ghosts, observe whether sarah receives a cognition job from the sync_calendar commission and whether a tool_call block appears in the LLM output"
    expected: "Sarah's next tick produces a cognition job with commission content, action-executor invokes the sync_calendar tool, result posted to conversations"
    why_human: "Requires live tick execution with LLM call — cannot verify programmatically without running the tick engine"
  - test: "Observe Kathryn's tick following a finance_positions commission insertion"
    expected: "Kathryn's cognition response contains a tool_call for finance_positions, action-executor invokes it, output attributed to Kathryn in conversations"
    why_human: "Same reason — live tick required with LLM call"
---

# Phase 24: Template Evaluation & Execution — Verification Report

**Phase Goal:** Ghosts read .dpn Templates from the noosphere, evaluate their Innate expressions, and Daily Note (agent){action} patterns trigger real tool invocations during operations
**Verified:** 2026-03-29T20:15:00Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths (from ROADMAP.md Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| SC1 | A ghost's cognition job includes evaluated Template content — Innate expressions in the Template body are resolved to concrete values before the LLM prompt is built | VERIFIED | `template-context` binding at line 952 in action-planner.lisp; format string at line 985 includes it; `## Template Context` header injected when non-empty |
| SC2 | Template evaluation results inform ghost planning: an executive reading a Template with `@projects{status=blocked}` sees actual blocked projects, not the raw expression | VERIFIED | `evaluate-template-for-project` (line 861) calls `load-bundle` → wraps in `:program` node → `evaluate` with `*noosphere-resolver*`; resolver's `resolve-search` (noosphere-resolver.lisp line 220) queries master_chronicle. Template body uses `![projects]{status=blocked}` (correct Innate search syntax, documented in 24-02-SUMMARY) |
| SC3 | `(sarah_lin){sync_calendar}` in a Daily Note template triggers the calendar sync tool invocation during ghost operations, producing real output | PARTIAL | Commission conversation created (DB id=9649, to_agent={sarah}, channel=commission, message=sync_calendar). BUT `find-unanswered-message` (line 227) filters it — from='system' is not human, message contains no HANDOFF/DELEGATE/ESCALATE keywords. Commission never becomes a cognition job. Tool invocation does NOT occur. |
| SC4 | `(kathryn){finance_positions}` triggers the trading positions tool, with output attributed to Kathryn in the conversations table | PARTIAL | Commission conversation created (DB id=9650, to_agent={kathryn}, channel=commission, message=finance_positions). Same filter issue as SC3. No tool invocation. |
| SC5 | Evaluation errors in a Template do not crash the tick — the ghost receives an error context and can skip or report the failure | VERIFIED | Double handler-case: `evaluate-template-for-project` catches `innate-resistance` (inline marker) and `error` (fallback string); outer `template-context` binding also has its own `handler-case`. SBCL compilation succeeds with "COMPILE OK". |

**Score: 3/5 success criteria verified**

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `/opt/project-noosphere-ghosts/launch.sh` | evaluator.lisp loaded at startup | VERIFIED | Line 9 contains `/opt/innatescript/src/eval/evaluator` in dolist load sequence |
| `/opt/project-noosphere-ghosts/lisp/packages.lisp` | Innate imports in action-planner package | VERIFIED | Lines 240-244: `innate.eval #:evaluate`, `innate.eval.resolver #:make-eval-env #:load-bundle`, `innate.types #:make-node #:node-children`, `innate.conditions #:innate-resistance`, `noosphere-resolver #:*noosphere-resolver*`. Package ordering fixed (noosphere-resolver before action-planner at line 206). |
| `/opt/project-noosphere-ghosts/lisp/runtime/action-planner.lisp` | `evaluate-template-for-project` + `format-innate-value` + template-context injection | VERIFIED | `format-innate-value` at line 845, `evaluate-template-for-project` at line 861, `template-context` binding at line 952, injected into format string at line 985. 2 occurrences of `evaluate-template-for-project` (defun + call site). |
| `master_chronicle.templates` | Test template with commission patterns | VERIFIED | Row id=4, name="Operation Normality", category="standing-order", body contains `(sarah){sync_calendar}`, `(kathryn){finance_positions}`, `![projects]{status=blocked}` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| `action-planner.lisp` | `innate.eval:evaluate` | `evaluate-template-for-project` helper | WIRED | `(evaluate program env)` at line 877; `innate.eval:evaluate` imported at packages.lisp line 240 |
| `action-planner.lisp` | `noosphere-resolver:*noosphere-resolver*` | `load-bundle *noosphere-resolver*` call | WIRED | `(load-bundle *noosphere-resolver* project-name)` at line 872; symbol imported at packages.lisp line 244 |
| `action-planner.lisp` | `deliver-commission on noosphere-resolver` | Innate evaluator commission adjacency during evaluate call | WIRED (indirect) | `deliver-commission` fires as side effect of `evaluate` when parser encounters `(agent){action}` pattern; 2 commission rows created in DB (ids 9649, 9650) |
| `conversations` (channel=commission) | target agent perception | `af64_perception.rs` query `$1 = ANY(to_agent)` | WIRED | Perception API at line 36 returns messages where agent is in `to_agent` array regardless of channel |
| commission perception message | cognition job | `find-unanswered-message` → `build-message-job` | NOT_WIRED | `find-unanswered-message` skips messages from 'system' that are not handoffs. Commission messages are filtered out before cognition job creation. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `action-planner.lisp build-project-review-job` | `template-context` | `evaluate-template-for-project` → `load-bundle` → templates table | Yes (when template name matches project name) | FLOWING for template loading and @reference resolution |
| `conversations` table (commission rows) | commission messages | `deliver-commission` → `db-insert-conversation` | Yes — 2 real rows with channel=commission | FLOWING to DB |
| target agent commission perception | cognition job trigger | `find-unanswered-message` | No — commission messages filtered out | DISCONNECTED at cognition job creation |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| evaluator.lisp in launch.sh load sequence | `grep "evaluator" /opt/project-noosphere-ghosts/launch.sh` | `/opt/innatescript/src/eval/evaluator` present | PASS |
| Innate imports in action-planner package | `grep "innate.eval" /opt/project-noosphere-ghosts/lisp/packages.lisp` | 4 import-from clauses found | PASS |
| evaluate-template-for-project defined and called | `grep -c "evaluate-template-for-project" action-planner.lisp` | 2 (defun + call site) | PASS |
| template-context in cognition job format string | `grep "template-context" action-planner.lisp` | 3 matches: binding, error fallback, format string | PASS |
| SBCL full compilation | launch sequence with `COMPILE OK` | "COMPILE OK" printed, exit 0 (2 style-warnings, not errors) | PASS |
| Commission conversations in DB | `SELECT COUNT(*) FROM conversations WHERE channel='commission'` | 2 rows: id=9649 (sarah/sync_calendar), id=9650 (kathryn/finance_positions) | PASS |
| Commission messages NOT becoming cognition jobs | Review `find-unanswered-message` logic | from='system' + no handoff keywords → filtered | FAIL (SC3, SC4 gap) |
| Test template in DB | `SELECT body FROM templates WHERE category='standing-order'` | body has (sarah){sync_calendar}, (kathryn){finance_positions}, ![projects]{status=blocked} | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| INNATE-02 | 24-01-PLAN.md | Ghosts evaluate .dpn Template bodies during cognition, receiving resolved content as actionable context | SATISFIED | `evaluate-template-for-project` wired into `build-project-review-job`; template content injected as `## Template Context` section; SC1 and SC2 verified |
| INNATE-04 | 24-02-PLAN.md | Daily Note (agent){action} patterns trigger real tool invocations during ghost operations | PARTIAL | Commission conversations created (SC1 of pipeline) and perceived by target agents (SC2 of pipeline), but `find-unanswered-message` filters prevent cognition job creation (SC3 of pipeline). Tool invocations do NOT occur. |

No orphaned requirements: both INNATE-02 and INNATE-04 appear in plan frontmatter. INNATE-03 is Phase 25 (pending). INNATE-01 is Phase 23 (complete). All Phase 24 requirement IDs accounted for.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `action-planner.lisp` | 242-245 | `find-unanswered-message` silently drops commission messages; no TODO or explicit documentation that commission bypass is needed | Warning | Commission pipeline is silently broken — messages created, perceived, but not actioned |

No placeholder/stub patterns found. No hardcoded empty returns that affect dynamic data. SBCL compilation produces style-warnings (BROKER-PENDING-AGENTS, BROKER-TICK-SUMMARY undefined at load time) — these are forward references in Common Lisp and do not indicate runtime failures.

### Human Verification Required

#### 1. Post-fix commission cognition test (sarah)

**Test:** After adding commission channel bypass to `find-unanswered-message`, insert a fresh commission row for sarah (`INSERT INTO conversations (from_agent, to_agent, message, channel) VALUES ('system', '{sarah}', 'sync_calendar', 'commission')`), restart noosphere-ghosts, wait for one tick, check tick logs.
**Expected:** Sarah's tick log shows a cognition job created, LLM response contains a `tool_call` block for sync_calendar, action-executor logs show tool execution.
**Why human:** Requires live tick engine + LLM call. Cannot verify programmatically without running the service.

#### 2. Post-fix commission cognition test (kathryn)

**Test:** Same as above but for kathryn / finance_positions commission.
**Expected:** Kathryn's tick produces a tool_call for finance_positions, result attributed to Kathryn in conversations.
**Why human:** Same reason.

### Gaps Summary

Two success criteria (SC3, SC4) fail for the same root cause: the commission→cognition link is broken.

**What works:** Template evaluation is fully wired (SC1, SC2). Errors are handled gracefully (SC5). Commission conversations ARE created in the DB with correct channel, from_agent, and to_agent values. The perception API returns them to target agents.

**What is missing:** `find-unanswered-message` (action-planner.lisp line 227) needs a bypass for commission channel messages. The fix is small — add a channel check before the human/handoff filter:

```lisp
;; Allow commission messages from system to proceed
(let ((channel (gethash :channel msg)))
  (when (string-equal channel "commission")
    (return msg)))
```

OR restructure the filter to allow any message with `channel="commission"` regardless of sender.

Both SC3 and SC4 fail from this single missing bypass. Once the bypass is added, commission messages will reach `build-message-job` → create a cognition job → the LLM will respond to the commission instruction → action-executor will invoke the matching tool.

---

_Verified: 2026-03-29T20:15:00Z_
_Verifier: Claude (gsd-verifier)_
