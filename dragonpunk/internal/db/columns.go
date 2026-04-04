// Package db — column metadata for validating write operations.
package db

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/n8k99/modular-fortress/dragonpunk/internal/tables"
)

// ColumnMap caches valid column names per table.
// Used to validate JSON field names before building dynamic SQL.
type ColumnMap map[string]map[string]bool

// LoadColumns queries information_schema for all Nine Tables and caches their column names.
// Call once at startup.
func LoadColumns(ctx context.Context, pool *pgxpool.Pool) (ColumnMap, error) {
	cm := make(ColumnMap)

	rows, err := pool.Query(ctx, `
		SELECT table_name, column_name 
		FROM information_schema.columns 
		WHERE table_schema = 'public' 
		ORDER BY table_name, ordinal_position`)
	if err != nil {
		return nil, fmt.Errorf("load columns: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var table, col string
		if err := rows.Scan(&table, &col); err != nil {
			return nil, err
		}
		if !tables.IsValid(table) {
			continue
		}
		if cm[table] == nil {
			cm[table] = make(map[string]bool)
		}
		cm[table][col] = true
	}

	for _, name := range tables.Names() {
		if cols, ok := cm[name]; ok {
			slog.Info("columns loaded", "table", name, "count", len(cols))
		}
	}

	return cm, rows.Err()
}

// ValidColumns filters a map of field:value pairs to only include valid column names
// for the given table. Returns the filtered map and any rejected field names.
func (cm ColumnMap) ValidColumns(table string, fields map[string]interface{}) (valid map[string]interface{}, rejected []string) {
	tableCols := cm[table]
	if tableCols == nil {
		return nil, nil
	}

	valid = make(map[string]interface{})
	for k, v := range fields {
		// Never allow writing to these auto-managed columns
		switch k {
		case "id", "created_at":
			rejected = append(rejected, k)
			continue
		}
		if tableCols[k] {
			valid[k] = v
		} else {
			rejected = append(rejected, k)
		}
	}
	return valid, rejected
}
