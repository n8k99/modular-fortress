# Phase 15: Financial Pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-28
**Phase:** 15-financial-pipeline
**Areas discussed:** Trading sessions, Calendar sync, Tool scope, Output format
**Mode:** Auto (all areas auto-selected with recommended defaults)

---

## Trading Session Tools

| Option | Description | Selected |
|--------|-------------|----------|
| Run existing trading_briefing.py per session | Ghost invokes with --session flag | ✓ |
| Rewrite as ghost-native market scanner | Build trading logic into Lisp | |
| Single combined briefing | One tool covers all sessions | |

**User's choice:** [auto] Run existing script per session (Phase 13/14 pattern)

---

## Calendar Sync

| Option | Description | Selected |
|--------|-------------|----------|
| Ghost runs wave_calendar.py on schedule | Add as 4th schedule entry on Project #10 | ✓ |
| Keep in OpenClaw | Calendar sync stays separate | |
| Merge into daily health check | Nova handles calendar | |

**User's choice:** [auto] Ghost runs on schedule under Kathryn's project (OPS-05 requirement)

---

## Tool Scope

| Option | Description | Selected |
|--------|-------------|----------|
| trading scope | Matches Kathryn's primary domain | ✓ |
| scheduling scope | Calendar sync is scheduling | |
| Mixed scopes | Trading for briefings, scheduling for calendar | |

**User's choice:** [auto] trading scope for all (Kathryn has it, simplest)

---

## Output Format

| Option | Description | Selected |
|--------|-------------|----------|
| Conversations table (noosphere-native) | Kathryn posts to conversations | ✓ |
| Discord (--discord flag) | Same as OpenClaw | |

**User's choice:** [auto] Conversations (consistent with Phase 13/14)

## Claude's Discretion

- Single vs multiple tool registrations per session
- Error handling for external API failures
- Calendar sync scope choice

## Deferred Ideas

- Real-time market alerts
- Kalshi position management
- Multi-source calendar aggregation
