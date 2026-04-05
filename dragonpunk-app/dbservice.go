// DbService exposes master_chronicle database operations to the Wails frontend.
// It wraps the existing dragonpunk/internal packages — no database code is duplicated.
package main

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/config"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/db"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/tables"
	"github.com/wailsapp/wails/v3/pkg/application"
)

// DbService is bound to the Wails frontend. All public methods are callable from TypeScript.
type DbService struct {
	pool *pgxpool.Pool
}

// HealthResult is returned by Health().
type HealthResult struct {
	Status     string `json:"status"`
	Connected  bool   `json:"connected"`
	TableCount int    `json:"table_count"`
	Timestamp  string `json:"timestamp"`
}

// TableInfo is one row in the ListTables response.
type TableInfo struct {
	Name     string `json:"name"`
	RowCount int    `json:"row_count"`
}

// KindInfo is one row in the ListKinds response.
type KindInfo struct {
	Kind  string `json:"kind"`
	Count int    `json:"count"`
}

// ServiceStartup is called by Wails when the app starts. Connects to PostgreSQL.
func (d *DbService) ServiceStartup(ctx context.Context, options application.ServiceOptions) error {
	cfg, err := config.Load()
	if err != nil {
		slog.Error("dbservice: config load failed", "error", err)
		return fmt.Errorf("config load: %w", err)
	}

	pool, err := db.NewPool(ctx, cfg.DatabaseURL)
	if err != nil {
		slog.Error("dbservice: database connection failed", "error", err)
		return fmt.Errorf("db connect: %w", err)
	}

	d.pool = pool
	slog.Info("dbservice: connected to master_chronicle", "url", db.RedactURL(cfg.DatabaseURL))
	return nil
}

// ServiceShutdown is called by Wails when the app exits. Closes the database pool.
func (d *DbService) ServiceShutdown() error {
	if d.pool != nil {
		d.pool.Close()
		slog.Info("dbservice: database pool closed")
	}
	return nil
}

// Health returns database connectivity status.
func (d *DbService) Health() HealthResult {
	result := HealthResult{
		Timestamp: time.Now().UTC().Format(time.RFC3339),
	}

	if d.pool == nil {
		result.Status = "disconnected"
		return result
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	var count int
	err := d.pool.QueryRow(ctx,
		"SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public'",
	).Scan(&count)

	if err != nil {
		slog.Warn("dbservice: health check failed", "error", err)
		result.Status = "degraded"
		return result
	}

	result.Status = "ok"
	result.Connected = true
	result.TableCount = count
	slog.Info("dbservice: health check", "tables", count)
	return result
}

// ListTables returns all Nine Tables with row counts.
func (d *DbService) ListTables() ([]TableInfo, error) {
	if d.pool == nil {
		return nil, fmt.Errorf("not connected")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	var results []TableInfo
	for _, name := range tables.Names() {
		var count int
		query := fmt.Sprintf("SELECT count(*) FROM %s", name) // safe — name from whitelist
		err := d.pool.QueryRow(ctx, query).Scan(&count)
		if err != nil {
			slog.Warn("dbservice: count failed", "table", name, "error", err)
			continue
		}
		results = append(results, TableInfo{Name: name, RowCount: count})
	}

	slog.Info("dbservice: listed tables", "count", len(results))
	return results, nil
}

// ListKinds returns the distinct kind values and counts for a given table.
func (d *DbService) ListKinds(table string) ([]KindInfo, error) {
	if d.pool == nil {
		return nil, fmt.Errorf("not connected")
	}

	if !tables.IsValid(table) {
		return nil, fmt.Errorf("unknown table: %s", table)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	query := fmt.Sprintf(
		"SELECT kind, count(*) as cnt FROM %s WHERE kind IS NOT NULL GROUP BY kind ORDER BY cnt DESC",
		table,
	)

	rows, err := d.pool.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("query failed: %w", err)
	}
	defer rows.Close()

	var results []KindInfo
	for rows.Next() {
		var ki KindInfo
		if err := rows.Scan(&ki.Kind, &ki.Count); err != nil {
			return nil, fmt.Errorf("scan failed: %w", err)
		}
		results = append(results, ki)
	}

	slog.Info("dbservice: listed kinds", "table", table, "kinds", len(results))
	return results, nil
}
