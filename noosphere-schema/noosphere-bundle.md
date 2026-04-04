# Noosphere v2.0 Design Bundle

This document bundles three artifacts for the Noosphere v2.0 redesign:
1. **Operations Interface Spec v1** — the original mockup prompt
2. **Operations Interface Spec v2** — Foundry-weight chrome revision (primary)
3. **Nine Tables Schema** — the complete PostgreSQL schema (6 sovereign + 3 substrate)

---

# Part 1: Operations Interface Spec v1

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

---

# Part 2: Operations Interface Spec v2 — Foundry-Weight Chrome

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

---

# Part 3: The Nine Tables Schema

The v2.0 vault architecture collapses 89 tables into 9 domain tables (6 sovereign + 3 substrate) plus supporting infrastructure. Every table shares a common spine: `id, slug, kind, title, body, meta, status, created_at, updated_at`. The `kind` column is the taxonomy. The `meta` JSONB column holds domain-specific structured data. `[[wikilinks]]` in body and meta are auto-extracted to `the_links` table by triggers.

## 00_extensions.sql

```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
```

## 01_functions.sql — Shared Utilities

```sql
-- Auto-update updated_at on row modification
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Extract [[wikilinks]] from text content
-- Handles: [[Target]], [[Target|Display]], [[Target#Heading]], [[Target#Heading|Display]]
-- Also handles qualified: [[forge:Nova]], [[realms:Archonate]]
CREATE OR REPLACE FUNCTION extract_wikilinks(content TEXT)
RETURNS TABLE(target TEXT, display_text TEXT, qualifier TEXT) AS $$
BEGIN
    RETURN QUERY
    SELECT
        TRIM(m[1])::TEXT AS target,
        TRIM(NULLIF(m[3], ''))::TEXT AS display_text,
        TRIM(NULLIF(m[4], ''))::TEXT AS qualifier
    FROM regexp_matches(
        content,
        '\[\[(?:([a-z]+):)?([^\]#|]+)(?:#[^\]|]+)?(?:\|([^\]]+))?\]\]',
        'g'
    ) AS m;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Slugify a title into a URL-safe slug
CREATE OR REPLACE FUNCTION slugify(title TEXT)
RETURNS TEXT AS $$
BEGIN
    RETURN LOWER(
        REGEXP_REPLACE(
            REGEXP_REPLACE(
                TRIM(title),
                '[^a-zA-Z0-9\s-]', '', 'g'
            ),
            '\s+', '-', 'g'
        )
    );
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Extract all wikilinks from JSONB string values (recursive)
CREATE OR REPLACE FUNCTION extract_wikilinks_from_jsonb(data JSONB)
RETURNS TABLE(target TEXT, field_name TEXT) AS $$
DECLARE
    key TEXT; val JSONB; elem JSONB; str_val TEXT; match_row RECORD;
BEGIN
    IF jsonb_typeof(data) = 'object' THEN
        FOR key, val IN SELECT * FROM jsonb_each(data) LOOP
            IF jsonb_typeof(val) = 'string' THEN
                str_val := val #>> '{}';
                FOR match_row IN SELECT * FROM extract_wikilinks(str_val) LOOP
                    target := match_row.target; field_name := key; RETURN NEXT;
                END LOOP;
            ELSIF jsonb_typeof(val) IN ('object', 'array') THEN
                FOR match_row IN SELECT * FROM extract_wikilinks_from_jsonb(val) LOOP
                    target := match_row.target; field_name := key; RETURN NEXT;
                END LOOP;
            END IF;
        END LOOP;
    ELSIF jsonb_typeof(data) = 'array' THEN
        FOR elem IN SELECT * FROM jsonb_array_elements(data) LOOP
            IF jsonb_typeof(elem) = 'string' THEN
                str_val := elem #>> '{}';
                FOR match_row IN SELECT * FROM extract_wikilinks(str_val) LOOP
                    target := match_row.target; field_name := 'array_element'; RETURN NEXT;
                END LOOP;
            ELSIF jsonb_typeof(elem) IN ('object', 'array') THEN
                FOR match_row IN SELECT * FROM extract_wikilinks_from_jsonb(elem) LOOP
                    target := match_row.target; field_name := match_row.field_name; RETURN NEXT;
                END LOOP;
            END IF;
        END LOOP;
    END IF;
END;
$$ LANGUAGE plpgsql IMMUTABLE;
```

## 02_the_chronicles — Grand Epics & Master Chronicles

Narrative canon, historical ages, story arcs. **Immutable once status = 'canon'.**

