#!/bin/bash
# E2E Tool Smoke Tests — Phase 04
# Validates all 6 TOOL requirements plus D-08 stage_notes persistence
# Usage: bash test_tools_e2e.sh [--quick]

set -o pipefail

PASS=0; FAIL=0; SKIP=0
QUICK=false
[[ "$1" == "--quick" ]] && QUICK=true

REGISTRY="/opt/project-noosphere-ghosts/config/tool-registry.json"
CLAUDE_TOOL="/opt/project-noosphere-ghosts/tools/claude-code-tool.sh"
TOOL_SOCKET="/opt/project-noosphere-ghosts/lisp/runtime/tool-socket.lisp"
ACTION_EXEC="/opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp"
API_BASE="http://127.0.0.1:8080"
API_KEY="dpn-nova-2026"
PYTHON="/root/gotcha-workspace/.venv/bin/python3"
LIST_TASKS="/root/gotcha-workspace/tools/engineering/list_unassigned_tasks.py"
READ_MEMORY="/root/gotcha-workspace/tools/engineering/read_own_memory.py"
DB_CMD="PGPASSWORD=chronicle2026 psql -h 127.0.0.1 -U chronicle -d master_chronicle -t -c"

test_result() {
  local name="$1" result="$2"
  if [ "$result" = "PASS" ]; then ((PASS++)); echo "  PASS: $name"
  elif [ "$result" = "SKIP" ]; then ((SKIP++)); echo "  SKIP: $name"
  else ((FAIL++)); echo "  FAIL: $name -- $result"; fi
}

# =============================================================================
echo "=== TOOL-01: Code Tools (Claude Code CLI) ==="
# =============================================================================

# Check script exists and is executable
if [ -x "$CLAUDE_TOOL" ]; then
  test_result "claude-code-tool.sh exists and is executable" "PASS"
else
  test_result "claude-code-tool.sh exists and is executable" "Script missing or not executable"
fi

# Check script contains correct invocation pattern
if grep -q 'claude -p' "$CLAUDE_TOOL" 2>/dev/null; then
  test_result "claude-code-tool.sh has correct CLI invocation" "PASS"
else
  test_result "claude-code-tool.sh has correct CLI invocation" "Missing 'claude -p' pattern"
fi

# Check tool registered in registry
if jq -e '.tools.claude_code.script' "$REGISTRY" >/dev/null 2>&1; then
  test_result "claude_code registered in tool-registry.json" "PASS"
else
  test_result "claude_code registered in tool-registry.json" "Not found in registry"
fi

if [ "$QUICK" = false ]; then
  # Live invocation test (requires Claude Code CLI)
  OUTPUT=$("$CLAUDE_TOOL" "Respond with exactly: TOOL_TEST_OK" "Read" 2>/dev/null)
  EXIT_CODE=$?
  if [ $EXIT_CODE -eq 0 ] && [ -n "$OUTPUT" ] && ! echo "$OUTPUT" | grep -q '"error"'; then
    test_result "claude-code-tool.sh live invocation" "PASS"
  else
    test_result "claude-code-tool.sh live invocation" "Exit=$EXIT_CODE or empty/error output"
  fi
else
  test_result "claude-code-tool.sh live invocation (skipped in --quick)" "SKIP"
fi

# =============================================================================
echo ""
echo "=== TOOL-02: DB Tools (query_db, write_document) ==="
# =============================================================================

# Test query_db: SELECT count from tasks
COUNT=$(eval $DB_CMD "\"SELECT count(*) FROM tasks;\"" 2>/dev/null | tr -d ' \n')
if [[ "$COUNT" =~ ^[0-9]+$ ]] && [ "$COUNT" -gt 0 ]; then
  test_result "query_db: SELECT count(*) FROM tasks returns $COUNT" "PASS"
else
  test_result "query_db: SELECT count(*) FROM tasks" "Got: '$COUNT' (expected number > 0)"
fi

