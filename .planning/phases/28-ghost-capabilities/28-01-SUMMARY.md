---
phase: 28-ghost-capabilities
plan: 01
subsystem: runtime
tags: [yaml, innate, ghost-capabilities, common-lisp, sbcl]

# Dependency graph
requires:
  - phase: 25-innate-generation
    provides: validate-innate-expression for parse-round-trip validation
provides:
  - yaml.lisp minimal YAML parser (parse-simple-yaml, serialize-simple-yaml)
  - ghost-capabilities.lisp module (load, format, validate, write, mutate)
  - 9 agent YAML files with InnateScipt responsibility expressions
  - af64.utils.yaml and af64.runtime.ghost-capabilities packages
affects: [28-02-action-planner-integration, 28-03-self-modification, 29-executive-oversight, 31-tool-migration]

# Tech tracking
tech-stack:
  added: [custom YAML parser (lisp/util/yaml.lisp)]
  patterns: [per-ghost YAML config at config/agents/{id}.yaml, handler-case eval for conditional package loading]

key-files:
  created:
    - /opt/project-noosphere-ghosts/lisp/util/yaml.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/ghost-capabilities.lisp
    - /opt/project-noosphere-ghosts/config/agents/nova.yaml
    - /opt/project-noosphere-ghosts/config/agents/eliana.yaml
    - /opt/project-noosphere-ghosts/config/agents/kathryn.yaml
    - /opt/project-noosphere-ghosts/config/agents/sylvia.yaml
    - /opt/project-noosphere-ghosts/config/agents/vincent.yaml
    - /opt/project-noosphere-ghosts/config/agents/jmax.yaml
    - /opt/project-noosphere-ghosts/config/agents/lrm.yaml
    - /opt/project-noosphere-ghosts/config/agents/sarah.yaml
    - /opt/project-noosphere-ghosts/config/agents/ethan_ng.yaml
  modified:
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/launch.sh

key-decisions:
  - "Used dots instead of hyphens/colons in InnateScipt expressions for valid parser syntax"
  - "Custom ~80-line YAML parser matching project zero-deps convention"
  - "handler-case eval pattern for ghost-capabilities package (depends on innate-builder)"

patterns-established:
  - "Per-ghost YAML at config/agents/{id}.yaml with id: and responsibilities: sections"
  - "All InnateScipt expressions double-quoted in YAML to escape special characters"

requirements-completed: [CAP-01, CAP-02, CAP-07]

# Metrics
duration: 4min
completed: 2026-03-30
---

# Phase 28 Plan 01: Ghost Capabilities Foundation Summary

**Minimal YAML parser and ghost-capabilities module with 9 agent YAML files declaring InnateScipt responsibilities, all validated via parse-round-trip**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-30T17:26:30Z
- **Completed:** 2026-03-30T17:30:23Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments
- Created yaml.lisp: custom minimal YAML parser for agent config files (parse-simple-yaml, serialize-simple-yaml, unquote-yaml-string)
- Created ghost-capabilities.lisp: load-ghost-capabilities, format-capabilities-for-prompt, validate-capability-list, write-ghost-yaml, process-responsibility-mutations
- Created 9 agent YAML files with 22 total InnateScipt responsibility expressions, all passing parse-round-trip validation
- Updated packages.lisp with af64.utils.yaml and af64.runtime.ghost-capabilities package definitions
- Updated launch.sh load order for yaml and ghost-capabilities modules

## Task Commits

Each task was committed atomically:

1. **Task 1: Create YAML parser and ghost-capabilities module with package definitions** - `f57bfd4` (feat)
2. **Task 2: Create 9 initial agent YAML files with validated InnateScipt responsibilities** - `fb326dd` (feat)

## Files Created/Modified
- `lisp/util/yaml.lisp` - Minimal YAML parser: split-lines, unquote-yaml-string, parse-simple-yaml, serialize-simple-yaml
- `lisp/runtime/ghost-capabilities.lisp` - Ghost capability loading, formatting, validation, writing, mutation processing
- `lisp/packages.lisp` - Added af64.utils.yaml and af64.runtime.ghost-capabilities package definitions
- `launch.sh` - Added util/yaml and runtime/ghost-capabilities to load order
- `config/agents/*.yaml` - 9 agent YAML files (nova, eliana, kathryn, sylvia, vincent, jmax, lrm, sarah, ethan_ng)

## Decisions Made
- Used dots instead of hyphens in bundle names (e.g., `{em.content.thought.police}` not `{em.content.thought-police}`) because InnateScipt parser treats hyphens as PROSE tokens inside bundles
- Used dots instead of colons in search expressions (e.g., `![fundamentals.feeds]` not `![fundamentals:feeds]`) because InnateScipt parser doesn't support colon-qualified search syntax
- Simplified `![technicals:oanda_api[pairs: major+minor]]` to `![technicals.oanda_api]` because bracket qualifier syntax within search expressions is not supported by the tokenizer
- Custom YAML parser (~80 lines) follows project zero-deps convention (no cl-yaml/Quicklisp)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed InnateScipt expression syntax for 3 planned expressions**
- **Found during:** Task 2 (agent YAML creation)
- **Issue:** Plan specified `{em.content.thought-police}`, `![fundamentals:feeds]`, and `![technicals:oanda_api[pairs: major+minor]]` which all fail InnateScipt parse-round-trip validation
- **Fix:** Used dot notation for all: `{em.content.thought.police}`, `![fundamentals.feeds]`, `![technicals.oanda_api]`
- **Files modified:** config/agents/sylvia.yaml, config/agents/ethan_ng.yaml
- **Verification:** All 22 expressions pass InnateScipt parse-round-trip via SBCL
- **Committed in:** fb326dd (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Necessary correction — plan specified expressions that don't parse. Dot notation preserves the same semantic intent.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- YAML parser and ghost-capabilities module ready for integration into action-planner (Plan 02)
- All 9 agent YAML files are loaded and validated
- format-capabilities-for-prompt produces prompt sections ready for LLM injection
- tool-registry.json remains as fallback per D-11

---
*Phase: 28-ghost-capabilities*
*Completed: 2026-03-30*
