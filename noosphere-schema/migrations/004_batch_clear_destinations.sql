-- Migration 004: Batch migration of all clear-destination legacy tables
-- ~14,500 rows across 38 tables → Nine Tables
--
-- Run: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/004_batch_clear_destinations.sql
-- 
-- Each section is idempotent (skips existing slugs).

BEGIN;

-- ============================================================
-- STEP 0: Widen kind constraints
-- ============================================================

ALTER TABLE identity DROP CONSTRAINT IF EXISTS identity_kind_check;
ALTER TABLE identity ADD CONSTRAINT identity_kind_check
  CHECK (kind = ANY (ARRAY['user','agent','contact','team','department','linked_account']));

ALTER TABLE the_work DROP CONSTRAINT IF EXISTS the_work_kind_check;
ALTER TABLE the_work ADD CONSTRAINT the_work_kind_check
  CHECK (kind = ANY (ARRAY['project','phase','goal','task','decision','routine','issue','extracted_task','project_history']));

-- ============================================================
-- STEP 1: the_work  (goals, projects, extracted_tasks, decisions, routines, projects_history)
-- ============================================================

-- goals (44 rows)
INSERT INTO the_work (slug, kind, title, body, type, status, parent_project, parent_goal, owner, priority, tags, created_at, updated_at)
SELECT
    'goal-' || g.id,
    'goal',
    g.title,
    COALESCE(g.body, g.content),
    g.type,
    g.status,
    g.project,
    NULL,
    g.owner,
    g.priority,
    g.tags,
    g.created_at,
    g.updated_at
FROM goals g
WHERE NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = 'goal-' || g.id);

-- projects (16 rows)
INSERT INTO the_work (slug, kind, title, body, status, owner, description, tags, created_at, updated_at)
SELECT
    COALESCE(p.slug, 'project-' || p.id),
    'project',
    p.name,
    p.description,
    p.status,
    p.owner,
    p.current_context,
    NULL,
    p.created_at,
    p.updated_at
FROM projects p
WHERE NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = COALESCE(p.slug, 'project-' || p.id));

-- extracted_tasks (1,141 rows)
INSERT INTO the_work (slug, kind, title, body, status, priority, due_date, assigned_to, department, tags, created_at, updated_at)
SELECT
    'etask-' || et.id,
    'extracted_task',
    et.title,
    et.raw_text,
    COALESCE(et.status, 'active'),
    et.priority,
    et.due_date,
    et.assignee,
    et.department,
    et.tags,
    et.created_at,
    et.created_at
FROM extracted_tasks et
WHERE NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = 'etask-' || et.id);

-- decisions (1 row)
INSERT INTO the_work (slug, kind, title, body, status, owner, decision_date, decision_outcome, department, created_at, updated_at)
SELECT
    'decision-' || d.id,
    'decision',
    d.decision,
    d.rationale,
    'active',
    d.owner,
    d.date,
    NULL,
    d.department,
    d.created_at,
    d.created_at
FROM decisions d
WHERE NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = 'decision-' || d.id);

-- routines (11 rows)
INSERT INTO the_work (slug, kind, title, body, status, assigned_to, description, created_at, updated_at)
SELECT
    'routine-' || r.id,
    'routine',
    r.name,
    r.description,
    COALESCE(r.status, 'active'),
    r.owner_agent,
    r.schedule,
    r.created_at,
    r.updated_at
FROM routines r
WHERE NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = 'routine-' || r.id);

