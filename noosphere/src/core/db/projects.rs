//! projects table access and CRUD operations
//!
//! Project management with status tracking and ownership.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};

use super::DbPool;

/// Project record from database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub status: String,                // active, paused, completed, archived
    pub description: Option<String>,
    pub owner: Option<String>,
    pub schedule: Option<serde_json::Value>,
    pub lifestage: String,             // Seed, Sapling, Tree, Harvest (NOT NULL)
    pub area_id: Option<i32>,          // FK to areas table (nullable)
}

impl Project {
    /// Get emoji for project status
    pub fn status_emoji(&self) -> &str {
        match self.status.as_str() {
            "active" => "🟢",
            "paused" => "🟡",
            "completed" => "✅",
            "archived" => "📦",
            _ => "⚪",
        }
    }

    /// Get display text (one-line summary)
    pub fn display_text(&self) -> String {
        format!("{} {}", self.status_emoji(), self.name)
    }
}

/// List all projects, ordered by status priority
pub async fn list_projects(pool: &DbPool) -> Result<Vec<Project>> {
    let projects = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, slug, status, description, owner, schedule, lifestage, area_id
        FROM projects
        ORDER BY
            CASE status
                WHEN 'active' THEN 1
                WHEN 'paused' THEN 2
                WHEN 'completed' THEN 3
                WHEN 'archived' THEN 4
                ELSE 5
            END,
            name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

/// List projects by status
pub async fn list_projects_by_status(pool: &DbPool, status: &str) -> Result<Vec<Project>> {
    let projects = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, slug, status, description, owner, schedule, lifestage, area_id
        FROM projects
        WHERE status = $1
        ORDER BY name
        "#,
    )
    .bind(status)
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

/// Get active projects
pub async fn list_active_projects(pool: &DbPool) -> Result<Vec<Project>> {
    list_projects_by_status(pool, "active").await
}

/// Get project by ID
pub async fn get_project_by_id(pool: &DbPool, id: i32) -> Result<Option<Project>> {
    let project = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, slug, status, description, owner, schedule, lifestage, area_id
        FROM projects
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(project)
}

/// Get project by slug
pub async fn get_project_by_slug(pool: &DbPool, slug: &str) -> Result<Option<Project>> {
    let project = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, slug, status, description, owner, schedule, lifestage, area_id
        FROM projects
        WHERE slug = $1
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(project)
}

/// Create a new project
pub async fn create_project(
    pool: &DbPool,
    name: &str,
    slug: &str,
    status: &str,
    description: Option<&str>,
    owner: Option<&str>,
    lifestage: &str,
    area_id: Option<i32>,
) -> Result<i32> {
    let result = sqlx::query(
        r#"
        INSERT INTO projects (name, slug, status, description, owner, lifestage, area_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(slug)
    .bind(status)
    .bind(description)
    .bind(owner)
    .bind(lifestage)
    .bind(area_id)
    .fetch_one(pool)
    .await?;

    let id: i32 = result.get("id");
    Ok(id)
}

/// Update project status
pub async fn update_project_status(pool: &DbPool, id: i32, status: &str) -> Result<()> {
    sqlx::query("UPDATE projects SET status = $1 WHERE id = $2")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Update project
pub async fn update_project(
    pool: &DbPool,
    id: i32,
    name: Option<&str>,
    description: Option<&str>,
    owner: Option<&str>,
    status: Option<&str>,
    schedule: Option<&serde_json::Value>,
    lifestage: Option<&str>,
    area_id: Option<i32>,
) -> Result<()> {
    let mut query = String::from("UPDATE projects SET ");
    let mut updates = vec![];
    let mut param_idx = 1;

    if let Some(_) = name {
        updates.push(format!("name = ${}", param_idx));
        param_idx += 1;
    }
    if let Some(_) = description {
        updates.push(format!("description = ${}", param_idx));
        param_idx += 1;
    }
    if let Some(_) = owner {
        updates.push(format!("owner = ${}", param_idx));
        param_idx += 1;
    }
    if let Some(_) = status {
        updates.push(format!("status = ${}", param_idx));
        param_idx += 1;
    }
    if let Some(_) = schedule {
        updates.push(format!("schedule = ${}", param_idx));
        param_idx += 1;
    }
    if let Some(_) = lifestage {
        updates.push(format!("lifestage = ${}", param_idx));
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
    query.push_str(&format!(" WHERE id = ${}", param_idx));

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
    if let Some(sch) = schedule {
        q = q.bind(sch);
    }
    if let Some(ls) = lifestage {
        q = q.bind(ls);
    }
    if let Some(aid) = area_id {
        q = q.bind(aid);
    }
    q = q.bind(id);

    q.execute(pool).await?;

    Ok(())
}

/// Delete a project
pub async fn delete_project(pool: &DbPool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_status_emoji() {
        let mut project = Project {
            id: 1,
            name: "Test Project".to_string(),
            slug: "test-project".to_string(),
            status: "active".to_string(),
            description: None,
            owner: None,
            schedule: None,
            lifestage: "Tree".to_string(),
            area_id: None,
        };

        assert_eq!(project.status_emoji(), "🟢");

        project.status = "paused".to_string();
        assert_eq!(project.status_emoji(), "🟡");

        project.status = "completed".to_string();
        assert_eq!(project.status_emoji(), "✅");

        project.status = "archived".to_string();
        assert_eq!(project.status_emoji(), "📦");
    }

    #[test]
    fn test_project_display_text() {
        let project = Project {
            id: 1,
            name: "DragonPunk".to_string(),
            slug: "dragonpunk".to_string(),
            status: "active".to_string(),
            description: Some("DPN Suite".to_string()),
            owner: Some("Nova".to_string()),
            schedule: None,
            lifestage: "Harvest".to_string(),
            area_id: Some(2),
        };

        let display = project.display_text();
        assert!(display.contains("🟢"));
        assert!(display.contains("DragonPunk"));
    }
}
