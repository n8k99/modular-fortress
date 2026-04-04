//! Publish endpoints for RSS-first content management
//!
//! Handles:
//! - Stream CRUD (content streams like Myths of Orbis, Living Room Music, Thought Police)
//! - Drop CRUD (publishable content)
//! - RSS/Atom feed generation
//! - Thought Police responses

use axum::{
    extract::{Path, Query, State},
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use dpn_core::DbPool;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ApiError;

// =============================================================================
// Request/Response types
// =============================================================================

#[derive(Deserialize)]
pub struct CreateDropRequest {
    pub stream_id: i32,
    pub slug: String,
    pub title: String,
    pub content_markdown: String,
    pub excerpt: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub status: String,
    pub published_at: Option<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub enclosure_length: Option<i64>,
    pub duration_seconds: Option<i32>,
    pub featured_image: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdateDropRequest {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub content_markdown: Option<String>,
    pub excerpt: Option<String>,
    pub author: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub enclosure_length: Option<i64>,
    pub duration_seconds: Option<i32>,
    pub featured_image: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct CreateStreamRequest {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub language: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    #[serde(default)]
    pub is_podcast: bool,
    pub podcast_category: Option<String>,
    pub podcast_image: Option<String>,
    #[serde(default)]
    pub podcast_explicit: bool,
}

#[derive(Deserialize, Default)]
pub struct ListDropsQuery {
    pub stream_id: Option<i32>,
    pub stream_slug: Option<String>,
    pub status: Option<String>,
    pub tag: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct FeedQuery {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_format() -> String {
    "rss".to_string()
}

fn default_limit() -> i64 {
    50
}

#[derive(Deserialize)]
pub struct CreateResponseRequest {
    pub drop_id: i32,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ModerateRequest {
    pub approved: bool,
}

// =============================================================================
// Stream endpoints
// =============================================================================

/// List all streams
pub async fn list_streams(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let streams = dpn_core::list_streams(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "streams": streams })))
}

/// Get a stream by ID or slug
pub async fn get_stream(
    State(pool): State<DbPool>,
    Path(id_or_slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let stream = if let Ok(id) = id_or_slug.parse::<i32>() {
        dpn_core::get_stream(&pool, id).await
    } else {
        dpn_core::get_stream_by_slug(&pool, &id_or_slug).await
    }
    .map_err(|e| ApiError::Database(e.to_string()))?;

    match stream {
        Some(s) => Ok(Json(serde_json::json!({ "stream": s }))),
        None => Err(ApiError::NotFound("Stream not found".to_string())),
    }
}

/// Create a new stream
pub async fn create_stream(
    State(pool): State<DbPool>,
    Json(req): Json<CreateStreamRequest>,
) -> Result<Json<Value>, ApiError> {
    let stream_create = dpn_core::StreamCreate {
        slug: req.slug,
        title: req.title,
        description: req.description,
        site_url: req.site_url,
        language: req.language,
        author: req.author,
        email: req.email,
        is_podcast: req.is_podcast,
        podcast_category: req.podcast_category,
        podcast_image: req.podcast_image,
        podcast_explicit: req.podcast_explicit,
    };

    let stream = dpn_core::create_stream(&pool, stream_create)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "stream": stream
    })))
}

/// Update a stream
pub async fn update_stream(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<CreateStreamRequest>,
) -> Result<Json<Value>, ApiError> {
    let stream_create = dpn_core::StreamCreate {
        slug: req.slug,
        title: req.title,
        description: req.description,
        site_url: req.site_url,
        language: req.language,
        author: req.author,
        email: req.email,
        is_podcast: req.is_podcast,
        podcast_category: req.podcast_category,
        podcast_image: req.podcast_image,
        podcast_explicit: req.podcast_explicit,
    };

    let stream = dpn_core::update_stream(&pool, id, stream_create)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "stream": stream
    })))
}

/// Delete a stream
pub async fn delete_stream(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    dpn_core::delete_stream(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "success": true })))
}

// =============================================================================
// Drop endpoints
// =============================================================================

/// List drops with optional filtering
pub async fn list_drops(
    State(pool): State<DbPool>,
    Query(query): Query<ListDropsQuery>,
) -> Result<Json<Value>, ApiError> {
    let filter = dpn_core::DropFilter {
        stream_id: query.stream_id,
        stream_slug: query.stream_slug,
        status: query.status.map(|s| dpn_core::DropStatus::from_str(&s)),
        tag: query.tag,
        limit: query.limit,
        offset: query.offset,
    };

    let drops = dpn_core::list_drops(&pool, filter)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "drops": drops })))
}