-- projects_history (20 rows)
INSERT INTO the_work (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'projhist-' || ph.history_id,
    'project_history',
    'History: ' || COALESCE(ph.name, 'unknown'),
    ph.description,
    COALESCE(ph.status, 'archived'),
    COALESCE(ph.archived_at, ph.original_created_at),
    COALESCE(ph.archived_at, ph.original_updated_at)
FROM projects_history ph
WHERE NOT EXISTS (SELECT 1 FROM the_work w WHERE w.slug = 'projhist-' || ph.history_id);

DO $$ BEGIN RAISE NOTICE 'the_work migration complete'; END $$;

-- ============================================================
-- STEP 2: the_post  (agent_requests, noosphere_feedback, noosphere_replies, comments)
-- ============================================================

-- agent_requests (1,247 rows)
INSERT INTO the_post (slug, kind, title, body, status, from_identity, to_identity, subject, protocol, type, created_at, updated_at)
SELECT
    'areq-' || ar.id,
    'agent_request',
    ar.subject,
    ar.context,
    COALESCE(ar.status, 'pending'),
    ar.from_agent,
    ar.to_agent,
    ar.subject,
    'internal',
    ar.request_type,
    ar.created_at,
    ar.resolved_at
FROM agent_requests ar
WHERE NOT EXISTS (SELECT 1 FROM the_post p WHERE p.slug = 'areq-' || ar.id);

-- noosphere_feedback (108 rows)
INSERT INTO the_post (slug, kind, title, body, status, conversation_id, protocol, created_at, updated_at)
SELECT
    'feedback-' || nf.id,
    'feedback',
    'Feedback on conversation ' || nf.conversation_id,
    nf.feedback,
    'active',
    nf.conversation_id::text,
    'noosphere',
    nf.created_at,
    nf.created_at
FROM noosphere_feedback nf
WHERE NOT EXISTS (SELECT 1 FROM the_post p WHERE p.slug = 'feedback-' || nf.id);

-- noosphere_replies (17 rows)
INSERT INTO the_post (slug, kind, title, body, status, conversation_id, protocol, created_at, updated_at)
SELECT
    'nreply-' || nr.id,
    'reply',
    'Reply on conversation ' || nr.conversation_id,
    nr.comment,
    'active',
    nr.conversation_id::text,
    'noosphere',
    nr.created_at,
    nr.created_at
FROM noosphere_replies nr
WHERE NOT EXISTS (SELECT 1 FROM the_post p WHERE p.slug = 'nreply-' || nr.id);

-- comments (2 rows)
INSERT INTO the_post (slug, kind, title, body, status, from_identity, protocol, created_at, updated_at)
SELECT
    'comment-' || c.id,
    'comment',
    'Comment by ' || COALESCE(c.author, 'unknown'),
    c.content,
    'active',
    c.author,
    COALESCE(c.source, 'internal'),
    COALESCE(c.collected_at, now()),
    COALESCE(c.collected_at, now())
FROM comments c
WHERE NOT EXISTS (SELECT 1 FROM the_post p WHERE p.slug = 'comment-' || c.id);

DO $$ BEGIN RAISE NOTICE 'the_post migration complete'; END $$;

-- ============================================================
-- STEP 3: the_markets  (market_signals, market_data, forex_*, positions, orders, trades, sentiment)
-- ============================================================

-- market_signals (1,482 rows)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'msig-' || ms.id,
    'signal',
    ms.symbol || ' ' || ms.signal,
    'Price: ' || ms.price || ' Change24h: ' || ms.change_24h || ' Volume: ' || ms.volume_24h || ' Momentum: ' || ms.momentum_score,
    'active',
    ms.scanned_at,
    ms.scanned_at
FROM market_signals ms
WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'msig-' || ms.id);

-- market_data (124 rows)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'mdata-' || md.id,
    'ohlcv',
    md.symbol || ' ' || md.bar_interval || ' ' || md.ts::text,
    'O:' || md.open || ' H:' || md.high || ' L:' || md.low || ' C:' || md.close || ' V:' || md.volume,
    'active',
    md.ingested_at,
    md.ingested_at
FROM market_data md
WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'mdata-' || md.id);

-- forex_news_scores (594 rows)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'fnews-' || fns.id,
    'news_score',
    fns.headline,
    'Source: ' || COALESCE(fns.source, '') || ' RiskOff: ' || fns.risk_off_score || ' RiskOn: ' || fns.risk_on_score || ' Net: ' || fns.net_score,
    CASE WHEN fns.blackout THEN 'blackout' ELSE 'active' END,
    COALESCE(fns.pub_date, fns.fetched_at),
    fns.fetched_at