# Test write_document: INSERT with ON CONFLICT
DOC_ID=$(eval $DB_CMD "\"INSERT INTO documents (path, title, content, created_at, modified_at) VALUES ('test/tool-smoke-test', 'Smoke Test', 'E2E test content', NOW(), NOW()) ON CONFLICT (path) DO UPDATE SET content = EXCLUDED.content, modified_at = NOW() RETURNING id;\"" 2>/dev/null | head -1 | tr -d ' \n')
if [[ "$DOC_ID" =~ ^[0-9]+$ ]]; then
  test_result "write_document: INSERT/UPSERT returns id=$DOC_ID" "PASS"
  # Clean up
  eval $DB_CMD "\"DELETE FROM documents WHERE path = 'test/tool-smoke-test';\"" >/dev/null 2>&1
else
  test_result "write_document: INSERT/UPSERT" "Got: '$DOC_ID' (expected numeric id)"
fi

# =============================================================================
echo ""
echo "=== TOOL-03: API/Task Tools (list_tasks, task API) ==="
# =============================================================================

# Test list_unassigned_tasks.py
if [ -f "$LIST_TASKS" ]; then
  TASK_OUTPUT=$($PYTHON "$LIST_TASKS" --limit 1 2>/dev/null)
  TASK_EXIT=$?
  if [ $TASK_EXIT -eq 0 ]; then
    test_result "list_unassigned_tasks.py executes (exit 0)" "PASS"
  else
    test_result "list_unassigned_tasks.py executes" "Exit code $TASK_EXIT"
  fi
else
  test_result "list_unassigned_tasks.py exists" "File not found"
fi

# Test task API via curl
API_TASKS=$(curl -s "$API_BASE/api/af64/tasks?limit=1" -H "X-API-Key: $API_KEY" 2>/dev/null)
if echo "$API_TASKS" | jq -e 'type == "array" or .tasks' >/dev/null 2>&1; then
  test_result "Task API returns JSON with task data" "PASS"
else
  test_result "Task API returns JSON with task data" "Response: ${API_TASKS:0:100}"
fi

# =============================================================================
echo ""
echo "=== TOOL-04: External Tools (DEFERRED) ==="
# =============================================================================

echo "  TOOL-04: DEFERRED -- No web_search or url_fetch implementations exist."
echo "  RSS-based news_aggregator and article_fetcher exist but are not general external tools."
echo "  Defer to Phase 4.5 or v2."
test_result "TOOL-04 external tools (deferred per plan)" "SKIP"

# =============================================================================
echo ""
echo "=== TOOL-05: Scope Enforcement ==="
# =============================================================================

# Verify claude_code has engineering+tools scope
CC_SCOPE=$(jq -r '.tools.claude_code.scope | sort | join(",")' "$REGISTRY" 2>/dev/null)
if echo "$CC_SCOPE" | grep -q "engineering" && echo "$CC_SCOPE" | grep -q "tools"; then
  test_result "claude_code scope contains engineering+tools" "PASS"
else
  test_result "claude_code scope contains engineering+tools" "Got: $CC_SCOPE"
fi

# Verify read_own_memory has wildcard scope
ROM_SCOPE=$(jq -r '.tools.read_own_memory.scope' "$REGISTRY" 2>/dev/null)
if [ "$ROM_SCOPE" = "*" ]; then
  test_result "read_own_memory scope is wildcard (*)" "PASS"
else
  test_result "read_own_memory scope is wildcard (*)" "Got: $ROM_SCOPE"
fi

# Verify tool-socket.lisp contains wildcard fix
if grep -q 'stringp tool-scope' "$TOOL_SOCKET" 2>/dev/null; then
  test_result "tool-socket.lisp has wildcard scope fix" "PASS"
else
  test_result "tool-socket.lisp has wildcard scope fix" "Pattern not found"
fi

# Verify eliana (engineering exec) has engineering in tool_scope via API
ELIANA_SCOPE=$(curl -s "$API_BASE/api/agents/eliana" -H "X-API-Key: $API_KEY" 2>/dev/null | jq -r '.agent.tool_scope // empty' 2>/dev/null)
if echo "$ELIANA_SCOPE" | grep -q "engineering"; then
  test_result "Agent eliana has 'engineering' in tool_scope" "PASS"
else
  test_result "Agent eliana has 'engineering' in tool_scope" "Got: $ELIANA_SCOPE"
