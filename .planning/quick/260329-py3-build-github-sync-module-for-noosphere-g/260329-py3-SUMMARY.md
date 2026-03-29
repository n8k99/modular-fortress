---
phase: quick
plan: 260329-py3
subsystem: noosphere-ghosts
tags: [github, sync, lisp, af64]
dependency_graph:
  requires: [af64.utils.http, af64.runtime.db-tasks, af64.runtime.db-auxiliary]
  provides: [af64.utils.github]
  affects: [task-sync, github-integration]
tech_stack:
  added: []
  patterns: [github-rest-api, bidirectional-sync, dedup-task-id]
key_files:
  created:
    - /opt/project-noosphere-ghosts/lisp/util/github.lisp
  modified:
    - /opt/project-noosphere-ghosts/lisp/packages.lisp
    - /opt/project-noosphere-ghosts/lisp/af64.asd
    - /opt/project-noosphere-ghosts/lisp/runtime/db-auxiliary.lisp
    - /opt/project-noosphere-ghosts/lisp/runtime/db-tasks.lisp
decisions:
  - Package af64.utils.github placed after db-tasks in packages.lisp load order (depends on runtime packages)
  - util/github registered as standalone file after runtime module in af64.asd (not in util module)
  - Added parse-task-id helper for extracting repo/number from task-id strings
metrics:
  duration: 4min
  completed: 2026-03-29
  tasks_completed: 2
  tasks_total: 2
  files_created: 1
  files_modified: 4
---

# Quick Task 260329-py3: Build GitHub Sync Module Summary

GitHub REST API client for AF64 runtime with bidirectional issue sync via curl/libpq (zero external deps).

## One-liner

Bidirectional GitHub Issues sync module using curl-based HTTP and direct PostgreSQL, with dedup-safe task IDs and independent error isolation per sync phase.

## Changes Made

### Task 1: DB helpers and package declaration (b7c505c)

- Added `db-get-github-projects` to db-auxiliary.lisp -- queries projects with github_url set and active status
- Added `db-find-task-by-task-id` to db-tasks.lisp -- lookup by string task_id (not integer id)
- Added `db-create-github-task` to db-tasks.lisp -- inserts with source='github', ON CONFLICT DO NOTHING for dedup
- Declared `af64.utils.github` package in packages.lisp after db-tasks (dependency order)
- Registered `(:file "util/github")` in af64.asd after runtime module, before main

### Task 2: Full sync implementation (9881944)

Created `/opt/project-noosphere-ghosts/lisp/util/github.lisp` (323 lines) with:

- `*github-token*` read from GITHUB_TOKEN env var at load time
- `github-headers` returns auth + accept + user-agent + api-version headers
- `github-api-get/post/patch` wrappers with JSON parsing and error handling
- `parse-github-url` extracts owner/repo from GitHub URLs (handles .git, trailing slashes)
- `github-task-id` builds dedup-safe task IDs: `gh-{repo}-{number}`
- `parse-task-id` reverse-parses task-id to extract repo and issue number
- `sync-github-issues` fetches issues from all GitHub-linked projects, creates/updates tasks
- `push-tasks-to-github` creates GitHub issues for noosphere-origin open tasks
- `sync-task-status` bidirectionally syncs open/closed state
- `sync-all` entry point with token check and independent handler-case per sub-call

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Package load order in packages.lisp**
- **Found during:** Task 1
- **Issue:** Plan said to place af64.utils.github after af64.utils.http, but it imports from af64.runtime.db-auxiliary and af64.runtime.db-tasks which are defined later in the file
- **Fix:** Moved package declaration to after af64.runtime.db-tasks
- **Files modified:** packages.lisp

**2. [Rule 2 - Missing functionality] Added parse-task-id helper**
- **Found during:** Task 2
- **Issue:** sync-task-status needs to extract repo and issue number from task-id strings like "gh-dpn-api-42"
- **Fix:** Added parse-task-id function that handles the reverse parsing
- **Files modified:** util/github.lisp

## Verification Results

- AF64 system loads cleanly with `(asdf:load-system "af64")`
- All 6 exported symbols resolve: sync-all, sync-github-issues, push-tasks-to-github, sync-task-status, parse-github-url, github-task-id
- parse-github-url correctly parses standard URLs, .git suffixed, and trailing-slash URLs
- github-task-id returns "gh-dpn-api-42" for (github-task-id "dpn-api" 42)
- sync-all gracefully handles missing GITHUB_TOKEN (logs warning and returns)

## Known Stubs

None -- all functions are fully implemented and wired to real DB queries and HTTP calls.

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | b7c505c | DB helpers and package declaration for GitHub sync |
| 2 | 9881944 | Full GitHub sync module implementation |

## Self-Check: PASSED
