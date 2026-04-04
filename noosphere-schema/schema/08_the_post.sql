-- the_post: Universal messaging bus (conversations, messages, RSS comments, IRC, Matrix, email)
-- Source: 9,846 conversations in document_versions + future protocol adapters
-- ALL YAML KEYS BECOME COLUMNS. NOT JSONB. COLUMNS.

CREATE TABLE IF NOT EXISTS the_post (

    -- Core
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL,  -- 'conversation','message','rss_comment','irc_message','matrix_message','email','discord_message'
    title           TEXT,
    body            TEXT,
    type            TEXT,
    icon            TEXT,
    status          TEXT,

    -- Conversation threading
    conversation_id TEXT,           -- groups messages into conversations
    from_identity   TEXT,           -- wikilink to sender identity
    to_identity     TEXT,           -- wikilink to recipient identity
    subject         TEXT,
    replied_to      TEXT,           -- wikilink to parent message

    -- Protocol metadata
    protocol        TEXT,           -- 'internal','rss','irc','matrix','email','discord'
    external_url    TEXT,
    channel         TEXT,
    server          TEXT,
    message_id      TEXT,           -- external system message ID

    -- RSS comment fields
    source_article_url TEXT,
    source_article_title TEXT,
    comment_text    TEXT,
    extracted_quotes TEXT,
    generated_editorial TEXT,

    -- Email fields
    email_from      TEXT,
    email_to        TEXT,
    email_cc        TEXT,
    email_headers   TEXT,
    attachments     TEXT,

    -- AI fields
    ai_summary      TEXT,
    tasks_description TEXT,
    nexus           TEXT,           -- nexus AI reference

    -- People
    ceo             TEXT,
    department_head TEXT,
    em_staff        TEXT,

    -- Meta
    aliases         TEXT,
    tags            TEXT,
    provider        TEXT,           -- AI provider if generated
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);

COMMENT ON TABLE the_post IS 'Universal messaging bus. Modular protocol adapters (RSS, IRC, Matrix, Email, Discord). 9,846+ conversations.';

-- Indexes
CREATE INDEX idx_post_kind              ON the_post(kind);
CREATE INDEX idx_post_status            ON the_post(status);
CREATE INDEX idx_post_conversation_id   ON the_post(conversation_id);
CREATE INDEX idx_post_from_identity     ON the_post(from_identity);
CREATE INDEX idx_post_to_identity       ON the_post(to_identity);
CREATE INDEX idx_post_protocol          ON the_post(protocol);
CREATE INDEX idx_post_channel           ON the_post(channel);
CREATE INDEX idx_post_created_at        ON the_post(created_at);

CREATE INDEX idx_post_fts ON the_post USING GIN(
    to_tsvector('english',
        coalesce(title, '') || ' ' ||
        coalesce(body, '') || ' ' ||
        coalesce(subject, '') || ' ' ||
        coalesce(comment_text, '')
    )
);
