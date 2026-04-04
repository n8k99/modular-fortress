//! templates table access and CRUD operations
//!
//! Template storage with .dpn body expressions and version history tracking.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

use super::DbPool;

/// Template record from database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub body: String,
    pub parameters: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub version: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Template history record for version tracking
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TemplateHistory {
    pub id: i32,
    pub template_id: i32,
    pub version: i32,
    pub body: String,
    pub parameters: Option<serde_json::Value>,
    pub changed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// List all templates, ordered by name
pub async fn list_templates(pool: &DbPool) -> Result<Vec<Template>> {
    let templates = sqlx::query_as::<_, Template>(
        r#"
        SELECT id, name, slug, category, description, body, parameters,
               metadata, version, created_at, updated_at
        FROM templates
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(templates)
}

/// Get template by ID
pub async fn get_template_by_id(pool: &DbPool, id: i32) -> Result<Option<Template>> {
    let template = sqlx::query_as::<_, Template>(
        r#"
        SELECT id, name, slug, category, description, body, parameters,
               metadata, version, created_at, updated_at
        FROM templates
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(template)
}

/// Create a new template
pub async fn create_template(
    pool: &DbPool,
    name: &str,
    slug: &str,
    category: Option<&str>,
    description: Option<&str>,
    body: &str,
    parameters: Option<&serde_json::Value>,
) -> Result<i32> {
    let result = sqlx::query(
        r#"
        INSERT INTO templates (name, slug, category, description, body, parameters)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(slug)
    .bind(category)
    .bind(description)
    .bind(body)
    .bind(parameters)
    .fetch_one(pool)
    .await?;

    let id: i32 = result.get("id");
    Ok(id)
}

/// Update template with dynamic field builder
/// The DB trigger handles version history automatically when body changes (D-08).
pub async fn update_template(
    pool: &DbPool,
    id: i32,
    name: Option<&str>,
    category: Option<&str>,
    description: Option<&str>,
    body: Option<&str>,
    parameters: Option<&serde_json::Value>,
) -> Result<()> {
    let mut query = String::from("UPDATE templates SET ");
    let mut updates = vec![];
    let mut param_idx = 1;

    if name.is_some() {
        updates.push(format!("name = ${}", param_idx));
        param_idx += 1;
    }
    if category.is_some() {
        updates.push(format!("category = ${}", param_idx));
        param_idx += 1;
    }
    if description.is_some() {
        updates.push(format!("description = ${}", param_idx));
        param_idx += 1;
    }
    if body.is_some() {
        updates.push(format!("body = ${}", param_idx));
        param_idx += 1;
    }
    if parameters.is_some() {
        updates.push(format!("parameters = ${}", param_idx));
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
    if let Some(c) = category {
        q = q.bind(c);
    }
    if let Some(d) = description {
        q = q.bind(d);
    }
    if let Some(b) = body {
        q = q.bind(b);
    }
    if let Some(p) = parameters {
        q = q.bind(p);
    }
    q = q.bind(id);

    q.execute(pool).await?;

    Ok(())
}

/// Get version history for a template
pub async fn get_template_history(pool: &DbPool, template_id: i32) -> Result<Vec<TemplateHistory>> {
    let history = sqlx::query_as::<_, TemplateHistory>(
        r#"
        SELECT id, template_id, version, body, parameters, changed_at
        FROM templates_history
        WHERE template_id = $1
        ORDER BY version DESC
        "#,
    )
    .bind(template_id)
    .fetch_all(pool)
    .await?;

    Ok(history)
}