```sql
CREATE TABLE the_chronicles (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- age, era, arc, event, prophecy, law, myth, cosmology, canon_doc
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'draft',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Canon immutability enforced by trigger: entries with `status='canon'` cannot have body/title modified — must set status to `'retcon'` first.

## 03_the_realms — Sovereign Realms of Orbis

Present-day worldbuilding, ghost movement, burg pipeline. Entity lifestage: Seed → Sapling → Tree.

```sql
CREATE TABLE the_realms (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- continent, state, province, burg, dungeon, wilderness,
                                     -- landmark, road, river, npc, creature, item, culture,
                                     -- religion, ghost_position
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Meta for burg: `x, y, latitude, longitude, parent_province, parent_state, population, lifestage, elevation, biome, features[]`
Meta for ghost_position: `agent_id, current_burg, trust, energy, travel_mode, biography`

## 04_the_press — Thought Police

Editorials, publishing pipeline, executive blogging. Pipeline: research → editorial (Sylvia) → hero image (Vincent) → publish.

```sql
CREATE TABLE the_press (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- editorial, blog_post, newsletter, research_brief,
                                     -- pitch, hero_image_req, published
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'draft',
    published_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 05_the_markets — Complete Success

Financial pipeline. OANDA forex, Kalshi prediction markets. Hard constraints: no stoploss = no order, max 20% bankroll, paper first.

```sql
CREATE TABLE the_markets (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- paper_trade, position, position_event, order, signal,
                                     -- alert, sentiment, probability, news_score, snapshot,
                                     -- watchlist, thesis, briefing, journal_entry, trade_log, fitness
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 06_the_music — Living Room Music

Podcast, musicology, episodes, audio analysis. Owner: L.R. Morgenstern.

```sql
CREATE TABLE the_music (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- episode, track, album, corpus_entry, analysis,
                                     -- show_note, playlist, wave_entry, venue
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 07_the_forge — Digital Sovereignty (Meta-Domain)

Ghost agents, memories, tick engine, infrastructure, vault notes, pipelines, projects, decisions, Innate templates. **Where Nathan works AND where ghosts live.**

```sql
CREATE TABLE the_forge (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL,       -- unique except for append-only kinds
    kind        TEXT NOT NULL,       -- agent, cognition_job, memory_daily, memory_entry,
                                     -- vault_note, decision, fitness_event, persona_mutation,
                                     -- metamorphosis, project, area, pipeline, pipeline_stage,
                                     -- codebase_scan, config
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    agent_id    TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Agent identity lives in meta: `full_name, role, department, reports_to, mentor, collaborators, responsibilities[], goals[], energy, tier, agent_tier, ticks_alive, drives, tool_scope[], avatar`

## 08_the_commons — @Resources (Shared Substrate)

No domain owns this. All domains draw from it. Feeds, articles, images, research surfaces, maps, avatars, contacts, events.

```sql
CREATE TABLE the_commons (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- document, feed, feed_entry, article, research_surface,
                                     -- image, avatar, map, contact, event, location, media,
                                     -- template, template_history, archive, comment
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'active',
    domain      TEXT,                -- optional affinity to a sovereign domain
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 09_the_work — @Tasks (Connective Tissue)

Every task points at a domain. Flat and universal. Ghosts perceive work from here.

```sql
CREATE TABLE the_work (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL,       -- task, goal, routine, issue, request
    title       TEXT NOT NULL,
    body        TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'open',
    domain      TEXT,
    assigned    TEXT[],              -- array of agent_ids
    priority    TEXT NOT NULL DEFAULT 'normal',
    deadline    TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Status values: open, pending, in-progress, blocked, done, completed, cancelled

## 10_the_post — @Communications

Messaging infrastructure. Ghost↔ghost via conversations. Nathan↔ghosts via email. **Append-only for messages.**

```sql
CREATE TABLE the_post (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL,       -- NOT unique (high-volume)
    kind        TEXT NOT NULL,       -- chat, report, handoff, notification, inbox, feedback
    title       TEXT NOT NULL DEFAULT '',
    body        TEXT NOT NULL,
    meta        JSONB NOT NULL DEFAULT '{}',
    status      TEXT NOT NULL DEFAULT 'sent',
    domain      TEXT,
    from_agent  TEXT NOT NULL,
    to_agent    TEXT[] NOT NULL,
    thread_id   TEXT,
    channel     TEXT NOT NULL DEFAULT 'noosphere',
    read_by     TEXT[] NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
    -- NO updated_at: messages are append-only
);
```

Chat = 1-2 sentences max (≤280 chars). Report = long-form. Handoffs never expire until responded to.

## 11_the_links — Wikilink Relationship Index

Trigger-maintained from all 9 domain tables. Every `[[wikilink]]` tracked for instant backlink queries.

```sql
CREATE TABLE the_links (
    id              BIGSERIAL PRIMARY KEY,
    source_table    TEXT NOT NULL,
    source_id       BIGINT NOT NULL,
    source_slug     TEXT NOT NULL,
    target_slug     TEXT NOT NULL,
    target_table    TEXT,           -- NULL if unresolved
    target_id       BIGINT,         -- NULL if unresolved
    link_context    TEXT NOT NULL,   -- 'body' or meta field name
    qualifier       TEXT,           -- from [[forge:Nova]]
    display_text    TEXT,           -- from [[Target|Display]]
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 12_the_index — Global Wikilink Resolution

Materialized view across all 9 tables. `[[Target]]` resolves by matching slug or title.

Resolution priority: the_forge (1) → the_work (2) → the_realms (3) → the_chronicles (4) → the_press (5) → the_markets (6) → the_music (7) → the_commons (8) → the_post (9)

## 13_the_aliases — Slug Rename Safety Net

When an entry is renamed, the old slug goes here so existing `[[wikilinks]]` don't break.

```sql
CREATE TABLE the_aliases (
    old_slug        TEXT PRIMARY KEY,
    new_slug        TEXT NOT NULL,
    source_table    TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 14_triggers — Wikilink Extraction

All 9 domain tables have INSERT/UPDATE triggers that extract `[[wikilinks]]` from body and meta JSONB, populating `the_links` for instant backlink queries. Special handling: `the_forge` skips append-only kinds (fitness_event, cognition_job). `the_post` skips chat messages (only extracts from reports and handoffs).

## 15_the_ledger — Invisible Infrastructure

Append-only tick telemetry. Managed by T.A.S.K.S. Not a domain card — users never see this.

```sql
CREATE TABLE the_ledger (
    id          BIGSERIAL PRIMARY KEY,
    kind        TEXT NOT NULL,       -- tick_log, tick_report, rollup_daily, rollup_monthly
    agent_id    TEXT,
    meta        JSONB NOT NULL DEFAULT '{}',
    body        TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
    -- NO slug, NO title, NO status, NO updated_at. Pure telemetry.
);
```
