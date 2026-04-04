-- 10_the_post.sql
-- @Communications — messaging infrastructure.
-- Ghost↔ghost via conversations. Nathan↔ghosts via email.
-- Chat is short (1-2 sentences). Reports are long-form.

CREATE TABLE the_post (
    id          BIGSERIAL PRIMARY KEY,
    slug        TEXT NOT NULL,
    kind        TEXT NOT NULL,
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
    -- NO updated_at: messages are append-only. Only read_by gets updated.
);

COMMENT ON TABLE the_post IS '@Communications — messaging infrastructure. Ghost↔ghost via conversations table. Nathan↔ghosts via email. Chat: 1-2 sentence max. Report: long-form.';

-- Kind taxonomy:
-- chat         Short message, 1-2 sentences max. Status updates, acknowledgments.
-- report       Long-form report. Substance, analysis, detailed findings.
-- handoff      Pipeline handoff between ghosts. meta.source='handoff',
--              meta.reason, meta.context. Handoffs never expire until responded to.
-- notification System notification
-- inbox        Inbound message pending Nathan's action
-- feedback     Noosphere feedback entry

-- channel values: noosphere, email, system

-- meta for chat/report:
--   responding_to: (id of message being responded to)
--   source: cognition_broker | handoff | system
--   job_id, provider, cached (cognition metadata)

-- meta for handoff:
--   source: 'handoff'
--   reason: why the handoff
--   context: what the next ghost needs to know
--   pipeline_slug: which pipeline this handoff is part of

-- Constraint: kind='chat' messages should be ≤ 280 chars in body
-- Enforced at application layer (ghost LLM prompt + API validation)

-- Slug is NOT unique here — messages are high-volume append-only
-- Slug format: msg-{from_agent}-{timestamp-fragment}

CREATE INDEX idx_post_kind ON the_post(kind);
CREATE INDEX idx_post_from ON the_post(from_agent);
CREATE INDEX idx_post_to ON the_post USING gin(to_agent);
CREATE INDEX idx_post_thread ON the_post(thread_id) WHERE thread_id IS NOT NULL;
CREATE INDEX idx_post_channel ON the_post(channel);
CREATE INDEX idx_post_read ON the_post USING gin(read_by);
CREATE INDEX idx_post_domain ON the_post(domain) WHERE domain IS NOT NULL;
CREATE INDEX idx_post_meta ON the_post USING gin(meta);
CREATE INDEX idx_post_created ON the_post(created_at DESC);

-- Perception hot path: "unread messages to agent X"
-- This is the single most critical query in the system
CREATE INDEX idx_post_unread_to ON the_post USING gin(to_agent)
    WHERE status = 'sent';

-- Handoff detection: handoffs that haven't been responded to
CREATE INDEX idx_post_handoffs ON the_post(created_at DESC)
    WHERE kind = 'handoff';
