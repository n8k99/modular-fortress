// Dragonpunk — the Go membrane for Modular Fortress.
// Replaces all Rust user-facing I/O (dpn-core, dpn-api, noosphere).
// Connects to master_chronicle PostgreSQL via pgx.
package main

import (
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/n8k99/modular-fortress/dragonpunk/internal/api"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/config"
	"github.com/n8k99/modular-fortress/dragonpunk/pkg/db"
)

func main() {
	// Load configuration from .env
	cfg, err := config.Load()
	if err != nil {
		slog.Error("failed to load config", "error", err)
		os.Exit(1)
	}

	slog.Info("dragonpunk starting",
		"host", cfg.Host,
		"port", cfg.Port,
	)

	// Connect to PostgreSQL
	ctx := context.Background()
	pool, err := db.NewPool(ctx, cfg.DatabaseURL)
	if err != nil {
		slog.Error("failed to connect to database", "error", err)
		os.Exit(1)
	}
	defer pool.Close()
	slog.Info("database connected", "url", db.RedactURL(cfg.DatabaseURL))

	// Load column metadata for write validation
	columns, err := db.LoadColumns(ctx, pool)
	if err != nil {
		slog.Error("failed to load column metadata", "error", err)
		os.Exit(1)
	}

	// Build router
	router := api.NewRouter(pool, columns)

	addr := fmt.Sprintf("%s:%d", cfg.Host, cfg.Port)
	srv := &http.Server{
		Addr:         addr,
		Handler:      router,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	// Graceful shutdown
	done := make(chan os.Signal, 1)
	signal.Notify(done, os.Interrupt, syscall.SIGTERM)

	go func() {
		slog.Info("listening", "addr", addr)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			slog.Error("server error", "error", err)
			os.Exit(1)
		}
	}()

	<-done
	slog.Info("shutting down")

	shutdownCtx, cancel := context.WithTimeout(ctx, 5*time.Second)
	defer cancel()
	if err := srv.Shutdown(shutdownCtx); err != nil {
		slog.Error("shutdown error", "error", err)
	}
	slog.Info("dragonpunk stopped")
}
