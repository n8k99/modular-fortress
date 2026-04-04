// Package tables defines the Nine Tables whitelist and shared column metadata.
package tables

// ValidTables is the whitelist of all Nine Tables that Dragonpunk can serve.
// Table names are safe for SQL interpolation because they come from this hardcoded set.
var ValidTables = map[string]bool{
	"identity":       true,
	"temporal":        true,
	"the_work":        true,
	"the_commons":     true,
	"the_chronicles":  true,
	"the_realms":      true,
	"the_music":       true,
	"the_post":        true,
	"the_press":       true,
	"the_markets":     true,
	"the_links":       true,
	"the_index":       true,
	"the_aliases":     true,
	"the_ledger":      true,
}

// ListColumns are the core columns returned by list endpoints.
// These exist on most Nine Tables (except infrastructure tables which vary).
var ListColumns = []string{"id", "slug", "kind", "title", "status", "created_at", "updated_at"}

// IsValid returns true if the table name is in the Nine Tables whitelist.
func IsValid(table string) bool {
	return ValidTables[table]
}

// Names returns all valid table names as a slice.
func Names() []string {
	names := make([]string, 0, len(ValidTables))
	for name := range ValidTables {
		names = append(names, name)
	}
	return names
}