FROM forex_news_scores fns
WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'fnews-' || fns.id);

-- forex_paper_trades (39 rows)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'fpt-' || fpt.id,
    'paper_trade',
    fpt.pair || ' ' || fpt.direction || ' ' || fpt.units || 'u',
    'Open: ' || fpt.open_price || ' Close: ' || COALESCE(fpt.close_price::text, 'open') || ' PnL: ' || COALESCE(fpt.pnl_pips::text, '0') || 'pips / $' || COALESCE(fpt.pnl_usd::text, '0'),
    COALESCE(fpt.status, 'closed'),
    fpt.open_at,
    COALESCE(fpt.close_at, fpt.open_at)
FROM forex_paper_trades fpt
WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'fpt-' || fpt.id);

-- forex_fitness_snapshots (38 rows)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'ffit-' || ffs.id,
    'fitness_snapshot',
    'Fitness Snapshot #' || ffs.id,
    ffs::text,
    'active',
    now(),
    now()
FROM forex_fitness_snapshots ffs
WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'ffit-' || ffs.id);

-- positions (3 rows)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'pos-' || po.id,
    'position',
    po.symbol || ' ' || po.side || ' ' || po.quantity,
    'AvgCost: ' || po.avg_cost || ' Thesis: ' || COALESCE(po.thesis, ''),
    COALESCE(po.status, 'open'),
    po.opened_at,
    po.last_updated
FROM positions po
WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'pos-' || po.id);

-- position_events (3)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT 'posevt-' || pe.id, 'position_event', pe.event_type || ' #' || pe.position_id, COALESCE(pe.notes,'') || ' qty=' || COALESCE(pe.quantity::text,'') || ' price=' || COALESCE(pe.price::text,''), 'active', pe.created_at, pe.created_at
FROM position_events pe WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'posevt-' || pe.id);

-- orders (2)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT 'order-' || o.id, 'order', o.order_type || ' ' || o.shares || ' shares', 'market_id=' || COALESCE(o.market_id::text,'') || ' price=' || COALESCE(o.price::text,''), CASE WHEN o.executed THEN 'filled' ELSE 'pending' END, now(), now()
FROM orders o WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'order-' || o.id);

-- trade_log (2)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT 'trade-' || tl.id, 'trade', COALESCE(tl.market_title, 'Trade #' || tl.id) || ' ' || tl.side, 'shares=' || tl.shares || ' price=' || tl.price_cents || ' pnl=' || COALESCE(tl.pnl_cents::text,'0') || ' thesis=' || COALESCE(tl.thesis,''), COALESCE(tl.status, 'closed'), tl.created_at, COALESCE(tl.filled_at, tl.created_at)
FROM trade_log tl WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'trade-' || tl.id);

-- trade_journal_entries (2)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT 'tjournal-' || tj.id, 'journal_entry', tj.title, tj.body || E'\nLessons: ' || COALESCE(tj.lessons,'') || E'\nMood: ' || COALESCE(tj.mood,''), 'active', tj.created_at, tj.updated_at
FROM trade_journal_entries tj WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'tjournal-' || tj.id);

-- sentiment_scores (2)
INSERT INTO the_markets (slug, kind, title, body, status, created_at, updated_at)
SELECT 'sentiment-' || ss.id, 'sentiment', COALESCE(ss.label, 'Sentiment') || ' #' || ss.id, 'compound=' || ss.compound || ' pos=' || ss.positive || ' neg=' || ss.negative || ' neu=' || ss.neutral || E'\n' || COALESCE(ss.text_snippet,''), 'active', ss.analyzed_at, ss.analyzed_at
FROM sentiment_scores ss WHERE NOT EXISTS (SELECT 1 FROM the_markets m WHERE m.slug = 'sentiment-' || ss.id);

DO $$ BEGIN RAISE NOTICE 'the_markets migration complete'; END $$;

