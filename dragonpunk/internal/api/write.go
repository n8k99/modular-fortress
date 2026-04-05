package api

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"strconv"
	"time"

	"github.com/n8k99/modular-fortress/dragonpunk/pkg/db"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/tables"
)

// CreateRow handles POST /api/{table} — create a new row from JSON body.
func (h *Handlers) CreateRow(w http.ResponseWriter, r *http.Request) {
	table := r.PathValue("table")
	if !tables.IsValid(table) {
		writeError(w, http.StatusNotFound, "unknown table: "+table)
		return
	}

	var body map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		writeError(w, http.StatusBadRequest, "invalid JSON: "+err.Error())
		return
	}

	valid, rejected := h.columns.ValidColumns(table, body)
	if len(rejected) > 0 {
		slog.Warn("create: rejected fields", "table", table, "rejected", rejected)
	}
	if len(valid) == 0 {
		writeError(w, http.StatusBadRequest, "no valid fields provided")
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 10*time.Second)
	defer cancel()

	row, err := db.Create(ctx, h.pool, table, valid)
	if err != nil {
		slog.Error("create failed", "table", table, "error", err)
		writeError(w, http.StatusInternalServerError, "database error: "+err.Error())
		return
	}

	slog.Info("created", "table", table, "id", row["id"], "fields", len(valid))

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(row)
}

// UpdateRow handles PATCH /api/{table}/{id} — update fields on existing row.
func (h *Handlers) UpdateRow(w http.ResponseWriter, r *http.Request) {
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

	var body map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		writeError(w, http.StatusBadRequest, "invalid JSON: "+err.Error())
		return
	}

	valid, rejected := h.columns.ValidColumns(table, body)
	if len(rejected) > 0 {
		slog.Warn("update: rejected fields", "table", table, "id", id, "rejected", rejected)
	}
	if len(valid) == 0 {
		writeError(w, http.StatusBadRequest, "no valid fields provided")
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 10*time.Second)
	defer cancel()

	row, err := db.Update(ctx, h.pool, table, id, valid)
	if err != nil {
		slog.Error("update failed", "table", table, "id", id, "error", err)
		writeError(w, http.StatusInternalServerError, "database error: "+err.Error())
		return
	}
	if row == nil {
		writeError(w, http.StatusNotFound, fmt.Sprintf("%s/%d not found", table, id))
		return
	}

	slog.Info("updated", "table", table, "id", id, "fields", len(valid))

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(row)
}

// DeleteRow handles DELETE /api/{table}/{id} �� remove a row.
func (h *Handlers) DeleteRow(w http.ResponseWriter, r *http.Request) {
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

	deleted, err := db.Delete(ctx, h.pool, table, id)
	if err != nil {
		slog.Error("delete failed", "table", table, "id", id, "error", err)
		writeError(w, http.StatusInternalServerError, "database error")
		return
	}
	if !deleted {
		writeError(w, http.StatusNotFound, fmt.Sprintf("%s/%d not found", table, id))
		return
	}

	slog.Info("deleted", "table", table, "id", id)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{"deleted": true, "id": id})
}

// MoveRequest is the JSON body for POST /api/{table}/{id}/move.
type MoveRequest struct {
	TargetTable string `json:"target_table"`
	Kind        string `json:"kind"`
}

// MoveRow handles POST /api/{table}/{id}/move — move row to another table.
func (h *Handlers) MoveRow(w http.ResponseWriter, r *http.Request) {
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

	var req MoveRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		writeError(w, http.StatusBadRequest, "invalid JSON: "+err.Error())
		return
	}
	if req.TargetTable == "" {
		writeError(w, http.StatusBadRequest, "target_table is required")
		return
	}
	if !tables.IsValid(req.TargetTable) {
		writeError(w, http.StatusBadRequest, "invalid target_table: "+req.TargetTable)
		return
	}
	if req.Kind == "" {
		writeError(w, http.StatusBadRequest, "kind is required")
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 10*time.Second)
	defer cancel()

	row, err := db.Move(ctx, h.pool, table, id, req.TargetTable, req.Kind)
	if err != nil {
		slog.Error("move failed", "source", table, "id", id, "target", req.TargetTable, "error", err)
		writeError(w, http.StatusInternalServerError, "move error: "+err.Error())
		return
	}
	if row == nil {
		writeError(w, http.StatusNotFound, fmt.Sprintf("%s/%d not found", table, id))
		return
	}

	slog.Info("moved", "source", table, "target", req.TargetTable, "id", id, "new_kind", req.Kind)

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(row)
}
