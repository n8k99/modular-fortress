# Phase 31: Tool Migration - Discussion Log (Assumptions Mode)

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions captured in CONTEXT.md — this log preserves the analysis.

**Date:** 2026-03-30
**Phase:** 31-tool-migration
**Mode:** assumptions (auto)
**Areas analyzed:** Tool Inventory, Python Invocation, Expression Format, Metadata Storage, Registry Retirement, Result Flow

## Assumptions Presented

### Tool Inventory
| Assumption | Confidence | Evidence |
|------------|-----------|----------|
| All tools in tool-registry.json need InnateScipt wrappers | Confident | tool-registry.json has 30+ tools, ghost YAMLs have partial coverage |

### Python Invocation
| Assumption | Confidence | Evidence |
|------------|-----------|----------|
| Use uiop:run-program subprocess (existing pattern) | Likely | tool-socket.lisp lines 277-287 |

### Expression Format
| Assumption | Confidence | Evidence |
|------------|-----------|----------|
| Use existing ![tool_name] search syntax | Likely | 9 ghost YAML files already use this format |

### Tool Metadata Storage
| Assumption | Confidence | Evidence |
|------------|-----------|----------|
| Migrate to master_chronicle (DB is the OS) | Likely | Phase 30 area_content pattern, project constraint |

### Registry Retirement
| Assumption | Confidence | Evidence |
|------------|-----------|----------|
| Remove 4 fallback blocks + delete file | Confident | action-planner.lisp D-11 pattern at 4 locations |

### Result Flow
| Assumption | Confidence | Evidence |
|------------|-----------|----------|
| Existing cognition pipeline pattern unchanged | Confident | action-executor.lisp process-tool-calls |

## Corrections Made

No corrections — all assumptions auto-confirmed (--auto mode).

## Auto-Resolved

- Tool metadata storage: auto-selected DB storage (master_chronicle) based on Phase 30 precedent and "DB is the OS" constraint
