# Phase 20: Nexus Import & Temporal Compression - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-29
**Phase:** 20-nexus-import-temporal-compression
**Areas discussed:** Compression approach, Nova memory injection, Daily/weekly note linking

---

## Compression Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic then LLM | Group by month, extract titles+dates+topic keywords deterministically, then one LLM call per month | |
| Fully deterministic | No LLM — structured lists only | |
| LLM per conversation | Each conversation gets an LLM summary first, then grouped and re-summarized per month | ✓ |

**User's choice:** LLM per conversation
**Notes:** User wants thorough per-conversation summaries as the foundation.

### Cascade Direction

| Option | Description | Selected |
|--------|-------------|----------|
| Cascade up | Quarterly = synthesis of monthly summaries. Yearly = synthesis of quarterly. | ✓ |
| Re-process originals | Go back to per-conversation summaries for each tier | |

**User's choice:** Cascade up
**Notes:** User clarified: "Once the weekly is summarized, the monthly, quarterly and yearly need to be summarized" — confirming the cascade chain where each tier builds on the one below.

### Content Filtering

| Option | Description | Selected |
|--------|-------------|----------|
| Summarize all | Every conversation regardless of length | |
| Filter by content size | Skip under-threshold conversations, mark as 'trivial' | ✓ |
| You decide | Claude picks threshold | |

**User's choice:** Filter by content size

---

## Nova Memory Injection

### Memory Entry Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| One entry per tier | Separate memory rows per tier per ghost | ✓ |
| Single synthesized entry | One master row per ghost | |
| Per-month only | Monthly memories only | |

**User's choice:** One entry per tier

### Synthesis Perspective

| Option | Description | Selected |
|--------|-------------|----------|
| Operational lens | What Nathan worked on, tools discussed, decisions made | |
| Full narrative | Rich retelling capturing tone, evolution, personal context | ✓ |
| You decide | Claude picks based on role | |

**User's choice:** Full narrative

### Ghost Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Nova only | Only Nova/T.A.S.K.S. | |
| Topic-routed | Route to relevant executives by topic | ✓ |

**User's choice:** Topic-routed — expanded to 4 executives: Nova, LRM, Vincent, Sylvia

---

## Daily/Weekly Note Linking

### Link Format

| Option | Description | Selected |
|--------|-------------|----------|
| Wikilink section append | Append `## Nexus Imports` section with [[wikilinks]] | ✓ |
| Inline markdown links | Insert links into note body | |
| Metadata only | Junction table, no content changes | |

**User's choice:** Wikilink section append

### Missing Notes

| Option | Description | Selected |
|--------|-------------|----------|
| Skip silently | Only link to existing notes | |
| Create stub notes | Create minimal daily notes | |
| You decide | Claude picks based on existing note coverage | |

**User's choice:** Other — "use the @Templates/Daily Note to generate one" — generate full daily notes from the existing Daily Note template in the documents table.

---

## Claude's Discretion

- Content size threshold for trivial filtering
- LLM prompt design for summaries and synthesis
- Domain classification approach
- Batch sizing for LLM calls
- Direct SQL vs API for import
- Operation ordering

## Deferred Ideas

None