-- ============================================================
-- STEP 4: the_music  (episodes, music_files, spectral, chroma, audio)
-- ============================================================

-- episodes (4)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT 'episode-' || e.id, 'episode', e.title, e.description, 'active', e.created_at, e.created_at
FROM episodes e WHERE NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'episode-' || e.id);

-- music_files (1)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT 'mfile-' || mf.id, 'audio_file', mf.file_path, 'tempo:' || COALESCE(mf.tempo::text,'?') || ' key:' || COALESCE(mf.key::text,'?'), 'active', mf.created_at, mf.created_at
FROM music_files mf WHERE NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'mfile-' || mf.id);

-- music_spectral_features (17)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT 'spectral-' || ms.id, 'spectral_analysis', 'Spectral #' || ms.id, 'track_id=' || ms.id, 'active', now(), now()
FROM music_spectral_features ms WHERE NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'spectral-' || ms.id);

-- music_chroma_features (12)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT 'chroma-' || mc.id, 'chroma_analysis', 'Chroma #' || mc.id, 'track_id=' || mc.id, 'active', now(), now()
FROM music_chroma_features mc WHERE NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'chroma-' || mc.id);

-- audio_files (1)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT 'audiofile-' || af.id, 'audio_file', af.file_path, 'dur:' || af.duration || ' tempo:' || COALESCE(af.tempo::text,'?') || ' key:' || COALESCE(af.key::text,'?'), 'active', af.created_at, af.created_at
FROM audio_files af WHERE NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'audiofile-' || af.id);

-- audio_features (1)
INSERT INTO the_music (slug, kind, title, body, status, created_at, updated_at)
SELECT 'audiofeat-' || afeat.id, 'audio_feature', 'Audio Feature #' || afeat.id, 'audio_file_id=' || afeat.audio_file_id, 'active', afeat.created_at, afeat.created_at
FROM audio_features afeat WHERE NOT EXISTS (SELECT 1 FROM the_music m WHERE m.slug = 'audiofeat-' || afeat.id);

DO $$ BEGIN RAISE NOTICE 'the_music migration complete'; END $$;

-- ============================================================
-- STEP 5: temporal  (wave_calendar)
-- ============================================================

INSERT INTO temporal (slug, kind, title, body, type, icon, start, time, duration, tags, is_external, created_at, updated_at)
SELECT
    'wave-' || wc.id,
    'wave_event',
    wc.event_title,
    'Currency: ' || COALESCE(wc.currency, '') || ' Impact: ' || COALESCE(wc.impact, '') || ' Forecast: ' || COALESCE(wc.forecast, '') || ' Previous: ' || COALESCE(wc.previous, '') || ' Actual: ' || COALESCE(wc.actual, ''),
    'economic',
    '🌊',
    wc.event_time::date,
    to_char(wc.event_time, 'HH24:MI:SS'),
    NULL,
    COALESCE(array_to_string(wc.affected_pairs, ','), ''),
    true,
    wc.fetched_at,
    wc.fetched_at
FROM wave_calendar wc
WHERE NOT EXISTS (SELECT 1 FROM temporal t WHERE t.slug = 'wave-' || wc.id);

DO $$ BEGIN RAISE NOTICE 'temporal migration complete'; END $$;

-- ============================================================
-- STEP 6: identity  (contacts, teams, departments)
-- ============================================================

-- contacts (140 rows)
INSERT INTO identity (slug, kind, title, full_name, icon, status, created_at, updated_at)
SELECT
    'contact-' || c.id,
    'contact',
    COALESCE(NULLIF(c.full_name,''), COALESCE(c.first_name,'') || ' ' || COALESCE(c.last_name,''), 'Contact #' || c.id),
    COALESCE(NULLIF(c.full_name,''), COALESCE(c.first_name,'') || ' ' || COALESCE(c.last_name,''), 'Contact #' || c.id),
    '👤',
    'active',
    c.created_at,
    c.updated_at
