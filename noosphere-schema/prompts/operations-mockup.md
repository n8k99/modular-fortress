# Prompt: Noosphere Operations Interface Mockup

Create a single-file HTML/CSS/JS mockup of "The Noosphere" — a Foundry VTT-inspired operations interface for managing an AI agent organization. This is the command table where a user runs their sovereign AI platform.

## Design System

Use this exact palette and typography (from the existing vault architecture page):

```css
--void: #08080D;
--smoke: #13131A;
--panel: #0F0F16;
--ember: #C8860A;
--ember-mid: #8B5E00;
--ember-dim: rgba(200,134,10,0.15);
--cyan: #00B8C0;
--text: #D8D4CC;
--text-dim: #777770;
--text-muted: #44443F;
--border: rgba(200,134,10,0.18);
--border-dim: rgba(200,134,10,0.07);
```

Fonts: `'IBM Plex Mono'` for data/UI, `'Cormorant Garamond'` for headings. Dark, cinematic, austere. No rounded corners beyond 3px. No bright colors. Ember accents only. This should feel like a mission control room, not a SaaS dashboard.

## Layout Structure

The interface is ONE full-viewport page. No scrolling on the main frame. No URL bar navigation. It should feel like a native application, not a website.

### Left Sidebar (Scene Selector) — 220px wide, always visible
A vertical list of scene buttons. Each scene changes what appears in center stage. Scenes:

- **Forge Workshop** — Nathan's desk. Daily notes, system health, inbox summary. (default scene)
- **Pipeline Workbench** — n8n-style flowchart view of active pipelines
- **Orbis Map** — placeholder world map with ghost token positions
- **Nova's Bridge** — example ghost command view (one ghost's full state)
- **Domain View** — raw table browser for any of the 9 domains

The active scene is highlighted with an ember left-border. Scene icons are simple text glyphs or unicode, not images.

### Right Sidebar (Context Panel) — 280px wide, collapsible
Shows contextual details for whatever is selected in center stage. When nothing is selected, shows system overview:
- Ghost count (active/idle/dormant)
- Current tick number
- LLM budget used
- Last tick timestamp

When a ghost is selected: their name, role, energy bar, tier, department, reports_to, current task, pipeline position, recent 3 actions.

When a task is selected: title, status, assigned ghosts, domain, blocked_by, deadline.

### Bottom Bar (Chat) — 180px tall, always visible
Persistent chat interface. Shows ghost messages in real time (mock data). Left side has channel tabs:
- **All** — all ghost chatter
- **T.A.S.K.S.** — direct chat with COO
- **Engineering** — pipeline channel

Each message shows: ghost name (ember colored), timestamp (muted), 1-2 sentence message (text color). There's an input field at the bottom to type messages.

### Top Bar — 40px tall
Left: Noosphere name/logo ("(.)(.) ghosts-in-the-noosphere" in Cormorant Garamond)
Center: Five tabs: **Scenes** | **Journal** | **Email** | **Chat** | **Reader** (Scenes is active by default, highlighted with ember underline)
Right: Gear icon (returns to dashboard), bell icon (notifications), user avatar placeholder

### Center Stage — fills remaining space
This is where the active scene renders. For the mockup, build out the **Forge Workshop** as the default scene:

#### Forge Workshop Scene
A 3-column layout within center stage:

**Left column (Daily Notes):**
- Today's date as header
- A mock daily note with markdown-style content
- Previous days collapsed below

**Center column (Inbox):**
- "Pending your attention" header
- 3-4 mock items: a ghost report waiting for review, a task needing approval, a decision to ratify, a starred Reader article
- Each item shows: source ghost avatar (colored circle with initial), title, time ago, domain tag

**Right column (System Health):**
- Tick engine status (green dot + "Ticking — 30s interval")
- Ghost ecology mini-chart (horizontal stacked bar: active/idle/dormant counts)
- Pipeline status list (Engineering: 3 tasks flowing, Complete Success: idle, etc.)
- Recent notable events (3-4 one-liners from tick reports)

## Mock Data

Populate with realistic data from this ghost roster:

| Ghost | Role | Department | Tier |
|-------|------|------------|------|
| Nova (T.A.S.K.S.) | COO | Operations | prime |
| Eliana | CTO | Engineering | working |
| Kathryn | CSO | Strategy | working |
| Sylvia | Content Chief | Press | working |
| Vincent | Creative Director | Creative | base |
| Samir | Senior Engineer | Engineering | working |
| Sophie Lee | Researcher | Research | base |
| Ethan Ng | Trader | Markets | working |

Mock chat messages should be 1-2 sentences max. Examples:
- "Nova: Morning briefing posted. 3 items need your attention."
- "Samir: Pipeline stage 4 complete for task-api-refactor. Handing off to Casey."
- "Sophie Lee: Starred 2 articles from Winer's blog. Tagged for Research Feed."

## Interactive Elements (JS)

1. Clicking scene buttons in left sidebar changes a "current-scene" class (just show/hide divs is fine)
2. Clicking the top tabs changes between Scenes/Journal/Email/Chat/Reader views (can be placeholder divs for non-Scenes tabs)
3. The gear icon shows a tooltip "Back to Dashboard"
4. Chat input field — pressing Enter adds the message to the chat log with "You:" prefix
5. Right sidebar items are clickable — clicking a ghost name in chat populates the right sidebar with their details
6. Collapsible right sidebar via a toggle arrow

## What NOT to include
- No React, no build tools, no npm. Single HTML file with inline CSS and vanilla JS.
- No images or external assets beyond Google Fonts.
- No loading spinners or skeleton screens.
- No mobile responsiveness — this is a desktop command table.
- No login screen — we're already in.
- No actual API calls — all data is hardcoded mock data in JS objects.

## Spirit

This should feel like sitting in a command center. Quiet, dark, information-dense but not cluttered. The ghosts are working around you — you see their activity in the chat, their status in the sidebar, their work flowing through pipelines. You are the operator. This is your station.
