# Phase 28: Ghost Capabilities - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-30
**Phase:** 28-ghost-capabilities
**Areas discussed:** YAML file location/format, Responsibility expression syntax, Tick engine integration, Self-modification mechanism
**Mode:** Auto (--auto flag, all defaults selected)

---

## YAML File Location/Format

| Option | Description | Selected |
|--------|-------------|----------|
| /opt/project-noosphere-ghosts/config/agents/{id}.yaml | Colocated with existing config, one per ghost | ✓ |
| Database-only (no files) | Store in agents table JSONB column | |
| Persona files extended with YAML frontmatter | Augment existing persona .md files | |

**User's choice:** [auto] Config directory YAML files (recommended default)
**Notes:** Files are versionable, inspectable, and editable by both humans and ghosts.

---

## Responsibility Expression Syntax

| Option | Description | Selected |
|--------|-------------|----------|
| Valid InnateScipt expression strings | Each responsibility is a parseable Innate expression | ✓ |
| Free-text descriptions | Natural language capabilities | |
| Tool name references | Just tool names from registry | |

**User's choice:** [auto] InnateScipt expressions (recommended default)
**Notes:** Expressions are machine-parseable AND human-readable. Parse-round-trip ensures validity.

---

## Tick Engine Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Replace get-tools-for-agent with YAML capabilities | Action planner reads YAML instead of tool-registry.json | ✓ |
| Merge YAML + tool-registry | Both sources combined | |
| YAML overrides tool-registry | YAML takes precedence, registry as fallback | |

**User's choice:** [auto] Replace with YAML, tool-registry.json as fallback for ghosts without YAML (recommended default)
**Notes:** Gradual migration — full removal in Phase 31.

---

## Self-Modification Mechanism

| Option | Description | Selected |
|--------|-------------|----------|
| Cognition output mutations with parse-round-trip validation | Ghost outputs add/remove/edit, validated before YAML write | ✓ |
| API endpoint for YAML updates | HTTP-based modification | |
| Direct DB writes that sync to YAML | DB-first, file follows | |

**User's choice:** [auto] Cognition output mutations (recommended default)
**Notes:** Reuses Phase 25 innate-builder pattern. Same path for self-modification and executive modification.

---

## Claude's Discretion

- YAML parsing library choice
- Cognition output format for mutations
- Which ghosts get initial YAML files
- Error handling for missing YAML

## Deferred Ideas

- Full tool-registry.json retirement (Phase 31)
- YAML sections beyond responsibilities (future)
- DB frontmatter → YAML migration
- Pipeline definitions in YAML (Phase 30)
