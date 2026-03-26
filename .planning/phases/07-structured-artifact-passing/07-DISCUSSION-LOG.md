# Phase 7: Structured Artifact Passing - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 07-structured-artifact-passing
**Areas discussed:** Schema design per stage, Storage location, Validation strictness, Context handoff format

---

## Schema Design Per Stage

| Option | Description | Selected |
|--------|-------------|----------|
| Universal base + stage overrides | Shared base schema with optional stage-specific extensions. Manageable while allowing specialization. | ✓ |
| Per-pipeline-per-stage schemas | Each of 20+ stages gets own JSON schema. Maximum precision, high maintenance. | |
| Per-pipeline-type schemas only | One schema per pipeline type. Stages within pipeline use same structure. | |
| You decide | Claude picks based on existing validation patterns. | |

**User's choice:** Universal base + stage overrides (Recommended)

### Follow-up: Base schema fields

| Option | Description | Selected |
|--------|-------------|----------|
| Summary + outputs + issues | Base: summary, key_outputs array, issues array, metadata object. | ✓ |
| Minimal — summary only | Base: summary, metadata only. Stage-specific fields carry all detail. | |
| You decide | Claude designs based on what perception/reporting consume. | |

**User's choice:** Summary + outputs + issues (Recommended)

---

## Storage Location

| Option | Description | Selected |
|--------|-------------|----------|
| stage_notes as JSONB | Migrate from TEXT to JSONB. Structured output in DB. Remove disk file dependency. | ✓ |
| New artifacts table | Separate table: task_artifacts. Clean separation, adds join complexity. | |
| Keep both, structure DB only | Migrate stage_notes to JSONB, keep disk files for backward compat. | |
| You decide | Claude picks based on DB-is-the-OS constraint. | |

**User's choice:** stage_notes as JSONB (Recommended)

### Follow-up: Migration of existing data

| Option | Description | Selected |
|--------|-------------|----------|
| Wrap existing text in JSON | ALTER to JSONB, UPDATE to {"legacy_text": "...", "schema_version": 0}. No data loss. | ✓ |
| Null out existing values | ALTER to JSONB, SET to NULL. Clean slate. | |
| You decide | Claude checks if existing data matters. | |

**User's choice:** Wrap existing text in JSON (Recommended)
**Notes:** User added: "final results of pipeline must live in specific areas in the DB" — meaning completed pipeline deliverables go to appropriate tables (documents, vault_notes, etc.), not just stage_notes.

---

## Validation Strictness

| Option | Description | Selected |
|--------|-------------|----------|
| Required base + optional extensions | Base fields MUST be present. Extensions validated if present but not required. | ✓ |
| Strict full schema validation | Every defined field must be present and match types. Risk of rejecting useful output. | |
| Soft validation with warnings | Parse as JSON, check base fields. Warn on missing extensions, don't reject. | |
| You decide | Claude picks based on existing rejection/retry pattern. | |

**User's choice:** Required base + optional extensions (Recommended)

---

## Context Handoff Format

| Option | Description | Selected |
|--------|-------------|----------|
| Load from DB via stage_notes JSONB | action-planner reads predecessor's JSONB. Replaces disk-file loading. DB-is-the-OS. | ✓ |
| Include in perception context | Perception endpoint returns predecessor output in task context. | |
| Post as conversation message | Post structured output as conversation to next assignee. | |
| You decide | Claude picks based on action-planner patterns. | |

**User's choice:** Load from DB via stage_notes JSONB (Recommended)

### Follow-up: Hard prompts location

| Option | Description | Selected |
|--------|-------------|----------|
| Keep hard prompts separate | Instructions stay in documents table. Stage schema carries output data only. | ✓ |
| Embed in stage schema | Stage schema includes instructions field from documents. Self-contained but duplicates. | |
| You decide | Claude picks based on prompt builder patterns. | |

**User's choice:** Keep hard prompts separate

---

## Claude's Discretion

- Schema version field inclusion
- Stage-specific extension JSON structures
- Validation placement (advance-pipeline vs validate-stage-output)
- Legacy/structured transition handling

## Deferred Ideas

None — discussion stayed within phase scope