FROM contacts c
WHERE NOT EXISTS (SELECT 1 FROM identity i WHERE i.slug = 'contact-' || c.id);

-- teams (13 rows)
INSERT INTO identity (slug, kind, title, full_name, icon, status, created_at, updated_at)
SELECT
    'team-' || t.id,
    'team',
    t.name,
    t.name,
    '👥',
    'active',
    now(),
    now()
FROM teams t
WHERE NOT EXISTS (SELECT 1 FROM identity i WHERE i.slug = 'team-' || t.id);

-- departments (8 rows)
INSERT INTO identity (slug, kind, title, full_name, icon, status, created_at, updated_at)
SELECT
    'dept-' || d.id,
    'department',
    d.name,
    d.name,
    '🏢',
    'active',
    now(),
    now()
FROM departments d
WHERE NOT EXISTS (SELECT 1 FROM identity i WHERE i.slug = 'dept-' || d.id);

DO $$ BEGIN RAISE NOTICE 'identity migration complete'; END $$;

-- ============================================================
-- STEP 7: the_realms  (locations)
-- ============================================================

INSERT INTO the_realms (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'loc-' || l.id,
    'location',
    l.name,
    'Type: ' || COALESCE(l.type, '') || ' Realm: ' || COALESCE(l.realm, '') || ' Lat: ' || COALESCE(l.lat::text, '') || ' Lng: ' || COALESCE(l.lng::text, '') || ' Notes: ' || COALESCE(l.notes, ''),
    'active',
    l.created_at,
    l.created_at
FROM locations l
WHERE NOT EXISTS (SELECT 1 FROM the_realms r WHERE r.slug = 'loc-' || l.id);

DO $$ BEGIN RAISE NOTICE 'the_realms migration complete'; END $$;

-- ============================================================
-- STEP 8: the_ledger  (tick_reports, agent_daily_memory, persona_mutations, agent_fitness)
-- ============================================================

-- Temporarily disable immutability triggers
ALTER TABLE the_ledger DISABLE TRIGGER trg_ledger_no_delete;
ALTER TABLE the_ledger DISABLE TRIGGER trg_ledger_no_update;

-- Build agent map
CREATE TEMP TABLE agent_id_map (agent_slug TEXT PRIMARY KEY, identity_id INTEGER);
INSERT INTO agent_id_map (agent_slug, identity_id)
SELECT lower(replace(slug, ' ', '')), id FROM identity
ON CONFLICT (agent_slug) DO NOTHING;
INSERT INTO agent_id_map (agent_slug, identity_id)
SELECT lower(split_part(full_name, ' ', 1)), id FROM identity
WHERE lower(split_part(full_name, ' ', 1)) NOT IN (SELECT agent_slug FROM agent_id_map)
ON CONFLICT (agent_slug) DO NOTHING;
INSERT INTO agent_id_map VALUES ('jmax', 21) ON CONFLICT DO NOTHING;
INSERT INTO agent_id_map VALUES ('lrm', 31) ON CONFLICT DO NOTHING;
INSERT INTO agent_id_map VALUES ('nova', 0) ON CONFLICT DO NOTHING;

-- tick_reports (440 rows)
INSERT INTO the_ledger (tick_number, ghost_id, tick_started_at, tick_status, perception_summary, action_taken, created_at)
SELECT
    tr.tick_number,
    0,
    tr.tick_at,
    'report',
    'agents=' || tr.total_agents || ' active=' || tr.active_agents || ' idle=' || tr.idle_agents || ' dormant=' || tr.dormant_agents,
    'llm_calls=' || tr.llm_calls || ' budget=' || tr.budget_used || '/' || tr.budget_max || E'\n' || COALESCE(tr.notable_events::text, ''),
    tr.tick_at
FROM tick_reports tr
WHERE NOT EXISTS (SELECT 1 FROM the_ledger l WHERE l.tick_number = tr.tick_number AND l.ghost_id = 0 AND l.tick_status = 'report');

-- agent_daily_memory (135 rows)
INSERT INTO the_ledger (tick_number, ghost_id, tick_started_at, tick_status, perception_summary, action_taken, created_at)
SELECT
    0,
    COALESCE(am.identity_id, 0),
    adm.created_at,
    'daily_memory',
    COALESCE(adm.daily_summary, ''),
    'actions: ' || COALESCE(adm.actions_taken, '') || E'\ndecisions: ' || COALESCE(adm.decisions_made, '') || E'\nblockers: ' || COALESCE(adm.blockers, ''),
    adm.created_at
FROM agent_daily_memory adm
LEFT JOIN agent_id_map am ON am.agent_slug = lower(replace(replace(adm.agent_id, '_', ''), ' ', ''))
WHERE NOT EXISTS (SELECT 1 FROM the_ledger l WHERE l.tick_status = 'daily_memory' AND l.created_at = adm.created_at AND l.ghost_id = COALESCE(am.identity_id, 0));

-- persona_mutations (67 rows)
INSERT INTO the_ledger (tick_number, ghost_id, tick_started_at, tick_status, perception_summary, action_taken, created_at)
SELECT
    0,
    COALESCE(am.identity_id, 0),
    pm.created_at,
    'persona_mutation',
    pm.mutation_type || ': ' || pm.description,
    COALESCE(pm.proposed_text, '') || ' status=' || pm.status || ' fitness_delta=' || COALESCE(pm.fitness_delta::text, '0'),
    pm.created_at
FROM persona_mutations pm
LEFT JOIN agent_id_map am ON am.agent_slug = lower(replace(replace(pm.agent_id, '_', ''), ' ', ''))
WHERE NOT EXISTS (SELECT 1 FROM the_ledger l WHERE l.tick_status = 'persona_mutation' AND l.created_at = pm.created_at AND l.ghost_id = COALESCE(am.identity_id, 0));

-- agent_fitness (164 rows)
INSERT INTO the_ledger (tick_number, ghost_id, tick_started_at, tick_status, perception_summary, action_taken, created_at)
SELECT
    0,
    COALESCE(am.identity_id, 0),
    af.created_at,
    'fitness_score',
    af.outcome || ' score=' || af.score,
    COALESCE(af.context, ''),
    af.created_at
FROM agent_fitness af
LEFT JOIN agent_id_map am ON am.agent_slug = lower(replace(replace(af.agent_id::text, '_', ''), ' ', ''))
WHERE NOT EXISTS (SELECT 1 FROM the_ledger l WHERE l.tick_status = 'fitness_score' AND l.created_at = af.created_at AND l.ghost_id = COALESCE(am.identity_id, 0));

-- Re-enable immutability triggers
ALTER TABLE the_ledger ENABLE TRIGGER trg_ledger_no_delete;
ALTER TABLE the_ledger ENABLE TRIGGER trg_ledger_no_update;
DROP TABLE agent_id_map;

DO $$ BEGIN RAISE NOTICE 'the_ledger migration complete'; END $$;

-- ============================================================
-- STEP 9: the_links  (agent_document_links, ghost_relationships, team_members, memory_links)
-- ============================================================

-- agent_document_links (3,015 rows) — agent_id is varchar (slug), need to resolve to identity.id
INSERT INTO the_links (source_table, source_id, source_field, target_slug, created_at)
SELECT
    'identity',
    COALESCE(i.id, 0),
    adl.link_type,
    'doc-' || adl.document_id,
    adl.created_at
FROM agent_document_links adl
LEFT JOIN identity i ON lower(replace(i.slug, ' ', '')) = lower(replace(replace(adl.agent_id, '_', ''), ' ', ''))
WHERE NOT EXISTS (SELECT 1 FROM the_links l WHERE l.source_table = 'identity' AND l.source_field = adl.link_type AND l.target_slug = 'doc-' || adl.document_id AND l.source_id = COALESCE(i.id, 0));

-- ghost_relationships (500 rows)
INSERT INTO the_links (source_table, source_id, source_field, target_slug)
SELECT
    'identity',
    COALESCE((SELECT id FROM identity WHERE lower(replace(slug,'','')) = lower(replace(replace(gr.from_agent,'_',''),' ',''))), 0),
    'ghost_' || gr.relationship_type,
    gr.to_agent
FROM ghost_relationships gr
WHERE NOT EXISTS (SELECT 1 FROM the_links l WHERE l.source_field = 'ghost_' || gr.relationship_type AND l.target_slug = gr.to_agent 
    AND l.source_id = COALESCE((SELECT id FROM identity WHERE lower(replace(slug,'','')) = lower(replace(replace(gr.from_agent,'_',''),' ',''))), 0));

-- team_members (66 rows)
INSERT INTO the_links (source_table, source_id, source_field, target_slug, created_at)
SELECT
    'identity',
    tm.team_id::bigint,
    'team_member_' || COALESCE(tm.role_in_team, 'member'),
    tm.agent_id,
    tm.joined_at
FROM team_members tm
WHERE NOT EXISTS (SELECT 1 FROM the_links l WHERE l.source_table = 'identity' AND l.source_field LIKE 'team_member%' AND l.target_slug = tm.agent_id AND l.source_id = tm.team_id::bigint);

DO $$ BEGIN RAISE NOTICE 'the_links migration complete'; END $$;

-- ============================================================
-- STEP 10: the_commons  (area_content, archives, annotations, feeds, fetched_articles, templates)
-- ============================================================

-- archives (993 rows)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'archive-' || a.id,
    'archive',
    a.title,
    a.content,
    'archived',
    a.created_at,
    a.created_at
FROM archives a
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'archive-' || a.id);

