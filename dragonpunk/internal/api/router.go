// Package api provides HTTP routing and handlers for Dragonpunk.
package api

import (
	"log/slog"
	"net/http"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/db"
)

// NewRouter builds the Dragonpunk HTTP router.
func NewRouter(pool *pgxpool.Pool, columns db.ColumnMap) http.Handler {
	mux := http.NewServeMux()

	h := &Handlers{pool: pool, columns: columns}

	// Health
	mux.HandleFunc("GET /api/health", h.Health)

	// Nine Tables — Read API
	mux.HandleFunc("GET /api/{table}/slug/{slug}", h.GetBySlug)
	mux.HandleFunc("GET /api/{table}/kinds", h.ListKinds)
	mux.HandleFunc("GET /api/{table}/{id}", h.GetRow)
	mux.HandleFunc("GET /api/{table}", h.ListTable)

	// Nine Tables — Write API
	mux.HandleFunc("POST /api/{table}/{id}/move", h.MoveRow)
	mux.HandleFunc("POST /api/{table}", h.CreateRow)
	mux.HandleFunc("PATCH /api/{table}/{id}", h.UpdateRow)
	mux.HandleFunc("DELETE /api/{table}/{id}", h.DeleteRow)

	return withLogging(mux)
}

// withLogging wraps a handler with structured request logging.
func withLogging(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		next.ServeHTTP(w, r)
		slog.Info("request",
			"method", r.Method,
			"path", r.URL.Path,
			"query", r.URL.RawQuery,
			"duration", time.Since(start).String(),
		)
	})
}
