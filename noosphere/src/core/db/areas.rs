//! areas table access and CRUD operations
//!
//! Area management for PARAT five-pillar architecture.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

use super::DbPool;

/// Area record from database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// List all areas, ordered by name
pub async fn list_areas(pool: &DbPool) -> Result<Vec<Area>> {
    let areas = sqlx::query_as::<_, Area>(
        r#"
        SELECT id, name, slug, description, owner, status, metadata, created_at, updated_at
        FROM areas
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(areas)
}

/// Get area by ID
pub async fn get_area_by_id(pool: &DbPool, id: i32) -> Result<Option<Area>> {
    let area = sqlx::query_as::<_, Area>(
        r#"
        SELECT id, name, slug, description, owner, status, metadata, created_at, updated_at
        FROM areas
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(area)
}

/// Create a new area
pub async fn create_area(
    pool: &DbPool,
    name: &str,
    slug: &str,
    description: Option<&str>,
    owner: Option<&str>,
    status: &str,
) -> Result<i32> {
    let result = sqlx::query(
        r#"
        INSERT INTO areas (name, slug, description, owner, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(slug)
    .bind(description)
    .bind(owner)
    .bind(status)
    .fetch_one(pool)
    .await?;

    let id: i32 = result.get("id");
    Ok(id)
}

/// Update area with dynamic field builder
pub async fn update_area(
    pool: &DbPool,
    id: i32,
    name: Option<&str>,
    description: Option<&str>,
    owner: Option<&str>,
    status: Option<&str>,
) -> Result<()> {
    let mut query = String::from("UPDATE areas SET ");
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
    if owner.is_some() {
        updates.push(format!("owner = ${}", param_idx));
        param_idx += 1;
    }
    if status.is_some() {
        updates.push(format!("status = ${}", param_idx));
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
    if let Some(o) = owner {
        q = q.bind(o);
    }
    if let Some(s) = status {
        q = q.bind(s);
    }
    q = q.bind(id);

    q.execute(pool).await?;

    Ok(())
}
