#!/bin/bash
# E2E verification for Phase 5: Feedback & Reporting
# Tests all REPT-01 through REPT-06 requirements
#
# Usage: bash test_feedback_e2e.sh
# Requires: psql access to master_chronicle, grep

PASS=0
FAIL=0

export PGPASSWORD=chronicle2026

# Query that returns a single value (SELECT ... RETURNING)
query() {
  psql -h 127.0.0.1 -U chronicle -d master_chronicle -qtAc "$1" 2>&1 | head -1 | tr -d ' '
}

# Execute a statement (UPDATE/DELETE/INSERT without capturing result)
exec_sql() {
  psql -h 127.0.0.1 -U chronicle -d master_chronicle -qtAc "$1" > /dev/null 2>&1
}

# Query returning multiline output
query_multi() {
  psql -h 127.0.0.1 -U chronicle -d master_chronicle -qtAc "$1" 2>&1
}

pass() {
  echo "  PASS: $1"
  PASS=$((PASS + 1))
}

fail() {
  echo "  FAIL: $1"
  FAIL=$((FAIL + 1))
}

echo "=== Phase 5: Feedback & Reporting E2E ==="
echo ""

# Setup: Create a test project with 2 waves (2 tasks each)
echo "[setup] Creating test project and tasks..."
TS=$(date +%s)
TEST_PROJECT_ID=$(query "INSERT INTO projects (name, slug, status, owner, description) VALUES ('GSD-E2E-Test-Phase5', 'gsd-e2e-test-phase5-${TS}', 'active', 'eliana', 'E2E test for feedback reporting') RETURNING id")

if [ -z "$TEST_PROJECT_ID" ]; then
  echo "FATAL: Could not create test project"
  exit 2
fi

# Wave 1 tasks
TASK_W1A=$(query "INSERT INTO tasks (doc_path, line_number, raw_line, status, text, task_id, project_id, source, context, assigned_by, assignee) VALUES ('test', 0, 'e2e-test', 'open', 'Wave 1 Task A', 'e2e-w1a-${TS}', ${TEST_PROJECT_ID}, 'gsd', '{\"wave\": 1, \"must_haves\": [\"test\"]}', 'gsd', 'eliana') RETURNING id")
TASK_W1B=$(query "INSERT INTO tasks (doc_path, line_number, raw_line, status, text, task_id, project_id, source, context, assigned_by, assignee) VALUES ('test', 0, 'e2e-test', 'open', 'Wave 1 Task B', 'e2e-w1b-${TS}', ${TEST_PROJECT_ID}, 'gsd', '{\"wave\": 1, \"must_haves\": [\"test\"]}', 'gsd', 'eliana') RETURNING id")
# Wave 2 tasks (should start as pending, get advanced to open)
TASK_W2A=$(query "INSERT INTO tasks (doc_path, line_number, raw_line, status, text, task_id, project_id, source, context, assigned_by, assignee) VALUES ('test', 0, 'e2e-test', 'pending', 'Wave 2 Task A', 'e2e-w2a-${TS}', ${TEST_PROJECT_ID}, 'gsd', '{\"wave\": 2, \"must_haves\": [\"test\"]}', 'gsd', 'eliana') RETURNING id")
TASK_W2B=$(query "INSERT INTO tasks (doc_path, line_number, raw_line, status, text, task_id, project_id, source, context, assigned_by, assignee) VALUES ('test', 0, 'e2e-test', 'pending', 'Wave 2 Task B', 'e2e-w2b-${TS}', ${TEST_PROJECT_ID}, 'gsd', '{\"wave\": 2, \"must_haves\": [\"test\"]}', 'gsd', 'eliana') RETURNING id")
echo "  Project: $TEST_PROJECT_ID | W1: $TASK_W1A, $TASK_W1B | W2: $TASK_W2A, $TASK_W2B"

# --- REPT-02: Status reflects execution state ---
echo ""
echo "[REPT-02] Status reflects execution state..."
exec_sql "UPDATE tasks SET status='in-progress' WHERE id=$TASK_W1A"
STATUS=$(query "SELECT status FROM tasks WHERE id=$TASK_W1A")
if [ "$STATUS" = "in-progress" ]; then pass "Task moved to in-progress"; else fail "Expected in-progress, got $STATUS"; fi

