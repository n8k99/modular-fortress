# Prompt: Noosphere Operations Interface v2 — Foundry-Weight Chrome

Create a single-file HTML/CSS/JS mockup of "The Noosphere" — a Foundry VTT-inspired operations interface for managing an AI agent organization. The CRITICAL design principle: **the scene IS the application. All chrome is peripheral.**

Reference: Look at Foundry VTT's actual interface. The left sidebar is a 36px strip of icons. The right sidebar is collapsible and overlays the scene, it does not push it. The top bar is barely there. The scene fills 95%+ of the viewport.

## Design System

```css
--void: #08080D;
--smoke: #13131A;
--panel: #0F0F16;
--ember: #C8860A;
--ember-mid: #8B5E00;
--ember-dim: rgba(200,134,10,0.12);
--cyan: #00B8C0;
--text: #D8D4CC;
--text-dim: #777770;
--text-muted: #44443F;
--border: rgba(200,134,10,0.18);
--border-dim: rgba(200,134,10,0.07);
```

Fonts: `'IBM Plex Mono'` for data/UI, `'Cormorant Garamond'` for headings. Dark, cinematic, austere. This should feel like mission control — not a SaaS dashboard.

## Layout — Chrome Disappears, Scene Dominates

### Left Icon Strip — 40px wide, semi-transparent
A vertical strip of small icon buttons, ICONS ONLY, no text labels. Background is semi-transparent (rgba over the scene). Each icon is ~28px, vertically stacked with 4px gaps. Hover shows a tooltip with the scene/function name. Active scene has an ember left-border accent (2px).

Icons (use unicode glyphs, no images):
- ⚒ Forge Workshop (your desk — default scene)
- ⟶ Pipeline Workbench
- ◎ Orbis Map
- ◈ Ghost Bridges (opens a sub-selector for individual ghosts)
- ⊞ Domain View
- — separator line —
- 📓 Journal
- ✉ Email
- 💬 Chat (full view)
- ☰ Reader
- — separator —
- ⚙ Settings / Dashboard (bottom of strip)

The strip should feel like it's floating over the scene, not framing it. Use a subtle backdrop-blur or slight darkening.

### Right Sidebar — COLLAPSED by default, OVERLAYS scene when open
Width: 300px when open, 0px when collapsed. It slides OVER the scene content, not pushing it. Has a slight backdrop-blur and shadow to float over the scene. A thin vertical grab handle (4px, ember-dim) on the left edge shows it can be opened.

When open, shows context for whatever is selected:
- Default: system overview (ghost counts, tick, LLM budget, infra health)
- Ghost selected: name (Cormorant, large), role, trust/energy bars, domain, pipeline, location, current task, recent 3 actions
- Task selected: title, status, assigned, domain, pipeline, blocked_by, description
- Entry selected: slug, kind, domain, meta preview, outgoing [[wikilinks]]

Close button (X or ◀) in the top-right of the sidebar.

### Top Bar — 32px, minimal, semi-transparent
Left: logo `(.)(.)` only — no full text unless hovered. The logotype appears on hover.
Center: Current scene name in small muted text (e.g., "Forge Workshop")
Right: notification bell (just a dot indicator if unread), user initial avatar

The top bar should be barely noticeable. Semi-transparent background, blending into the scene.

### Bottom Chat — COLLAPSED by default, expands on focus
When collapsed: a single thin input bar (32px tall) at the bottom of the viewport. Semi-transparent. Shows placeholder "Message the noosphere..." and a ghost selector dropdown (who am I talking to: All / T.A.S.K.S. / Engineering / specific ghost).

When focused/expanded: grows to ~180px. Shows recent messages above the input. Messages show: ghost name (colored), time (muted), text. 1-2 sentence max per message. Ghost names are clickable (opens their card in right sidebar).

Pressing Escape or clicking outside collapses it back to the input bar.

### Center Stage — FILLS EVERYTHING
The scene content fills the entire viewport, edge to edge, underneath the semi-transparent chrome overlays. The scene should feel immersive. When all chrome is collapsed, the scene is literally fullscreen with just a thin icon strip on the left.

## Scenes to Build

### 1. Forge Workshop (default)
Three-panel layout within center stage. Each panel has a semi-transparent dark background (not opaque — the void shows through). Panels:

