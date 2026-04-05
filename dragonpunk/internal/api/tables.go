package api

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"strconv"
	"time"

	"github.com/n8k99/modular-fortress/dragonpunk/pkg/db"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/tables"
)

const (
	defaultLimit = 50
	maxLimit     = 200
)

// ListTable handles GET /api/{table} — paginated list with kind filter and text search.
func (h *Handlers) ListTable(w http.ResponseWriter, r *http.Request) {
	table := r.PathValue("table")
	if !tables.IsValid(table) {
		writeError(w, http.StatusNotFound, "unknown table: "+table)
		return
	}

	// Parse query params
	limit := parseIntParam(r, "limit", defaultLimit)
	if limit < 1 {
		limit = 1
	}
	if limit > maxLimit {
		limit = maxLimit
	}
	offset := parseIntParam(r, "offset", 0)
	if offset < 0 {
		offset = 0
	}
	kind := r.URL.Query().Get("kind")
	query := r.URL.Query().Get("q")

	ctx, cancel := context.WithTimeout(r.Context(), 10*time.Second)
	defer cancel()

	result, err := db.List(ctx, h.pool, db.ListParams{
		Table:  table,
		Kind:   kind,
		Query:  query,
		Limit:  limit,
		Offset: offset,
	})
	if err != nil {
		slog.Error("list table failed", "table", table, "error", err)
		writeError(w, http.StatusInternalServerError, "database error")
		return
	}

	slog.Info("list", "table", table, "kind", kind, "q", query, "total", result.Total, "returned", len(result.Rows))

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(result)
}

// GetRow handles GET /api/{table}/{id} — full row by primary key.
func (h *Handlers) GetRow(w http.ResponseWriter, r *http.Request) {
	table := r.PathValue("table")
	if !tables.IsValid(table) {
		writeError(w, http.StatusNotFound, "unknown table: "+table)
		return
	}

	idStr := r.PathValue("id")
	id, err := strconv.ParseInt(idStr, 10, 64)
	if err != nil {
		writeError(w, http.StatusBadRequest, "invalid id: "+idStr)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()

	row, err := db.GetByID(ctx, h.pool, table, id)
	if err != nil {
		slog.Error("get row failed", "table", table, "id", id, "error", err)
		writeError(w, http.StatusInternalServerError, "database error")
		return
	}
	if row == nil {
		writeError(w, http.StatusNotFound, table+"/"+idStr+" not found")
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(row)
}

// GetBySlug handles GET /api/{table}/slug/{slug} — full row by slug.
func (h *Handlers) GetBySlug(w http.ResponseWriter, r *http.Request) {
	table := r.PathValue("table")
	if !tables.IsValid(table) {
		writeError(w, http.StatusNotFound, "unknown table: "+table)
		return
	}

	slug := r.PathValue("slug")
	if slug == "" {
		writeError(w, http.StatusBadRequest, "slug is required")
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()

	row, err := db.GetBySlug(ctx, h.pool, table, slug)
	if err != nil {
		slog.Error("get by slug failed", "table", table, "slug", slug, "error", err)
		writeError(w, http.StatusInternalServerError, "database error")
		return
	}
	if row == nil {
		writeError(w, http.StatusNotFound, table+"/slug/"+slug+" not found")
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(row)
}

// ListKinds handles GET /api/{table}/kinds — distinct kind values with counts.
func (h *Handlers) ListKinds(w http.ResponseWriter, r *http.Request) {
	table := r.PathValue("table")
	if !tables.IsValid(table) {
		writeError(w, http.StatusNotFound, "unknown table: "+table)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()

	kinds, err := db.ListKinds(ctx, h.pool, table)
	if err != nil {
		slog.Error("list kinds failed", "table", table, "error", err)
		writeError(w, http.StatusInternalServerError, "database error")
		return
	}
	if kinds == nil {
		// Table doesn't have a kind column
		writeError(w, http.StatusNotFound, table+" has no kind column")
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(kinds)
}

func parseIntParam(r *http.Request, name string, defaultVal int) int {
	s := r.URL.Query().Get(name)
	if s == "" {
		return defaultVal
	}
	v, err := strconv.Atoi(s)
	if err != nil {
		return defaultVal
	}
	return v
}
