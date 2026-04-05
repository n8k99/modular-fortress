// Package db — generic query functions for polymorphic Nine Tables.
package db

import (
	"context"
	"fmt"
	"strings"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/tables"
)

// ListParams controls pagination, filtering, and search for list queries.
type ListParams struct {
	Table  string
	Kind   string // exact match on kind column (empty = no filter)
	Query  string // ILIKE search on title + body (empty = no search)
	Limit  int
	Offset int
}

// ListResult holds paginated rows and total count.
type ListResult struct {
	Table  string                   `json:"table"`
	Total  int                      `json:"total"`
	Limit  int                      `json:"limit"`
	Offset int                      `json:"offset"`
	Rows   []map[string]interface{} `json:"rows"`
}

// List returns paginated rows from a Nine Table with optional kind filter and text search.
// Table name MUST be validated against the whitelist before calling.
func List(ctx context.Context, pool *pgxpool.Pool, p ListParams) (*ListResult, error) {
	if !tables.IsValid(p.Table) {
		return nil, fmt.Errorf("invalid table: %s", p.Table)
	}

	// Determine which columns to select for the list view.
	// Infrastructure tables (the_links, the_index, the_aliases, the_ledger) have different schemas.
	cols := listColumnsFor(p.Table)

	// Build WHERE clause
	where, args := buildWhere(p.Kind, p.Query)

	// Count query
	countSQL := fmt.Sprintf("SELECT count(*) FROM %s%s", p.Table, where)
	var total int
	err := pool.QueryRow(ctx, countSQL, args...).Scan(&total)
	if err != nil {
		return nil, fmt.Errorf("count %s: %w", p.Table, err)
	}

	// Data query
	dataSQL := fmt.Sprintf("SELECT %s FROM %s%s ORDER BY id DESC LIMIT $%d OFFSET $%d",
		cols, p.Table, where, len(args)+1, len(args)+2)
	args = append(args, p.Limit, p.Offset)

	rows, err := pool.Query(ctx, dataSQL, args...)
	if err != nil {
		return nil, fmt.Errorf("list %s: %w", p.Table, err)
	}
	defer rows.Close()

	results, err := scanRows(rows)
	if err != nil {
		return nil, fmt.Errorf("scan %s: %w", p.Table, err)
	}

	return &ListResult{
		Table:  p.Table,
		Total:  total,
		Limit:  p.Limit,
		Offset: p.Offset,
		Rows:   results,
	}, nil
}

// GetByID returns a single full row from a Nine Table by primary key.
func GetByID(ctx context.Context, pool *pgxpool.Pool, table string, id int64) (map[string]interface{}, error) {
	if !tables.IsValid(table) {
		return nil, fmt.Errorf("invalid table: %s", table)
	}

	sql := fmt.Sprintf("SELECT * FROM %s WHERE id = $1", table)
	rows, err := pool.Query(ctx, sql, id)
	if err != nil {
		return nil, fmt.Errorf("get %s/%d: %w", table, id, err)
	}
	defer rows.Close()

	results, err := scanRows(rows)
	if err != nil {
		return nil, fmt.Errorf("scan %s/%d: %w", table, id, err)
	}

	if len(results) == 0 {
		return nil, nil // not found — caller returns 404
	}
	return results[0], nil
}

// GetBySlug returns a single full row from a Nine Table by slug.
func GetBySlug(ctx context.Context, pool *pgxpool.Pool, table string, slug string) (map[string]interface{}, error) {
	if !tables.IsValid(table) {
		return nil, fmt.Errorf("invalid table: %s", table)
	}

	sql := fmt.Sprintf("SELECT * FROM %s WHERE slug = $1", table)
	rows, err := pool.Query(ctx, sql, slug)
	if err != nil {
		return nil, fmt.Errorf("get %s/slug/%s: %w", table, slug, err)
	}
	defer rows.Close()

	results, err := scanRows(rows)
	if err != nil {
		return nil, fmt.Errorf("scan %s/slug/%s: %w", table, slug, err)
	}

	if len(results) == 0 {
		return nil, nil
	}
	return results[0], nil
}

// KindCount holds a kind value and its row count.
type KindCount struct {
	Kind  string `json:"kind"`
	Count int    `json:"count"`
}

