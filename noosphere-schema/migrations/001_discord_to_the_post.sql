-- Migration: discord_messages -> the_post
-- Source: 13,457 rows from discord_messages
-- Target: the_post with kind='discord_message', protocol='discord'
-- 
-- Run with: psql -h localhost -p 5432 -d master_chronicle -f noosphere-schema/migrations/001_discord_to_the_post.sql
-- Rollback: DELETE FROM the_post WHERE kind = 'discord_message';

BEGIN;

-- Verify source count before migration
DO $$
DECLARE
    src_count INTEGER;
BEGIN
    SELECT count(*) INTO src_count FROM discord_messages;
    RAISE NOTICE 'Source discord_messages: % rows', src_count;
END $$;

-- Insert discord_messages into the_post
INSERT INTO the_post (
    slug,
    kind,
    title,
    body,
    type,
    icon,
    status,
    conversation_id,
    from_identity,
    to_identity,
    protocol,
    channel,
    server,
    message_id,
    attachments,
    created_at,
    updated_at
)
SELECT
    -- slug: discord-{message_id} for uniqueness
    'discord-' || dm.id                                     AS slug,
    'discord_message'                                       AS kind,
    -- title: first 80 chars of content or author + timestamp
    COALESCE(
        NULLIF(left(dm.content, 80), ''),
        dm.author_name || ' at ' || dm.timestamp::text
    )                                                       AS title,
    dm.content                                              AS body,
    CASE dm.message_type
        WHEN 0 THEN 'default'
        WHEN 19 THEN 'reply'
        WHEN 7 THEN 'member_join'
        ELSE 'type_' || dm.message_type::text
    END                                                     AS type,
    '💬'                                                    AS icon,
    'active'                                                AS status,
    -- conversation_id: group by channel
    dm.channel_id                                           AS conversation_id,
    -- from_identity: map known users to identity slugs
    CASE dm.author_username
        WHEN 'n8k99' THEN 'NathanEckenrode'
        WHEN 'T.A.S.K.S.' THEN 'TASKS'
        WHEN 'JayHarper' THEN 'JayHarper'
        WHEN 'KaiNakamoto' THEN 'KaiNakamoto'
        WHEN 'MiloGaines' THEN 'MiloGaines'
        WHEN 'NinaCastillo' THEN 'NinaCastillo'
        WHEN 'PriyaPatel' THEN 'PriyaPatel'
        WHEN 'SarahLin' THEN 'SarahLin'
        WHEN 'TaraBennett' THEN 'TaraBennett'
        WHEN 'TinaGray' THEN 'TinaGray'
        WHEN 'ZaraKhan' THEN 'ZaraKhan'
        ELSE dm.author_username
    END                                                     AS from_identity,
    NULL                                                    AS to_identity,
    'discord'                                               AS protocol,
    dm.channel_id                                           AS channel,
    NULL                                                    AS server,
    dm.id::text                                             AS message_id,
    CASE 
        WHEN dm.attachments IS NOT NULL AND dm.attachments::text != '[]' 
        THEN dm.attachments::text
        ELSE NULL
    END                                                     AS attachments,
    dm.timestamp                                            AS created_at,
    COALESCE(dm.edited_at, dm.timestamp)                    AS updated_at
FROM discord_messages dm
-- Avoid duplicates if re-run
WHERE NOT EXISTS (
    SELECT 1 FROM the_post tp 
    WHERE tp.slug = 'discord-' || dm.id
)
ORDER BY dm.timestamp;

-- Verify result
DO $$
DECLARE
    new_count INTEGER;
    total_count INTEGER;
BEGIN
    SELECT count(*) INTO new_count FROM the_post WHERE kind = 'discord_message';
    SELECT count(*) INTO total_count FROM the_post;
    RAISE NOTICE 'Migrated discord_messages: % rows', new_count;
    RAISE NOTICE 'Total the_post rows: %', total_count;
END $$;

COMMIT;
