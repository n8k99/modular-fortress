package api

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/db"
)

// Handlers holds shared dependencies for API handlers.
type Handlers struct {
	pool    *pgxpool.Pool
	columns db.ColumnMap
}

// HealthResponse is the JSON body returned by GET /api/health.
type HealthResponse struct {
	Status      string `json:"status"`
	DBConnected bool   `json:"db_connected"`
	TableCount  int    `json:"table_count"`
	Timestamp   string `json:"timestamp"`
}

// Health reports database connectivity and table count from master_chronicle.
func (h *Handlers) Health(w http.ResponseWriter, r *http.Request) {
	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()

	resp := HealthResponse{
		Timestamp: time.Now().UTC().Format(time.RFC3339),
	}

	var count int
	err := h.pool.QueryRow(ctx,
		"SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public'",
	).Scan(&count)

	if err != nil {
		slog.Warn("health check: database query failed", "error", err)
		resp.Status = "degraded"
		resp.DBConnected = false
	} else {
		resp.Status = "ok"
		resp.DBConnected = true
		resp.TableCount = count
	}

	w.Header().Set("Content-Type", "application/json")
	if !resp.DBConnected {
		w.WriteHeader(http.StatusServiceUnavailable)
	}
	json.NewEncoder(w).Encode(resp)
}