// ListKinds returns distinct kind values with counts for a Nine Table.
// Returns nil for tables that don't have a kind column.
func ListKinds(ctx context.Context, pool *pgxpool.Pool, table string) ([]KindCount, error) {
	if !tables.IsValid(table) {
		return nil, fmt.Errorf("invalid table: %s", table)
	}

	// Infrastructure tables don't have a kind column
	switch table {
	case "the_links", "the_index", "the_aliases", "the_ledger":
		return nil, nil
	}

	sql := fmt.Sprintf("SELECT kind, count(*)::int FROM %s GROUP BY kind ORDER BY count(*) DESC", table)
	rows, err := pool.Query(ctx, sql)
	if err != nil {
		return nil, fmt.Errorf("kinds %s: %w", table, err)
	}
	defer rows.Close()

	var kinds []KindCount
	for rows.Next() {
		var kc KindCount
		if err := rows.Scan(&kc.Kind, &kc.Count); err != nil {
			return nil, err
		}
		kinds = append(kinds, kc)
	}
	return kinds, rows.Err()
}

// Create inserts a new row into a Nine Table from a map of field:value pairs.
// Fields must be pre-validated against ColumnMap. Returns the new row.
func Create(ctx context.Context, pool *pgxpool.Pool, table string, fields map[string]interface{}) (map[string]interface{}, error) {
	if !tables.IsValid(table) {
		return nil, fmt.Errorf("invalid table: %s", table)
	}
	if len(fields) == 0 {
		return nil, fmt.Errorf("no fields provided")
	}

	cols := make([]string, 0, len(fields))
	placeholders := make([]string, 0, len(fields))
	args := make([]interface{}, 0, len(fields))
	i := 1
	for k, v := range fields {
		cols = append(cols, k)
		placeholders = append(placeholders, fmt.Sprintf("$%d", i))
		args = append(args, v)
		i++
	}

	sql := fmt.Sprintf("INSERT INTO %s (%s) VALUES (%s) RETURNING *",
		table, strings.Join(cols, ", "), strings.Join(placeholders, ", "))

	rows, err := pool.Query(ctx, sql, args...)
	if err != nil {
		return nil, fmt.Errorf("create %s: %w", table, err)
	}
	defer rows.Close()

	results, err := scanRows(rows)
	if err != nil {
		return nil, fmt.Errorf("scan create %s: %w", table, err)
	}
	if len(results) == 0 {
		return nil, fmt.Errorf("create %s: no row returned", table)
	}
	return results[0], nil
}

// Update modifies specified fields on an existing row. Sets updated_at = now().
// Fields must be pre-validated against ColumnMap. Returns the updated row.
func Update(ctx context.Context, pool *pgxpool.Pool, table string, id int64, fields map[string]interface{}) (map[string]interface{}, error) {
	if !tables.IsValid(table) {
		return nil, fmt.Errorf("invalid table: %s", table)
	}
	if len(fields) == 0 {
		return nil, fmt.Errorf("no fields provided")
	}

	setClauses := make([]string, 0, len(fields)+1)
	args := make([]interface{}, 0, len(fields)+1)
	i := 1
	for k, v := range fields {
		setClauses = append(setClauses, fmt.Sprintf("%s = $%d", k, i))
		args = append(args, v)
		i++
	}
	setClauses = append(setClauses, "updated_at = now()")

	args = append(args, id)
	sql := fmt.Sprintf("UPDATE %s SET %s WHERE id = $%d RETURNING *",
		table, strings.Join(setClauses, ", "), i)

	rows, err := pool.Query(ctx, sql, args...)
	if err != nil {
		return nil, fmt.Errorf("update %s/%d: %w", table, id, err)
	}
	defer rows.Close()

	results, err := scanRows(rows)
	if err != nil {
		return nil, fmt.Errorf("scan update %s/%d: %w", table, id, err)
	}
	if len(results) == 0 {
		return nil, nil // not found
	}
	return results[0], nil
}

// Delete removes a row by ID. Returns true if a row was deleted.
func Delete(ctx context.Context, pool *pgxpool.Pool, table string, id int64) (bool, error) {
	if !tables.IsValid(table) {
		return false, fmt.Errorf("invalid table: %s", table)
	}

	sql := fmt.Sprintf("DELETE FROM %s WHERE id = $1", table)
	tag, err := pool.Exec(ctx, sql, id)
	if err != nil {
		return false, fmt.Errorf("delete %s/%d: %w", table, id, err)
	}
	return tag.RowsAffected() > 0, nil
}