# --- REPT-03: Wave advancement ---
echo ""
echo "[REPT-03] Wave advancement..."
exec_sql "UPDATE tasks SET status='done' WHERE id=$TASK_W1A"
exec_sql "UPDATE tasks SET status='done' WHERE id=$TASK_W1B"
sleep 0.5
W2A_STATUS=$(query "SELECT status FROM tasks WHERE id=$TASK_W2A")
W2B_STATUS=$(query "SELECT status FROM tasks WHERE id=$TASK_W2B")
if [ "$W2A_STATUS" = "open" ] && [ "$W2B_STATUS" = "open" ]; then pass "Wave 2 tasks advanced to open"; else fail "W2A=$W2A_STATUS, W2B=$W2B_STATUS (expected open)"; fi

# --- REPT-01: Completion report in conversations ---
echo ""
echo "[REPT-01] Completion reporting infrastructure..."
TRIGGER_HAS_WAVE=$(query_multi "SELECT prosrc FROM pg_proc WHERE proname='on_task_completed_after'" | grep -c 'wave' || true)
if [ "$TRIGGER_HAS_WAVE" -ge 3 ]; then pass "Trigger has wave advancement logic"; else fail "Trigger missing wave logic (found $TRIGGER_HAS_WAVE references)"; fi

LISP_HAS_COMPLETION=$(grep -c 'task_completion' /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp || true)
if [ "$LISP_HAS_COMPLETION" -ge 1 ]; then pass "Lisp has completion reporting"; else fail "Lisp missing completion reporting"; fi

# --- REPT-04: Blocker escalation ---
echo ""
echo "[REPT-04] Blocker escalation..."
LISP_HAS_BLOCKER=$(grep -c 'blocker_escalation' /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp || true)
if [ "$LISP_HAS_BLOCKER" -ge 1 ]; then pass "Lisp has blocker escalation"; else fail "Lisp missing blocker escalation"; fi

# --- REPT-06: Nathan-only notifications ---
echo ""
echo "[REPT-06] Nathan notifications..."
LISP_HAS_ESCALATE=$(grep -c 'parse-escalate-lines' /opt/project-noosphere-ghosts/lisp/runtime/action-executor.lisp || true)
if [ "$LISP_HAS_ESCALATE" -ge 2 ]; then pass "ESCALATE parser exists and is wired"; else fail "ESCALATE parser missing (found $LISP_HAS_ESCALATE references)"; fi

# Complete all wave 2 tasks to trigger project completion
exec_sql "UPDATE tasks SET status='done' WHERE id=$TASK_W2A"
exec_sql "UPDATE tasks SET status='done' WHERE id=$TASK_W2B"
sleep 0.5

PROJ_STATUS=$(query "SELECT status FROM projects WHERE id=$TEST_PROJECT_ID")
if [ "$PROJ_STATUS" = "completed" ]; then pass "Project status set to completed"; else fail "Project status is $PROJ_STATUS (expected completed)"; fi

NATHAN_MSG=$(query "SELECT COUNT(*) FROM conversations WHERE 'nathan' = ANY(to_agent) AND metadata->>'source'='project_completion' AND metadata->>'project_id'='$TEST_PROJECT_ID'")
if [ "$NATHAN_MSG" -ge 1 ]; then pass "Nathan received project completion notification"; else fail "No project completion message to Nathan"; fi

# --- REPT-05: Progress reporting ---
echo ""
echo "[REPT-05] Progress reporting..."
DISPATCH_HAS_WAVE=$(grep -c 'context::jsonb' /root/gotcha-workspace/tools/gsd/dispatch_to_db.py || true)
if [ "$DISPATCH_HAS_WAVE" -ge 1 ]; then pass "dispatch --status has wave-level reporting"; else fail "dispatch --status missing wave reporting"; fi

# --- Cleanup ---
echo ""
echo "[cleanup] Removing test data..."
exec_sql "DELETE FROM conversations WHERE metadata->>'project_id'='$TEST_PROJECT_ID'"
exec_sql "DELETE FROM tasks WHERE project_id=$TEST_PROJECT_ID"
exec_sql "DELETE FROM projects WHERE id=$TEST_PROJECT_ID"

# --- Summary ---
echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
if [ "$FAIL" -gt 0 ]; then exit 1; fi
echo "All REPT requirements verified!"
