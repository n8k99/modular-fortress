//! Database operations for drops and streams

use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;

use super::content::markdown_to_html;
use super::types::*;

// =============================================================================
// Stream operations
// =============================================================================

/// Create a new stream
pub async fn create_stream(pool: &PgPool, stream: StreamCreate) -> Result<Stream> {
    let result = sqlx::query_as!(
        Stream,
        r#"
        INSERT INTO streams (
            slug, title, description, site_url, language, author, email,
            is_podcast, podcast_category, podcast_image, podcast_explicit
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, slug, title, description, site_url, language, author, email,
                  is_podcast, podcast_category, podcast_image, podcast_explicit,
                  created_at, updated_at
        "#,
        stream.slug,
        stream.title,
        stream.description,
        stream.site_url,
        stream.language,
        stream.author,
        stream.email,
        stream.is_podcast,
        stream.podcast_category,
        stream.podcast_image,
        stream.podcast_explicit
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// Get a stream by ID
pub async fn get_stream(pool: &PgPool, id: i32) -> Result<Option<Stream>> {
    let stream = sqlx::query_as!(
        Stream,
        r#"
        SELECT id, slug, title, description, site_url, language, author, email,
               is_podcast, podcast_category, podcast_image, podcast_explicit,
               created_at, updated_at
        FROM streams
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(stream)
}

/// Get a stream by slug
pub async fn get_stream_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Stream>> {
    let stream = sqlx::query_as!(
        Stream,
        r#"
        SELECT id, slug, title, description, site_url, language, author, email,
               is_podcast, podcast_category, podcast_image, podcast_explicit,
               created_at, updated_at
        FROM streams
        WHERE slug = $1
        "#,
        slug
    )
    .fetch_optional(pool)
    .await?;

    Ok(stream)
}

/// List all streams
pub async fn list_streams(pool: &PgPool) -> Result<Vec<Stream>> {
    let streams = sqlx::query_as!(
        Stream,
        r#"
        SELECT id, slug, title, description, site_url, language, author, email,
               is_podcast, podcast_category, podcast_image, podcast_explicit,
               created_at, updated_at
        FROM streams
        ORDER BY title
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(streams)
}

/// Update a stream
pub async fn update_stream(pool: &PgPool, id: i32, stream: StreamCreate) -> Result<Stream> {
    let result = sqlx::query_as!(
        Stream,
        r#"
        UPDATE streams
        SET slug = $2, title = $3, description = $4, site_url = $5,
            language = $6, author = $7, email = $8, is_podcast = $9,
            podcast_category = $10, podcast_image = $11, podcast_explicit = $12,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, slug, title, description, site_url, language, author, email,
                  is_podcast, podcast_category, podcast_image, podcast_explicit,
                  created_at, updated_at
        "#,
        id,
        stream.slug,
        stream.title,
        stream.description,
        stream.site_url,
        stream.language,
        stream.author,
        stream.email,
        stream.is_podcast,
        stream.podcast_category,
        stream.podcast_image,
        stream.podcast_explicit
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// Delete a stream
pub async fn delete_stream(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM streams WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

// =============================================================================
// Drop operations
// =============================================================================

/// Create a new drop
pub async fn create_drop(pool: &PgPool, drop: DropCreate) -> Result<Drop> {
    // Convert markdown to HTML
    let content_html = markdown_to_html(&drop.content_markdown);

    let result = sqlx::query_as!(
        DropRow,
        r#"
        INSERT INTO drops (
            stream_id, slug, title, content_markdown, content_html, excerpt, author,
            status, published_at, enclosure_url, enclosure_type, enclosure_length,
            duration_seconds, featured_image, tags
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                  status, published_at, enclosure_url, enclosure_type, enclosure_length,
                  duration_seconds, featured_image, tags, created_at, updated_at
        "#,
        drop.stream_id,
        drop.slug,
        drop.title,
        drop.content_markdown,
        content_html,
        drop.excerpt,
        drop.author,
        drop.status.as_str(),
        drop.published_at,
        drop.enclosure_url,
        drop.enclosure_type,
        drop.enclosure_length,
        drop.duration_seconds,
        drop.featured_image,
        drop.tags.as_deref()
    )
    .fetch_one(pool)
    .await?;

    Ok(result.into())
}

/// Get a drop by ID
pub async fn get_drop(pool: &PgPool, id: i32) -> Result<Option<Drop>> {
    let drop = sqlx::query_as!(
        DropRow,
        r#"
        SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
               status, published_at, enclosure_url, enclosure_type, enclosure_length,
               duration_seconds, featured_image, tags, created_at, updated_at
        FROM drops
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(drop.map(Into::into))
}

/// Get a drop by slug within a stream
pub async fn get_drop_by_slug(pool: &PgPool, stream_id: i32, slug: &str) -> Result<Option<Drop>> {
    let drop = sqlx::query_as!(
        DropRow,
        r#"
        SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
               status, published_at, enclosure_url, enclosure_type, enclosure_length,
               duration_seconds, featured_image, tags, created_at, updated_at
        FROM drops
        WHERE stream_id = $1 AND slug = $2
        "#,
        stream_id,
        slug
    )
    .fetch_optional(pool)
    .await?;

    Ok(drop.map(Into::into))
}

/// List drops with optional filtering
pub async fn list_drops(pool: &PgPool, filter: DropFilter) -> Result<Vec<Drop>> {
    let limit = filter.limit.unwrap_or(50);
    let offset = filter.offset.unwrap_or(0);

    // Build dynamic query based on filters
    let drops = if let Some(stream_id) = filter.stream_id {
        if let Some(status) = filter.status {
            sqlx::query_as!(
                DropRow,
                r#"
                SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                       status, published_at, enclosure_url, enclosure_type, enclosure_length,
                       duration_seconds, featured_image, tags, created_at, updated_at
                FROM drops
                WHERE stream_id = $1 AND status = $2
                ORDER BY published_at DESC NULLS LAST, created_at DESC
                LIMIT $3 OFFSET $4
                "#,
                stream_id,
                status.as_str(),
                limit,
                offset
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                DropRow,
                r#"
                SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                       status, published_at, enclosure_url, enclosure_type, enclosure_length,
                       duration_seconds, featured_image, tags, created_at, updated_at
                FROM drops
                WHERE stream_id = $1
                ORDER BY published_at DESC NULLS LAST, created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                stream_id,
                limit,
                offset
            )
            .fetch_all(pool)
            .await?
        }
    } else if let Some(status) = filter.status {
        sqlx::query_as!(
            DropRow,
            r#"
            SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                   status, published_at, enclosure_url, enclosure_type, enclosure_length,
                   duration_seconds, featured_image, tags, created_at, updated_at
            FROM drops
            WHERE status = $1
            ORDER BY published_at DESC NULLS LAST, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            status.as_str(),
            limit,
            offset
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            DropRow,
            r#"
            SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                   status, published_at, enclosure_url, enclosure_type, enclosure_length,
                   duration_seconds, featured_image, tags, created_at, updated_at
            FROM drops
            ORDER BY published_at DESC NULLS LAST, created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?
    };

    Ok(drops.into_iter().map(Into::into).collect())
}

/// List published drops for a stream (for feed generation)
pub async fn list_published_drops(pool: &PgPool, stream_id: i32, limit: i64) -> Result<Vec<Drop>> {
    let drops = sqlx::query_as!(
        DropRow,
        r#"
        SELECT id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
               status, published_at, enclosure_url, enclosure_type, enclosure_length,
               duration_seconds, featured_image, tags, created_at, updated_at
        FROM drops
        WHERE stream_id = $1 AND status = 'published' AND published_at <= NOW()
        ORDER BY published_at DESC
        LIMIT $2
        "#,
        stream_id,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(drops.into_iter().map(Into::into).collect())
}

/// Update a drop
pub async fn update_drop(pool: &PgPool, id: i32, update: DropUpdate) -> Result<Drop> {
    // Get existing drop first
    let existing = get_drop(pool, id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Drop not found"))?;

    let slug = update.slug.unwrap_or(existing.slug);
    let title = update.title.unwrap_or(existing.title);
    let content_markdown = update.content_markdown.unwrap_or(existing.content_markdown);
    let content_html = markdown_to_html(&content_markdown);
    let excerpt = update.excerpt.or(existing.excerpt);
    let author = update.author.or(existing.author);
    let status = update.status.unwrap_or(existing.status);
    let published_at = update.published_at.or(existing.published_at);
    let enclosure_url = update.enclosure_url.or(existing.enclosure_url);
    let enclosure_type = update.enclosure_type.or(existing.enclosure_type);
    let enclosure_length = update.enclosure_length.or(existing.enclosure_length);
    let duration_seconds = update.duration_seconds.or(existing.duration_seconds);
    let featured_image = update.featured_image.or(existing.featured_image);
    let tags = update.tags.or(existing.tags);

    let result = sqlx::query_as!(
        DropRow,
        r#"
        UPDATE drops
        SET slug = $2, title = $3, content_markdown = $4, content_html = $5,
            excerpt = $6, author = $7, status = $8, published_at = $9,
            enclosure_url = $10, enclosure_type = $11, enclosure_length = $12,
            duration_seconds = $13, featured_image = $14, tags = $15,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                  status, published_at, enclosure_url, enclosure_type, enclosure_length,
                  duration_seconds, featured_image, tags, created_at, updated_at
        "#,
        id,
        slug,
        title,
        content_markdown,
        content_html,
        excerpt,
        author,
        status.as_str(),
        published_at,
        enclosure_url,
        enclosure_type,
        enclosure_length,
        duration_seconds,
        featured_image,
        tags.as_deref()
    )
    .fetch_one(pool)
    .await?;

    Ok(result.into())
}

/// Delete a drop
pub async fn delete_drop(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM drops WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Publish a drop (set status to published and set published_at if not set)
pub async fn publish_drop(pool: &PgPool, id: i32) -> Result<Drop> {
    let result = sqlx::query_as!(
        DropRow,
        r#"
        UPDATE drops
        SET status = 'published',
            published_at = COALESCE(published_at, NOW()),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, stream_id, slug, title, content_markdown, content_html, excerpt, author,
                  status, published_at, enclosure_url, enclosure_type, enclosure_length,
                  duration_seconds, featured_image, tags, created_at, updated_at
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.into())
}

// =============================================================================
// Response operations (Thought Police comments)
// =============================================================================

/// Create a response to a drop
pub async fn create_response(pool: &PgPool, response: ResponseCreate) -> Result<ThoughtPoliceResponse> {
    let result = sqlx::query_as!(
        ThoughtPoliceResponse,
        r#"
        INSERT INTO responses (drop_id, author_name, author_email, content, approved)
        VALUES ($1, $2, $3, $4, false)
        RETURNING id, drop_id, author_name, author_email, content, approved, created_at
        "#,
        response.drop_id,
        response.author_name,
        response.author_email,
        response.content
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// List responses for a drop
pub async fn list_responses(pool: &PgPool, drop_id: i32, approved_only: bool) -> Result<Vec<ThoughtPoliceResponse>> {
    let responses = if approved_only {
        sqlx::query_as!(
            ThoughtPoliceResponse,
            r#"
            SELECT id, drop_id, author_name, author_email, content, approved, created_at
            FROM responses
            WHERE drop_id = $1 AND approved = true
            ORDER BY created_at
            "#,
            drop_id
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            ThoughtPoliceResponse,
            r#"
            SELECT id, drop_id, author_name, author_email, content, approved, created_at
            FROM responses
            WHERE drop_id = $1
            ORDER BY created_at
            "#,
            drop_id
        )
        .fetch_all(pool)
        .await?
    };

    Ok(responses)
}

/// Approve or reject a response
pub async fn moderate_response(pool: &PgPool, id: i32, approved: bool) -> Result<ThoughtPoliceResponse> {
    let result = sqlx::query_as!(
        ThoughtPoliceResponse,
        r#"
        UPDATE responses
        SET approved = $2
        WHERE id = $1
        RETURNING id, drop_id, author_name, author_email, content, approved, created_at
        "#,
        id,
        approved
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// Delete a response
pub async fn delete_response(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM responses WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

// =============================================================================
// Internal types for sqlx mapping
// =============================================================================

/// Internal row type for sqlx (handles string status)
#[derive(Debug)]
struct DropRow {
    id: i32,
    stream_id: i32,
    slug: String,
    title: String,
    content_markdown: String,
    content_html: Option<String>,
    excerpt: Option<String>,
    author: Option<String>,
    status: String,
    published_at: Option<chrono::DateTime<Utc>>,
    enclosure_url: Option<String>,
    enclosure_type: Option<String>,
    enclosure_length: Option<i64>,
    duration_seconds: Option<i32>,
    featured_image: Option<String>,
    tags: Option<Vec<String>>,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

impl From<DropRow> for Drop {
    fn from(row: DropRow) -> Self {
        Drop {
            id: row.id,
            stream_id: row.stream_id,
            slug: row.slug,
            title: row.title,
            content_markdown: row.content_markdown,
            content_html: row.content_html,
            excerpt: row.excerpt,
            author: row.author,
            status: DropStatus::from_str(&row.status),
            published_at: row.published_at,
            enclosure_url: row.enclosure_url,
            enclosure_type: row.enclosure_type,
            enclosure_length: row.enclosure_length,
            duration_seconds: row.duration_seconds,
            featured_image: row.featured_image,
            tags: row.tags,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_row_conversion() {
        let row = DropRow {
            id: 1,
            stream_id: 1,
            slug: "test".to_string(),
            title: "Test".to_string(),
            content_markdown: "# Hello".to_string(),
            content_html: Some("<h1>Hello</h1>".to_string()),
            excerpt: None,
            author: Some("Author".to_string()),
            status: "published".to_string(),
            published_at: Some(Utc::now()),
            enclosure_url: None,
            enclosure_type: None,
            enclosure_length: None,
            duration_seconds: None,
            featured_image: None,
            tags: Some(vec!["tag1".to_string()]),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        let drop: Drop = row.into();
        assert_eq!(drop.id, 1);
        assert_eq!(drop.status, DropStatus::Published);
    }
}
