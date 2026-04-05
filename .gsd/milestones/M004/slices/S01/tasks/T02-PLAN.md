---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: Scaffold Wails v3 project

Create the Wails v3 project structure. The app needs to coexist with the existing dragonpunk/ directory. Options: (1) scaffold inside dragonpunk/ as a Wails app that replaces the HTTP server main.go, or (2) create a sibling directory dragonpunk-ui/ that imports dragonpunk/internal packages. Evaluate and choose based on how Wails project structure works. The frontend should use vanilla TypeScript initially (D015 defers framework choice). Create minimal frontend with app title and placeholder content.

## Inputs

- `Wails v3 docs on project structure`
- `dragonpunk/go.mod`

## Expected Output

- `Wails project files (main.go, frontend/, wails.json or equivalent)`
- `go.mod updated with wails v3 dependency`

## Verification

wails3 build compiles without errors