**Left panel — Daily Notes**
- Today's date (Cormorant, 28px)
- Note content (11px, dim, markdown-style)
- Previous days as collapsed rows

**Center panel — Inbox**
- "Pending your attention" header
- 4 mock items with ghost avatar (colored circle + initial), title, time, domain tag
- Items are clickable (loads ghost/task in right sidebar)

**Right panel — System Health**
- Tick engine status with green dot
- Ghost ecology bar (active/idle/dormant as horizontal segments)
- Pipeline status rows
- Recent events (ghost name + action + time)

### 2. Pipeline Workbench
Full center stage. Three pipelines rendered as horizontal node chains:
- Engineering: Eliana → Samir (active, ember border) → Isaac → Casey → Devin → Sanjay → Danielle → Morgan
- Complete Success: Ethan (active) → Tobias → Kathryn → Lucas → Kathryn → JMax → Log only
- Modular Fortress: JMax → Lucas → Lily → Felix → Elise → Eliana

Active nodes glow with ember. Clicking a node opens that ghost in right sidebar.

### 3. Orbis Map (placeholder)
Dark panel filling center stage. A large placeholder rectangle with:
- "ORBIS" in Cormorant at the top
- 6-8 colored dots (ghost tokens) scattered with position labels
- Province labels in muted text
- "Drunkard's Walk · n ghosts active" at bottom
- This should feel like looking at a satellite view of a dark world

### 4. Nova's Bridge
Full ghost profile for Nova/T.A.S.K.S. Two-column grid:
- Identity card (role, domain, reports_to, starting_point, location, tier)
- Vitals card (trust bar, energy bar, tick #, status, movement mode)
- Full-width Recent Actions card (timestamped action log)

Ghost name in Cormorant 32px, cyan colored. The whole scene should feel like you've walked onto their bridge.

### 5. Domain View
3x3 grid of domain cards, each with:
- Domain name (Cormorant, domain-colored)
- Subtitle
- Entry count
- Health indicator dot

## Mock Data

Use this ghost roster with consistent colors:

| Ghost | Color | Role |
|-------|-------|------|
| Nova | #00B8C0 (cyan) | T.A.S.K.S. · COO |
| Eliana | #2E9E78 | CTO · Engineering |
| Kathryn | #B87015 | CSO · Markets |
| Sylvia | #C45A1A | Content Chief · Press |
| Vincent | #7C6FD4 | Creative Director |
| Samir | #44A870 | Senior Engineer |
| Sophie Lee | #8E4BB0 | Researcher |
| Ethan Ng | #B87015 | Trader · Markets |

Chat messages should be 1-2 sentences max:
- "Nova: Morning briefing posted. 3 items need your attention."
- "Samir: Stage 4 complete for task-api-refactor. Handing off to Casey."
- "Sophie Lee: Starred 2 articles — HJB equations and ClawTeam. Tagged for Research Feed."
- "Ethan: EUR/USD showing confluence. Routing to Tobias."
- "Kathryn: London open in 52 minutes. Surf report queued."

## Interactive Elements (Vanilla JS)

1. Left icon strip: click switches scenes. Active icon gets ember left border.
2. Right sidebar: collapsed by default. Clicking an inbox item, ghost name in chat, or pipeline node opens it with context. X button closes it. It OVERLAYS, does not push.
3. Bottom chat: collapsed to input bar. Focus expands to show messages. Escape collapses. Enter sends message with "You:" prefix. Ghost selector dropdown next to input.
4. All chrome should have smooth transitions (0.2s ease).
5. Pipeline nodes are clickable → opens ghost in right sidebar.

## What NOT to include
- No React, no build tools. Single HTML file, inline CSS, vanilla JS.
- No images or external assets beyond Google Fonts.
- No mobile responsiveness — desktop only.
- No login screen.
- No actual API calls — hardcoded mock data.

## Spirit

When all chrome is collapsed, the user should feel like they're INSIDE the scene. The Forge Workshop feels like sitting at a desk. The Orbis Map feels like looking at a world. Nova's Bridge feels like standing on a command deck. The chrome appears when summoned and disappears when dismissed. The scene is not a widget in a dashboard — it IS the application.

Think Foundry VTT. Think Bloomberg Terminal. Think EVE Online's station view. Think the bridge of the Enterprise. Ambient, immersive, information-dense when you look for it, quiet when you don't.