// Move transfers a row from one table to another within a transaction.
// Reads full row from source, inserts into target with new kind, deletes from source.
func Move(ctx context.Context, pool *pgxpool.Pool, sourceTable string, id int64, targetTable string, newKind string) (map[string]interface{}, error) {
	if !tables.IsValid(sourceTable) {
		return nil, fmt.Errorf("invalid source table: %s", sourceTable)
	}
	if !tables.IsValid(targetTable) {
		return nil, fmt.Errorf("invalid target table: %s", targetTable)
	}
	if sourceTable == targetTable {
		return nil, fmt.Errorf("source and target are the same table")
	}

	tx, err := pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Read source row
	srcSQL := fmt.Sprintf("SELECT * FROM %s WHERE id = $1 FOR UPDATE", sourceTable)
	srcRows, err := tx.Query(ctx, srcSQL, id)
	if err != nil {
		return nil, fmt.Errorf("read source %s/%d: %w", sourceTable, id, err)
	}
	srcResults, err := scanRows(srcRows)
	srcRows.Close()
	if err != nil {
		return nil, fmt.Errorf("scan source: %w", err)
	}
	if len(srcResults) == 0 {
		return nil, nil // not found
	}

	srcRow := srcResults[0]

	// Remove id and set new kind
	delete(srcRow, "id")
	srcRow["kind"] = newKind
	srcRow["updated_at"] = "now()"

	// Build insert for target
	cols := make([]string, 0, len(srcRow))
	placeholders := make([]string, 0, len(srcRow))
	args := make([]interface{}, 0, len(srcRow))
	i := 1
	for k, v := range srcRow {
		cols = append(cols, k)
		if v == "now()" {
			placeholders = append(placeholders, "now()")
		} else {
			placeholders = append(placeholders, fmt.Sprintf("$%d", i))
			args = append(args, v)
			i++
		}
	}

	insertSQL := fmt.Sprintf("INSERT INTO %s (%s) VALUES (%s) RETURNING *",
		targetTable, strings.Join(cols, ", "), strings.Join(placeholders, ", "))

	destRows, err := tx.Query(ctx, insertSQL, args...)
	if err != nil {
		return nil, fmt.Errorf("insert into %s: %w", targetTable, err)
	}
	destResults, err := scanRows(destRows)
	destRows.Close()
	if err != nil {
		return nil, fmt.Errorf("scan dest: %w", err)
	}

	// Delete from source
	delSQL := fmt.Sprintf("DELETE FROM %s WHERE id = $1", sourceTable)
	_, err = tx.Exec(ctx, delSQL, id)
	if err != nil {
		return nil, fmt.Errorf("delete from %s: %w", sourceTable, err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("commit move: %w", err)
	}

	if len(destResults) == 0 {
		return nil, fmt.Errorf("move produced no result")
	}
	return destResults[0], nil
}

// listColumnsFor returns the SELECT columns appropriate for a table's list view.
func listColumnsFor(table string) string {
	switch table {
	case "the_links":
		return "id, source_table, source_id, target_slug, link_text, created_at"
	case "the_index":
		return "id, source_table, source_id, source_slug, source_kind, source_title, created_at"
	case "the_aliases":
		return "id, alias_slug, canonical_slug, source_table, created_at"
	case "the_ledger":
		return "id, tick_number, ghost_id, tick_status, action_taken, created_at"
	case "temporal":
		return "id, slug, kind, title, type, day, start, created_at, updated_at"
	case "identity":
		return "id, slug, kind, title, full_name, type, status, tier, created_at, updated_at"
	default:
		return strings.Join(tables.ListColumns, ", ")
	}
}

// buildWhere constructs a WHERE clause and args for kind filter and text search.
func buildWhere(kind, query string) (string, []interface{}) {
	var conditions []string
	var args []interface{}
	argN := 1

	if kind != "" {
		conditions = append(conditions, fmt.Sprintf("kind = $%d", argN))
		args = append(args, kind)
		argN++
	}

	if query != "" {
		conditions = append(conditions, fmt.Sprintf("(title ILIKE $%d OR body ILIKE $%d)", argN, argN))
		args = append(args, "%"+query+"%")
	}

	if len(conditions) == 0 {
		return "", nil
	}
	return " WHERE " + strings.Join(conditions, " AND "), args
}

// scanRows converts pgx rows into []map[string]any, handling NULLs gracefully.
func scanRows(rows pgx.Rows) ([]map[string]interface{}, error) {
	fieldDescs := rows.FieldDescriptions()
	var results []map[string]interface{}

	for rows.Next() {
		values, err := rows.Values()
		if err != nil {
			return nil, err
		}

		row := make(map[string]interface{}, len(fieldDescs))
		for i, fd := range fieldDescs {
			val := values[i]
			if val == nil {
				continue // omit NULL fields from JSON
			}
			row[fd.Name] = val
		}
		results = append(results, row)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}
	return results, nil
}
