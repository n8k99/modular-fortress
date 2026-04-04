//! resources table access and CRUD operations
//!
//! Curated index referencing documents/media via source_type/source_id.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

use super::DbPool;

/// Resource record from database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Resource {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub resource_type: String,
    pub source_type: String,
    pub source_id: i32,
    pub description: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub frozen: bool,
    pub area_id: Option<i32>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// List all resources, ordered by name
pub async fn list_resources(pool: &DbPool) -> Result<Vec<Resource>> {
    let resources = sqlx::query_as::<_, Resource>(
        r#"
        SELECT id, name, slug, resource_type, source_type, source_id, description,
               tags, frozen, area_id, metadata, created_at, updated_at
        FROM resources
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(resources)
}

/// Get resource by ID
pub async fn get_resource_by_id(pool: &DbPool, id: i32) -> Result<Option<Resource>> {
    let resource = sqlx::query_as::<_, Resource>(
        r#"
        SELECT id, name, slug, resource_type, source_type, source_id, description,
               tags, frozen, area_id, metadata, created_at, updated_at
        FROM resources
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(resource)
}

/// Create a new resource
pub async fn create_resource(
    pool: &DbPool,
    name: &str,
    slug: &str,
    resource_type: &str,
    source_type: &str,
    source_id: i32,
    description: Option<&str>,
    tags: Option<&serde_json::Value>,
    frozen: bool,
    area_id: Option<i32>,
) -> Result<i32> {
    let result = sqlx::query(
        r#"
        INSERT INTO resources (name, slug, resource_type, source_type, source_id,
                               description, tags, frozen, area_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(slug)
    .bind(resource_type)
    .bind(source_type)
    .bind(source_id)
    .bind(description)
    .bind(tags)
    .bind(frozen)
    .bind(area_id)
    .fetch_one(pool)
    .await?;

    let id: i32 = result.get("id");
    Ok(id)
}

/// Update resource with dynamic field builder
/// NOTE: frozen is NOT updatable -- once frozen=true, the DB trigger blocks all updates.
/// The caller (dpn-api handler) should check frozen first and return 409.
pub async fn update_resource(
    pool: &DbPool,
    id: i32,
    name: Option<&str>,
    description: Option<&str>,
    tags: Option<&serde_json::Value>,
    resource_type: Option<&str>,
    area_id: Option<i32>,
) -> Result<()> {
    let mut query = String::from("UPDATE resources SET ");
    let mut updates = vec![];
    let mut param_idx = 1;

    if name.is_some() {
        updates.push(format!("name = ${}", param_idx));
        param_idx += 1;
    }
    if description.is_some() {
        updates.push(format!("description = ${}", param_idx));
        param_idx += 1;
    }
    if tags.is_some() {
        updates.push(format!("tags = ${}", param_idx));
        param_idx += 1;
    }
    if resource_type.is_some() {
        updates.push(format!("resource_type = ${}", param_idx));
        param_idx += 1;
    }
    if area_id.is_some() {
        updates.push(format!("area_id = ${}", param_idx));
        param_idx += 1;
    }

    if updates.is_empty() {
        return Ok(());
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(", updated_at = NOW() WHERE id = ${}", param_idx));

    let mut q = sqlx::query(&query);

    if let Some(n) = name {
        q = q.bind(n);
    }
    if let Some(d) = description {
        q = q.bind(d);
    }
    if let Some(tg) = tags {
        q = q.bind(tg);
    }
    if let Some(rt) = resource_type {
        q = q.bind(rt);
    }
    if let Some(aid) = area_id {
        q = q.bind(aid);
    }
    q = q.bind(id);

    q.execute(pool).await?;

    Ok(())
}