fi

# Verify sylvia (content chief) does NOT have engineering in scope
SYLVIA_SCOPE=$(curl -s "$API_BASE/api/agents/sylvia" -H "X-API-Key: $API_KEY" 2>/dev/null | jq -r '.agent.tool_scope // empty' 2>/dev/null)
if echo "$SYLVIA_SCOPE" | grep -q "content" && ! echo "$SYLVIA_SCOPE" | grep -q '"engineering"'; then
  test_result "Agent sylvia has 'content' but NOT 'engineering'" "PASS"
else
  test_result "Agent sylvia has 'content' but NOT 'engineering'" "Got: $SYLVIA_SCOPE"
fi

# =============================================================================
echo ""
echo "=== TOOL-06: Anti-hallucination Validation ==="
# =============================================================================

# Verify tools-executed validation exists
if grep -q 'tools-executed' "$ACTION_EXEC" 2>/dev/null; then
  test_result "action-executor.lisp has tools-executed validation" "PASS"
else
  test_result "action-executor.lisp has tools-executed validation" "Pattern not found"
fi

# Verify rejection message exists
if grep -q 'REJECTED.*0 tools executed' "$ACTION_EXEC" 2>/dev/null; then
  test_result "action-executor.lisp has REJECTED message for 0 tools" "PASS"
else
  test_result "action-executor.lisp has REJECTED message for 0 tools" "Pattern not found"
fi

# =============================================================================
echo ""
echo "=== TOOL-05 bonus: Memory Tools ==="
# =============================================================================

# Test read_own_memory.py exists and is callable
if [ -f "$READ_MEMORY" ]; then
  MEM_OUTPUT=$($PYTHON "$READ_MEMORY" nova --days 1 2>/dev/null)
  MEM_EXIT=$?
  if [ $MEM_EXIT -eq 0 ]; then
    test_result "read_own_memory.py executes for agent nova (exit 0)" "PASS"
  else
    test_result "read_own_memory.py executes for agent nova" "Exit code $MEM_EXIT"
  fi
else
  test_result "read_own_memory.py exists" "File not found"
fi

# =============================================================================
echo ""
echo "=== D-08: stage_notes Persistence ==="
# =============================================================================

if [ "$QUICK" = true ]; then
  # Quick mode: just verify column exists
  COL=$(eval $DB_CMD "\"SELECT column_name FROM information_schema.columns WHERE table_name='tasks' AND column_name='stage_notes';\"" 2>/dev/null | tr -d ' \n')
  if [ "$COL" = "stage_notes" ]; then
    test_result "stage_notes column exists in tasks table" "PASS"
  else
    test_result "stage_notes column exists in tasks table" "Column not found"
  fi
else
  # Full mode: insert, query, verify, cleanup
  eval $DB_CMD "\"INSERT INTO tasks (task_id, title, status, source, stage_notes) VALUES ('test-tool-e2e-stage', 'E2E Stage Notes Test', 'testing', 'e2e-test', 'tool_output: test result from claude_code') ON CONFLICT (task_id) DO UPDATE SET stage_notes = EXCLUDED.stage_notes;\"" >/dev/null 2>&1
  STAGE_VAL=$(eval $DB_CMD "\"SELECT stage_notes FROM tasks WHERE task_id = 'test-tool-e2e-stage';\"" 2>/dev/null | tr -d '\n' | xargs)
  if echo "$STAGE_VAL" | grep -q "tool_output"; then
    test_result "stage_notes stores and retrieves tool output" "PASS"
  else
    test_result "stage_notes stores and retrieves tool output" "Got: '$STAGE_VAL'"
  fi
  # Clean up
  eval $DB_CMD "\"DELETE FROM tasks WHERE task_id = 'test-tool-e2e-stage';\"" >/dev/null 2>&1
fi

# =============================================================================
echo ""
echo "=== Results ==="
echo "PASS: $PASS  FAIL: $FAIL  SKIP: $SKIP"
TOTAL=$((PASS + FAIL + SKIP))
echo "Total: $TOTAL tests"
[ $FAIL -eq 0 ] && exit 0 || exit 1
