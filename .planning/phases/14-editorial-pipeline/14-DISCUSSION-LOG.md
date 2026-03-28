# Phase 14: Editorial Pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 14-editorial-pipeline
**Areas discussed:** Editorial trigger, Content pipeline, Output destination, Ownership
**Mode:** Auto (all areas auto-selected with recommended defaults)

---

## Editorial Trigger Mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| Standing order on Project #12 | Schedule already seeded (0 1 * * * UTC) | ✓ |
| Manual dispatch only | Nathan triggers when ready | |
| Conversation watcher | Fire when Nathan stops commenting | |

**User's choice:** [auto] Standing order (pre-decided in Phase 12)
**Notes:** Schedule already exists on Project #12.

---

## Content Pipeline Steps

| Option | Description | Selected |
|--------|-------------|----------|
| Run existing nightly_editorial.py | Ghost executes proven Python script | ✓ |
| Rewrite as ghost-native pipeline | Build editorial logic into Lisp | |
| Decompose into separate tool calls | Each step (query, fetch, synthesize) as separate tool | |

**User's choice:** [auto] Run existing script (Phase 13 pattern)
**Notes:** Script already handles full pipeline end-to-end.

---

## Output Destination

| Option | Description | Selected |
|--------|-------------|----------|
| Documents table + conversations | Script saves to documents, Sylvia reports | ✓ |
| Discord channel | Post directly to Discord | |
| Both | Dual output | |

**User's choice:** [auto] Documents + conversations (noosphere-native)

---

## Sylvia vs Nova Ownership

| Option | Description | Selected |
|--------|-------------|----------|
| Sylvia owns and executes | She owns Project #12, runs the editorial | ✓ |
| Nova orchestrates, Sylvia reviews | Nova triggers, Sylvia approves | |
| Shared execution | Both involved | |

**User's choice:** [auto] Sylvia owns and executes (matches DB ownership)

---

## Claude's Discretion

- Generalizing label-to-tool mapping for multiple executives
- Additional editorial tool registration
- Error handling for API failures

## Deferred Ideas

- Real-time editorial trigger (conversation watcher)
- Multi-author support
- Editorial quality scoring
