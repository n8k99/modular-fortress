#!/bin/bash
# run.sh — Execute the Nine Tables schema against DoltgreSQL
# Usage: ./run.sh [doltgres_port] [database]
#
# Executes all schema files in order. Each file is idempotent-safe
# with CREATE TABLE (will fail if table exists — run on a clean DB).
#
# Designed to run against DoltgreSQL. Start doltgresql first:
#   doltgresql --data-dir /root/doltgres/databases --port 5435 &

PORT="${1:-5435}"
DB="${2:-noosphere}"
HOST="127.0.0.1"
USER="root"

echo "=== Noosphere v2.0 — The Nine Tables ==="
echo "Target: postgresql://${USER}@${HOST}:${PORT}/${DB}"
echo ""

# Create database if it doesn't exist
psql -h "$HOST" -p "$PORT" -U "$USER" -d postgres -c "CREATE DATABASE ${DB};" 2>/dev/null || true

SCHEMA_DIR="$(dirname "$0")/schema"
ERRORS=0

for sql_file in "$SCHEMA_DIR"/[0-9]*.sql; do
    filename=$(basename "$sql_file")
    echo -n "  ${filename} ... "

    output=$(psql -h "$HOST" -p "$PORT" -U "$USER" -d "$DB" -f "$sql_file" 2>&1)
    exit_code=$?

    if [ $exit_code -eq 0 ]; then
        echo "OK"
    else
        echo "FAILED"
        echo "$output" | head -5
        ERRORS=$((ERRORS + 1))
    fi
done

echo ""
if [ $ERRORS -eq 0 ]; then
    echo "All schema files executed successfully."
    echo ""
    echo "Next: dolt commit in the database directory"
    echo "  cd /root/doltgres/databases/noosphere"
    echo "  dolt add ."
    echo "  dolt commit -m 'v2.0: The Nine Tables schema'"
else
    echo "${ERRORS} file(s) failed. Check output above."
fi