/// Get a drop by ID
pub async fn get_drop(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let drop = dpn_core::get_drop(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    match drop {
        Some(d) => Ok(Json(serde_json::json!({ "drop": d }))),
        None => Err(ApiError::NotFound("Drop not found".to_string())),
    }
}

/// Create a new drop
pub async fn create_drop(
    State(pool): State<DbPool>,
    Json(req): Json<CreateDropRequest>,
) -> Result<Json<Value>, ApiError> {
    let published_at = req.published_at.and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(&s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let drop_create = dpn_core::DropCreate {
        stream_id: req.stream_id,
        slug: req.slug,
        title: req.title,
        content_markdown: req.content_markdown,
        excerpt: req.excerpt,
        author: req.author,
        status: dpn_core::DropStatus::from_str(&req.status),
        published_at,
        enclosure_url: req.enclosure_url,
        enclosure_type: req.enclosure_type,
        enclosure_length: req.enclosure_length,
        duration_seconds: req.duration_seconds,
        featured_image: req.featured_image,
        tags: req.tags,
    };

    let drop = dpn_core::create_drop(&pool, drop_create)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "drop": drop
    })))
}

/// Update an existing drop
pub async fn update_drop(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateDropRequest>,
) -> Result<Json<Value>, ApiError> {
    let published_at = req.published_at.and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(&s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let drop_update = dpn_core::DropUpdate {
        slug: req.slug,
        title: req.title,
        content_markdown: req.content_markdown,
        excerpt: req.excerpt,
        author: req.author,
        status: req.status.map(|s| dpn_core::DropStatus::from_str(&s)),
        published_at,
        enclosure_url: req.enclosure_url,
        enclosure_type: req.enclosure_type,
        enclosure_length: req.enclosure_length,
        duration_seconds: req.duration_seconds,
        featured_image: req.featured_image,
        tags: req.tags,
    };

    let drop = dpn_core::update_drop(&pool, id, drop_update)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "drop": drop
    })))
}

/// Delete a drop
pub async fn delete_drop(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    dpn_core::delete_drop(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Publish a drop (set status to published)
pub async fn publish_drop(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let drop = dpn_core::publish_drop(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "drop": drop
    })))
}

// =============================================================================
// Feed endpoints
// =============================================================================

/// Get RSS/Atom feed for a stream
pub async fn get_feed(
    State(pool): State<DbPool>,
    Path(stream_slug): Path<String>,
    Query(query): Query<FeedQuery>,
) -> Result<Response, ApiError> {
    // Get stream
    let stream = dpn_core::get_stream_by_slug(&pool, &stream_slug)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Stream not found".to_string()))?;

    // Get published drops
    let drops = dpn_core::list_published_drops(&pool, stream.id, query.limit)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    // Base URL from environment or default
    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "https://n8k99.com".to_string());

    // Generate feed based on format and type
    let (content, content_type) = match query.format.as_str() {
        "atom" => (
            dpn_core::generate_atom(&stream, &drops, &base_url),
            "application/atom+xml; charset=utf-8",
        ),
        _ => {
            // RSS (default) - use podcast RSS for podcast streams
            if stream.is_podcast {
                (
                    dpn_core::generate_podcast_rss(&stream, &drops, &base_url),
                    "application/rss+xml; charset=utf-8",
                )
            } else {
                (
                    dpn_core::generate_rss(&stream, &drops, &base_url),
                    "application/rss+xml; charset=utf-8",
                )
            }
        }
    };

    Ok((
        [(header::CONTENT_TYPE, content_type)],
        content,
    ).into_response())
}

// =============================================================================
// Response endpoints (Thought Police comments)
// =============================================================================

/// Create a response to a drop
pub async fn create_response(
    State(pool): State<DbPool>,
    Json(req): Json<CreateResponseRequest>,
) -> Result<Json<Value>, ApiError> {
    let response_create = dpn_core::ResponseCreate {
        drop_id: req.drop_id,
        author_name: req.author_name,
        author_email: req.author_email,
        content: req.content,
    };

    let response = dpn_core::create_response(&pool, response_create)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "response": response
    })))
}

/// List responses for a drop
pub async fn list_responses(
    State(pool): State<DbPool>,
    Path(drop_id): Path<i32>,
    Query(params): Query<ListResponsesQuery>,
) -> Result<Json<Value>, ApiError> {
    let approved_only = params.approved_only.unwrap_or(true);

    let responses = dpn_core::list_responses(&pool, drop_id, approved_only)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "responses": responses })))
}

#[derive(Deserialize, Default)]
pub struct ListResponsesQuery {
    pub approved_only: Option<bool>,
}

/// Moderate a response (approve/reject)
pub async fn moderate_response(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(req): Json<ModerateRequest>,
) -> Result<Json<Value>, ApiError> {
    let response = dpn_core::moderate_response(&pool, id, req.approved)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "response": response
    })))
}

/// Delete a response
pub async fn delete_response(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    dpn_core::delete_response(&pool, id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "success": true })))
}
