# Wails Framework Research

## Summary

Wails is confirmed as the right choice for Modular Fortress. Every capability we need is supported.

## Version Status

**Wails v3 is in alpha** (v3.0.0-alpha.68 as of Feb 2026). The API is described as "reasonably stable, and applications are running in production." Daily nightly releases. Moving toward beta. No fixed release date.

**Wails v2 is stable** (v2.12.x). Single-window only. Still maintained.

**Recommendation: Use Wails v3.** The features we need (multiple windows, system tray, procedural API) are v3-only. The alpha is actively developed with daily releases, and the API is stable enough for production use. If v3 hits a blocker, v2 is a fallback for an initial single-window prototype.

## Capability Matrix

| Need | Supported? | How |
|------|-----------|-----|
| Native macOS window | ✅ | WebKit webview, native title bar or frameless |
| Linux support | ✅ | WebKitGTK (GTK3 default, experimental GTK4) |
| Hyprland/tiling WM | ⚠️ | "Minimize/Maximize/SetSize behavior may vary" but Fullscreen works |
| Go→JS event binding | ✅ | `app.Event.Emit()` Go side, `Events.On()` JS side |
| JS→Go function calls | ✅ | Bind Go structs, all public methods auto-exposed to frontend |
| Drag and drop (files) | ✅ | `EnableFileDrop`, `OnWindowEvent(WindowFilesDropped)`, drop zone CSS |
| Drag and drop (HTML5) | ✅ | Standard HTML5 drag/drop works in webview (dragover, drop events) |
| Multiple windows | ✅ v3 only | `app.Window.New()`, each independently configurable |
| System tray | ✅ v3 only | `app.SystemTray.New()`, menu, icon, window attachment |
| Hidden/background mode | ✅ | `ActivationPolicy: Accessory`, hidden windows, tray-only mode |
| Frameless window | ✅ | `Frameless: true` option |
| Canvas/WebGL | ✅ | Full webview — Canvas, WebGL, Web Audio, all browser APIs available |
| TypeScript frontend | ✅ | Generated bindings are TypeScript-compatible |
| Single binary | ✅ | `embed.FS` for assets, single Go binary output |
| Cross-compile macOS→Linux | ⚠️ | Needs CGO cross-compilation setup, not trivial but documented |

## Two-Switch Architecture Fit

The system tray + hidden window pattern in v3 maps perfectly to D024:

**Switch 1 (headless daemon):** Not a Wails app. Separate Go binary that runs the membrane + protocol adapters + LISTEN/NOTIFY relay. No UI dependency.

**Switch 2 (UI):** Wails v3 app with system tray. Can run as tray icon (background) and open the main window on click. Or just launch as a full window. The system tray means the app can "minimize to tray" and keep running without a visible window.

This confirms the earlier split: `dragonpunk-daemon` (headless) + `dragonpunk` (Wails UI). Both connect to the same PostgreSQL.

## Scene Canvas Approach

The webview gives full access to HTML5 Canvas, WebGL, and all DOM APIs. The scene canvas options:

1. **HTML5 Canvas + JavaScript** — Direct 2D rendering. Good for positioned tokens, connections, simple scenes.
2. **PixiJS** — WebGL-accelerated 2D rendering. What Foundry VTT uses. Handles thousands of sprites, smooth panning/zooming, particle effects. Overkill initially but the right choice if scenes get complex.
3. **DOM-based** — Position elements with CSS transforms on a scrollable container. Simplest approach. Good enough for tokens + connections if we don't need smooth 60fps panning.
4. **Leaflet** — For the Orbis map scene specifically. Purpose-built for interactive maps with markers, layers, zoom.

**Recommendation:** Start with DOM-based positioning for tokens/objects + Leaflet for the Orbis map scene. Move to PixiJS/Canvas if performance demands it. The scene canvas doesn't need to be a single rendering technology — different scene types can use different renderers.

## Drag-and-Drop Details

Two kinds of drag-and-drop in play:

1. **File drops from OS** — Wails handles natively via `EnableFileDrop` + `OnWindowEvent`. Get coordinates + file paths. Useful for importing external files.

2. **Internal drag-and-drop** (ghost from roster → scene, entry from sidebar → scene) — This is standard HTML5 drag-and-drop within the webview. Wails doesn't interfere. Use `draggable`, `dragstart`, `dragover`, `drop` events. All standard browser APIs work.

The unified entity interaction pattern (D019) is purely frontend — TypeScript handles the drag events, tracks what's being dragged (ghost ID, entry slug, etc.), and on drop either positions a token on the scene or triggers an assignment action via Go binding.

## Frontend Framework

Wails v3 supports any frontend framework or vanilla. The existing mockups are vanilla HTML/CSS/JS. For the scene canvas + sidebars + floating windows complexity, a framework will help. Options:

- **Svelte** — Minimal overhead, compiles away, good for reactive UI. Wails has a Svelte template.
- **React** — Largest ecosystem, most libraries for canvas/graph rendering. Wails has a React template.
- **Vanilla TypeScript** — No framework overhead. Existing mockups prove it works. But managing floating windows + sidebars + scene state gets complex fast.

Decision deferred per D015. All three work with Wails.

## Key Risks

1. **v3 alpha stability** — Could hit breaking changes. Mitigated by: daily releases mean active maintenance, and v2 is a single-window fallback.
2. **Hyprland (tiling WM) behavior** — Window management functions "may vary." Need to test on HyprDeck. Fullscreen works, which may be sufficient for an immersive Linux experience.
3. **Cross-compilation** — Building macOS binary for Linux (and vice versa) requires CGO cross-compilation. May need separate build environments per platform.
4. **WebView performance** — Rendering a force-directed graph with thousands of nodes inside a webview. Need to benchmark. PixiJS/WebGL is the escape hatch if DOM rendering is too slow.

## Conclusion

Wails v3 alpha is the right choice. It provides everything we need: native windows, Go↔JS binding, events, drag-and-drop, system tray, multi-window, full browser API access in the webview. The alpha status is acceptable for a single-user application that won't ship publicly until v2.0.