-- annotations (14 rows)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'annot-' || an.id,
    'annotation',
    'Annotation on doc ' || an.doc_id,
    an.selected_text || E'\n---\n' || an.comment,
    CASE WHEN an.resolved THEN 'resolved' ELSE 'active' END,
    an.created_at,
    an.created_at
FROM annotations an
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'annot-' || an.id);

-- fetched_articles (45 rows, separate from the bulk articles already migrated)
INSERT INTO the_commons (slug, kind, title, body, status, created_at, updated_at)
SELECT
    'fetched-' || fa.id,
    'fetched_article',
    fa.title,
    COALESCE(fa.content, fa.summary),
    'active',
    fa.fetched_at,
    fa.fetched_at
FROM fetched_articles fa
WHERE NOT EXISTS (SELECT 1 FROM the_commons c WHERE c.slug = 'fetched-' || fa.id);

DO $$ BEGIN RAISE NOTICE 'the_commons migration complete'; END $$;

-- ============================================================
-- FINAL: Summary
-- ============================================================

DO $$
DECLARE
    r RECORD;
BEGIN
    FOR r IN
        SELECT 'identity' as tbl, count(*) as cnt FROM identity
        UNION ALL SELECT 'temporal', count(*) FROM temporal
        UNION ALL SELECT 'the_work', count(*) FROM the_work
        UNION ALL SELECT 'the_post', count(*) FROM the_post
        UNION ALL SELECT 'the_markets', count(*) FROM the_markets
        UNION ALL SELECT 'the_music', count(*) FROM the_music
        UNION ALL SELECT 'the_realms', count(*) FROM the_realms
        UNION ALL SELECT 'the_chronicles', count(*) FROM the_chronicles
        UNION ALL SELECT 'the_commons', count(*) FROM the_commons
        UNION ALL SELECT 'the_press', count(*) FROM the_press
        UNION ALL SELECT 'the_ledger', count(*) FROM the_ledger
        UNION ALL SELECT 'the_links', count(*) FROM the_links
        UNION ALL SELECT 'the_index', count(*) FROM the_index
        UNION ALL SELECT 'the_aliases', count(*) FROM the_aliases
        ORDER BY tbl
    LOOP
        RAISE NOTICE '% = % rows', r.tbl, r.cnt;
    END LOOP;
END $$;

COMMIT;
