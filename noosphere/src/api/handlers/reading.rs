//! RSS Reader endpoints

use axum::{
    extract::{Path, Query, State},
    Json,
};
use dpn_core::DbPool;
use serde::Deserialize;
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub url: String,
}

#[derive(Deserialize)]
pub struct FirehoseParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    100
}

#[derive(Deserialize)]
pub struct CommentRequest {
    pub article_url: String,
    pub article_title: String,
    pub comment: String,
}

#[derive(Deserialize)]
pub struct OpmlImportRequest {
    pub opml_content: String,
}

/// List all subscribed feeds
pub async fn list_feeds(
    State(pool): State<DbPool>,
) -> Result<Json<Value>, ApiError> {
    let feeds = dpn_core::list_feeds(&pool)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "feeds": feeds })))
}

/// Subscribe to a feed (with auto-discovery)
pub async fn subscribe(
    State(pool): State<DbPool>,
    Json(req): Json<SubscribeRequest>,
) -> Result<Json<Value>, ApiError> {
    let feed = dpn_core::subscribe_feed(&pool, &req.url)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "feed": feed
    })))
}

/// Refresh a feed (fetch new articles)
pub async fn refresh_feed(
    State(pool): State<DbPool>,
    Path(feed_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    let count = dpn_core::refresh_feed(&pool, feed_id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "new_articles": count
    })))
}

/// Get firehose (all articles, newest first)
pub async fn firehose(
    State(pool): State<DbPool>,
    Query(params): Query<FirehoseParams>,
) -> Result<Json<Value>, ApiError> {
    let articles = dpn_core::get_firehose(&pool, params.limit)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "articles": articles })))
}

/// Mark article as read
pub async fn mark_read(
    State(pool): State<DbPool>,
    Path(article_id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    dpn_core::mark_read(&pool, article_id)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Create a reading comment (stores in Thought Police)
pub async fn create_comment(
    State(pool): State<DbPool>,
    Json(req): Json<CommentRequest>,
) -> Result<Json<Value>, ApiError> {
    let doc_id = dpn_core::create_comment(
        &pool,
        &req.article_url,
        &req.article_title,
        &req.comment,
    )
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "document_id": doc_id
    })))
}

/// Extract feed URLs from OPML content using simple string parsing
fn extract_feed_urls(opml_content: &str) -> Vec<String> {
    let mut feed_urls: Vec<String> = Vec::new();
    let content_lower = opml_content.to_lowercase();
    let mut search_start = 0;
    
    while let Some(pos) = content_lower[search_start..].find("xmlurl=") {
        let abs_pos = search_start + pos;
        let after_attr = abs_pos + 7;
        
        if after_attr >= opml_content.len() {
            break;
        }
        
        let quote_char = opml_content.chars().nth(after_attr).unwrap_or('"');
        if quote_char != '"' && quote_char != '\'' {
            search_start = after_attr;
            continue;
        }
        
        let url_start = after_attr + 1;
        if let Some(url_end) = opml_content[url_start..].find(quote_char) {
            let url = &opml_content[url_start..url_start + url_end];
            if !url.is_empty() && url.starts_with("http") && !feed_urls.contains(&url.to_string()) {
                feed_urls.push(url.to_string());
            }
        }
        
        search_start = after_attr + 1;
    }
    
    feed_urls
}

/// Import OPML subscription list - queues to agent_requests for Nova to process
pub async fn import_opml(
    State(pool): State<DbPool>,
    Json(req): Json<OpmlImportRequest>,
) -> Result<Json<Value>, ApiError> {
    let feed_urls = extract_feed_urls(&req.opml_content);
    
    if feed_urls.is_empty() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "No feed URLs found in OPML content",
            "hint": "OPML should contain <outline> elements with xmlUrl attributes"
        })));
    }
    
    let payload = serde_json::json!({
        "feed_urls": feed_urls
    });
    
    let result: (i32,) = sqlx::query_as(
        "INSERT INTO agent_requests (from_agent, to_agent, request_type, subject, context, priority, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id"
    )
    .bind("api")
    .bind(&vec!["nova"])
    .bind("opml_import")
    .bind(format!("Import {} feeds from OPML", feed_urls.len()))
    .bind(payload.to_string())
    .bind(2_i32)
    .bind("pending")
    .fetch_one(&pool)
    .await
    .map_err(|e| ApiError::Database(e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "request_id": result.0,
        "feed_count": feed_urls.len(),
        "feeds": feed_urls,
        "message": "OPML import queued. Nova will process the subscriptions."
    })))
}
