# S01: Wails v3 Install + Hello Window

**Goal:** Install Wails v3 CLI, scaffold a Wails project that integrates with the existing dragonpunk/ Go code, and get a native macOS window building and running.
**Demo:** After this: A native macOS window opens with 'Dragonpunk — Modular Fortress' title and placeholder content

## Tasks
- [x] **T01: Installed Wails v3.0.0-alpha.74 CLI — wails3 doctor confirms system ready for development** — Install the Wails v3 alpha CLI tool. Verify it works with Go 1.25 on macOS ARM64. Run wails3 doctor to confirm all dependencies (Go, Node, npm, WebKit) are satisfied.
  - Estimate: 10min
  - Verify: wails3 doctor reports all checks passed
- [x] **T02: Scaffolded Wails v3 project as dragonpunk-app/ with vanilla TypeScript, Dragonpunk branding, and vault-map color palette** — Create the Wails v3 project structure. The app needs to coexist with the existing dragonpunk/ directory. Options: (1) scaffold inside dragonpunk/ as a Wails app that replaces the HTTP server main.go, or (2) create a sibling directory dragonpunk-ui/ that imports dragonpunk/internal packages. Evaluate and choose based on how Wails project structure works. The frontend should use vanilla TypeScript initially (D015 defers framework choice). Create minimal frontend with app title and placeholder content.
  - Estimate: 30min
  - Files: dragonpunk/, new Wails project files
  - Verify: wails3 build compiles without errors
- [x] **T03: Dragonpunk Wails app launches — native macOS window opens at 1280x800 with void background** — Build the Wails app with wails3 build. Run the resulting binary. Verify a native macOS window opens with the correct title. Take note of binary size, startup time, and any warnings. If build fails, debug and fix.
  - Estimate: 15min
  - Files: build output binary
  - Verify: Binary launches and a native macOS window appears with title containing 'Dragonpunk'
