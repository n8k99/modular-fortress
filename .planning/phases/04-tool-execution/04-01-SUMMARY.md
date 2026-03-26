---
phase: 04-tool-execution
plan: 01
subsystem: tools
tags: [common-lisp, sbcl, tool-socket, claude-code-cli, scope-fix]

# Dependency graph
requires:
  - phase: 03-task-creation
    provides: "Ghost task creation and execution pipeline"
provides:
  - "Wildcard scope handling for tools with scope='*' (memory tools now visible)"
  - "claude_code tool registered for engineering/tools scope agents"
  - "claude-code-tool.sh shell wrapper for Claude Code CLI invocation"
affects: [04-tool-execution, ghost-cognition, memory-tools]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Lisp :wildcard keyword for scope='*' short-circuit"]

key-files:
  created:
    - "/opt/project-noosphere-ghosts/tools/claude-code-tool.sh"
  modified:
    - "/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp"
    - "/opt/project-noosphere-ghosts/config/tool-registry.json"

key-decisions:
  - "Used :wildcard keyword symbol for scope='*' detection to avoid string comparison in intersection"
  - "claude_code tool placed after build_tool in registry for logical grouping"

patterns-established:
  - "Wildcard scope pattern: stringp check -> :wildcard keyword -> short-circuit intersection"

requirements-completed: [TOOL-01, TOOL-05]

# Metrics
duration: 2min
completed: 2026-03-26
---

# Phase 04 Plan 01: Tool Scope Fix + Claude Code Registration Summary

**Fixed wildcard scope bug making memory tools invisible, registered claude_code tool with bash wrapper for engineering ghosts**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-26T06:47:21Z
- **Completed:** 2026-03-26T06:49:47Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Fixed critical bug where scope="*" string fell through to empty list, making read_own_memory and write_own_memory invisible to all agents
- Registered claude_code tool in tool-registry.json with engineering+tools scope and /bin/bash interpreter
- Created claude-code-tool.sh with SBCL environment cleanup, 120s timeout, and JSON output format

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix wildcard scope bug and register claude_code tool** - `4c06ce8` (fix)
2. **Task 2: Create claude-code-tool.sh shell script** - `79dea6d` (feat)

## Files Created/Modified
- `/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp` - Added :wildcard handling in get-tools-for-agent scope cond clause
- `/opt/project-noosphere-ghosts/config/tool-registry.json` - Added claude_code tool entry with engineering+tools scope
- `/opt/project-noosphere-ghosts/tools/claude-code-tool.sh` - New synchronous Claude Code CLI wrapper script

## Decisions Made
- Used :wildcard keyword symbol instead of storing "*" string to make the short-circuit check clean (eq vs string-equal)
- Placed claude_code entry after build_tool in registry for logical grouping of code-execution tools

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Memory tools (read_own_memory, write_own_memory) now accessible to all agents with non-empty scope
- claude_code tool registered and ready for engineering/tools scope agents
- Plan 04-02 can build on this foundation for expanded tool dispatch

---
*Phase: 04-tool-execution*
*Completed: 2026-03-26*
