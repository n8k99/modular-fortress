#!/bin/bash
# E2E test for Phase 2: Perception Pipeline
# Tests all PERC requirements against live dpn-api
#
# Usage: bash test_perception_e2e.sh
# Requires: curl, jq, psql access to master_chronicle

set -euo pipefail

API="http://127.0.0.1:8080/api"
HDR="X-API-Key: dpn-nova-2026"
PASS=0
FAIL=0

test_req() {
  local req=$1 desc=$2 result=$3
  if [ "$result" = "true" ]; then
    echo "PASS: $req - $desc"
    PASS=$((PASS+1))
  else
    echo "FAIL: $req - $desc"
    FAIL=$((FAIL+1))
  fi
}

echo "=== Phase 2: Perception Pipeline E2E Tests ==="
echo "API: $API"
echo ""

# Fetch Eliana's perception once and reuse
ELIANA_RESPONSE=$(curl -s -H "$HDR" "$API/perception/eliana")

# -------------------------------------------------------------------
# PERC-01: Perception returns project-linked GSD tasks
# -------------------------------------------------------------------
echo "--- PERC-01: GSD tasks with project linkage ---"
GSD_TASKS_COUNT=$(echo "$ELIANA_RESPONSE" | jq '[.tasks[] | select(.source == "gsd" and .project_id != null)] | length')
test_req "PERC-01" "Executive perception returns GSD tasks with project_id" "$([ "$GSD_TASKS_COUNT" -gt 0 ] && echo true || echo false)"
echo "  (found $GSD_TASKS_COUNT GSD tasks with project_id)"

# -------------------------------------------------------------------
# PERC-02: Executive perceives owned projects with goals
# -------------------------------------------------------------------
echo ""
echo "--- PERC-02: Executive perceives owned projects ---"
PROJECTS_WITH_GOALS=$(echo "$ELIANA_RESPONSE" | jq '[.projects[] | select(.goals != null)] | length')
test_req "PERC-02" "Executive perception returns projects with goals" "$([ "$PROJECTS_WITH_GOALS" -gt 0 ] && echo true || echo false)"
echo "  (found $PROJECTS_WITH_GOALS projects with goals)"

# Check for project #51 specifically if it exists
PROJECT_51_NAME=$(echo "$ELIANA_RESPONSE" | jq -r '.projects[] | select(.id == 51) | .name // empty' 2>/dev/null || echo "")
if [ -n "$PROJECT_51_NAME" ]; then
  echo "  project #51 name: $PROJECT_51_NAME"
fi

# -------------------------------------------------------------------
# PERC-03: Staff perceives tasks with context and must_haves
# -------------------------------------------------------------------
echo ""
echo "--- PERC-03: Staff perceives tasks with context/must_haves ---"

# Try to find a staff agent with GSD tasks
STAFF_AGENT=$(psql -U chronicle -d master_chronicle -t -A -c \
  "SELECT DISTINCT unnest(assigned_to) FROM tasks WHERE source='gsd' AND assigned_to IS NOT NULL LIMIT 1" 2>/dev/null || echo "")
STAFF_AGENT=$(echo "$STAFF_AGENT" | tr -d '[:space:]')

if [ -z "$STAFF_AGENT" ] || [ "$STAFF_AGENT" = "" ]; then
  # Fallback: use eliana (executive also sees assigned GSD tasks)
  STAFF_AGENT="eliana"
  echo "  (no staff with GSD tasks found, using eliana as fallback)"
fi

STAFF_RESPONSE=$(curl -s -H "$HDR" "$API/perception/$STAFF_AGENT")
CONTEXT_TASKS=$(echo "$STAFF_RESPONSE" | jq '[.tasks[] | select(.context != null and (.context | contains("must_haves")))] | length')
test_req "PERC-03" "Agent '$STAFF_AGENT' perceives tasks with must_haves context" "$([ "$CONTEXT_TASKS" -gt 0 ] && echo true || echo false)"
echo "  (found $CONTEXT_TASKS tasks with must_haves in context)"

# -------------------------------------------------------------------
# PERC-04: Project ownership triggers urgency boost
# -------------------------------------------------------------------
echo ""
echo "--- PERC-04: Project ownership urgency boost ---"

# Check 1: API returns non-empty projects for Eliana (prerequisite for boost)
PROJECTS_COUNT=$(echo "$ELIANA_RESPONSE" | jq '.projects | length')
API_HAS_PROJECTS="$([ "$PROJECTS_COUNT" -gt 0 ] && echo true || echo false)"
echo "  API returns projects for eliana: $API_HAS_PROJECTS ($PROJECTS_COUNT projects)"

# Check 2: Lisp tick-engine.lisp contains the +15 boost code
LISP_BOOST_EXISTS=$(grep -c '(\* 15 (length projects))' /opt/project-noosphere-ghosts/lisp/runtime/tick-engine.lisp 2>/dev/null || echo 0)
LISP_OK="$([ "$LISP_BOOST_EXISTS" -gt 0 ] && echo true || echo false)"
echo "  Lisp tick-engine has (* 15 (length projects)): $LISP_OK"

# Both must be true
test_req "PERC-04" "Project ownership produces urgency boost data" "$([ "$API_HAS_PROJECTS" = "true" ] && [ "$LISP_OK" = "true" ] && echo true || echo false)"

# -------------------------------------------------------------------
# PERC-05: scheduled_at field present for Lisp client-side filtering
# -------------------------------------------------------------------
echo ""
echo "--- PERC-05: scheduled_at field present ---"

# Check API response includes the scheduled_at key on tasks
HAS_SCHEDULED_AT=$(echo "$ELIANA_RESPONSE" | jq '.tasks[0] | has("scheduled_at")')
test_req "PERC-05" "Tasks include scheduled_at field" "$HAS_SCHEDULED_AT"

# Verify Lisp filter exists
LISP_FILTER_EXISTS=$(grep -c 'filter-scheduled-tasks' /opt/project-noosphere-ghosts/lisp/runtime/task-scheduler.lisp 2>/dev/null || echo 0)
echo "  Lisp filter-scheduled-tasks function exists: $([ "$LISP_FILTER_EXISTS" -gt 0 ] && echo yes || echo no)"

# -------------------------------------------------------------------
# Summary
# -------------------------------------------------------------------
echo ""
echo "---"
echo "Results: $PASS passed, $FAIL failed out of $((PASS+FAIL)) tests"
exit $FAIL
