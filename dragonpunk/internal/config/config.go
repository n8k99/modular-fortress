// Package config loads Dragonpunk configuration from .env and environment variables.
package config

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"

	"github.com/joho/godotenv"
)

const (
	DefaultDatabaseURL = "postgresql://nebulab_user:nebulab_dev_password@localhost:5432/master_chronicle"
	DefaultHost        = "0.0.0.0"
	DefaultPort        = 8888
)

// Config holds all Dragonpunk runtime configuration.
type Config struct {
	DatabaseURL string
	Host        string
	Port        int
}

// Load reads configuration from .env file (at repo root) then environment variables.
// Environment variables override .env values.
func Load() (*Config, error) {
	// Try to load .env from repo root (one level up from dragonpunk/)
	// Silently ignore if not found — env vars may already be set.
	envPaths := []string{
		".env",
		filepath.Join("..", ".env"),
	}
	for _, p := range envPaths {
		_ = godotenv.Load(p)
	}

	cfg := &Config{
		DatabaseURL: getEnvOr("DATABASE_URL", DefaultDatabaseURL),
		Host:        getEnvOr("HOST", DefaultHost),
	}

	portStr := getEnvOr("PORT", strconv.Itoa(DefaultPort))
	port, err := strconv.Atoi(portStr)
	if err != nil {
		return nil, fmt.Errorf("invalid PORT %q: %w", portStr, err)
	}
	cfg.Port = port

	if err := cfg.Validate(); err != nil {
		return nil, err
	}
	return cfg, nil
}

// Validate checks that required configuration is present and sane.
func (c *Config) Validate() error {
	if c.DatabaseURL == "" {
		return fmt.Errorf("DATABASE_URL is required")
	}
	if c.Port < 1 || c.Port > 65535 {
		return fmt.Errorf("PORT must be 1-65535, got %d", c.Port)
	}
	return nil
}

func getEnvOr(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}
