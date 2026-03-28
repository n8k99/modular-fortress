# Phase 13: Operations Pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 13-operations-pipeline
**Areas discussed:** Ghost tool execution, Output destinations, Temporal cascade, Podcast watcher migration
**Mode:** Auto (all areas auto-selected with recommended defaults)

---

## Ghost Tool Execution

| Option | Description | Selected |
|--------|-------------|----------|
| Direct tool invocation via Claude CLI | Nova runs existing Python scripts as tools | ✓ |
| Rewrite scripts as Lisp functions | Embed operational logic in tick engine | |
| Create new ghost-native tool wrappers | Intermediate abstraction layer | |

**User's choice:** [auto] Direct tool invocation (recommended default)
**Notes:** Existing scripts are proven. No benefit to rewriting in another language.

---

## Output Destinations

| Option | Description | Selected |
|--------|-------------|----------|
| Conversations table (noosphere-native) | Output as Nova's conversation messages | ✓ |
| Discord channels (existing target) | Post directly to Discord like OpenClaw does | |
| Both (dual output) | Post to conversations AND Discord | |

**User's choice:** [auto] Conversations table (recommended default)
**Notes:** Noosphere is the substrate. Discord is an external concern for a future bridge.

---

## Temporal Cascade

| Option | Description | Selected |
|--------|-------------|----------|
| Independent standing order schedules | Each temporal level fires on its own schedule | ✓ |
| Task dependency chains | Daily creates weekly task, weekly creates monthly | |
| Single schedule with cascading logic | One nightly job handles all temporal levels | |

**User's choice:** [auto] Independent standing order schedules (recommended default)
**Notes:** Schedules already seeded on Project #14. Python scripts already handle reading prior-level output from vault_notes.

---

## Podcast Watcher Migration

| Option | Description | Selected |
|--------|-------------|----------|
| Nova executes existing podcast_watcher.py | Ghost runs the Python script on schedule | ✓ |
| Rewrite as ghost-native RSS checker | Build RSS feed checking into ghost tools | |
| Keep in OpenClaw until Discord bot migrates | Defer until Project #5 Discord Sovereignty | |

**User's choice:** [auto] Nova executes existing script (recommended default)
**Notes:** Script works, posts to Discord via webhook. Nova just needs to run it and report.

---

## Claude's Discretion

- Exact prompt wording for schedule-label-to-tool mapping
- Batching of simultaneous schedule fires
- Error handling for script failures

## Deferred Ideas

- Discord output bridge for ghost conversations
- Tasks archive migration (6h cycle)
- Pipeline wakeup + conversations poll (OpenClaw internal)
