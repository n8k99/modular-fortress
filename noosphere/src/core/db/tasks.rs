//! tasks table access and formatting
//!
//! Query tasks by due date and format in Obsidian Tasks format:
//! `- [ ] Task ⏫ 🆔 id ⛔ dep ➕ created ⏰ scheduled 📅 due 🔁 recur #tag`

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::DbPool;

/// Task record from database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub doc_id: Option<i32>,
    pub doc_path: String,
    pub doc_title: Option<String>,
    pub line_number: i32,
    pub indent: Option<i32>,
    pub status: String,
    pub text: String,
    pub assignee: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub notify_date: Option<String>,
    pub completed_date: Option<String>,
    pub raw_line: String,
    pub priority: Option<String>,
    pub task_id: Option<String>,
    pub dependencies: Option<String>,
    pub created_date: Option<String>,
    pub scheduled_date: Option<String>,
    pub recurrence: Option<String>,
    pub tags: Option<String>,
}

impl Task {
    /// Format task in Obsidian Tasks format
    /// `- [ ] Task ⏫ 🆔 id ⛔ dep ➕ created ⏰ scheduled 📅 due 🔁 recur #tag`
    pub fn to_obsidian_format(&self) -> String {
        let mut parts = Vec::new();

        // Status checkbox
        let checkbox = match self.status.as_str() {
            "x" | "X" | "done" => "- [x]",
            "-" | "cancelled" => "- [-]",
            "/" | "in_progress" => "- [/]",
            ">" | "deferred" => "- [>]",
            "<" | "scheduled" => "- [<]",
            "!" | "important" => "- [!]",
            "?" | "question" => "- [?]",
            _ => "- [ ]",
        };
        parts.push(checkbox.to_string());

        // Task text
        parts.push(self.text.clone());

        // Priority emoji
        if let Some(ref priority) = self.priority {
            let priority_emoji = match priority.to_lowercase().as_str() {
                "highest" => "🔺",
                "high" => "⏫",
                "medium" => "🔼",
                "low" => "🔽",
                "lowest" => "⏬",
                _ => "",
            };
            if !priority_emoji.is_empty() {
                parts.push(priority_emoji.to_string());
            }
        }

        // Task ID
        if let Some(ref task_id) = self.task_id {
            if !task_id.is_empty() {
                parts.push(format!("🆔 {}", task_id));
            }
        }

        // Dependencies
        if let Some(ref deps) = self.dependencies {
            if !deps.is_empty() {
                parts.push(format!("⛔ {}", deps));
            }
        }

        // Created date
        if let Some(ref created) = self.created_date {
            if !created.is_empty() {
                parts.push(format!("➕ {}", created));
            }
        }

        // Scheduled date
        if let Some(ref scheduled) = self.scheduled_date {
            if !scheduled.is_empty() {
                parts.push(format!("⏳ {}", scheduled));
            }
        }

        // Start date (notify in our schema)
        if let Some(ref start) = self.start_date {
            if !start.is_empty() {
                parts.push(format!("🛫 {}", start));
            }
        }

        // Due date
        if let Some(ref due) = self.due_date {
            if !due.is_empty() {
                parts.push(format!("📅 {}", due));
            }
        }

        // Recurrence
        if let Some(ref recur) = self.recurrence {
            if !recur.is_empty() {
                parts.push(format!("🔁 {}", recur));
            }
        }

        // Tags
        if let Some(ref tags) = self.tags {
            for tag in tags.split(',').map(|t| t.trim()) {
                if !tag.is_empty() {
                    if tag.starts_with('#') {
                        parts.push(tag.to_string());
                    } else {
                        parts.push(format!("#{}", tag));
                    }
                }
            }
        }

        parts.join(" ")
    }

    /// Check if task is incomplete (not done/cancelled)
    pub fn is_open(&self) -> bool {
        !matches!(self.status.as_str(), "x" | "X" | "done" | "-" | "cancelled")
    }
}

/// Get tasks due on a specific date
pub async fn get_tasks_due_on(pool: &DbPool, date: NaiveDate) -> Result<Vec<Task>> {
    let date_str = date.format("%Y-%m-%d").to_string();
    
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, doc_id, doc_path, doc_title, line_number, indent, status,
               text, assignee, start_date, due_date, notify_date, completed_date,
               raw_line, priority, task_id, dependencies, created_date,
               scheduled_date, recurrence, tags
        FROM tasks
        WHERE due_date = $1
        ORDER BY 
            CASE priority
                WHEN 'highest' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                WHEN 'lowest' THEN 5
                ELSE 3
            END,
            id
        "#,
    )
    .bind(&date_str)
    .fetch_all(pool)
    .await?;
    
    Ok(tasks)
}

/// Get open (incomplete) tasks due on a specific date
pub async fn get_open_tasks_due_on(pool: &DbPool, date: NaiveDate) -> Result<Vec<Task>> {
    let date_str = date.format("%Y-%m-%d").to_string();
    
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, doc_id, doc_path, doc_title, line_number, indent, status,
               text, assignee, start_date, due_date, notify_date, completed_date,
               raw_line, priority, task_id, dependencies, created_date,
               scheduled_date, recurrence, tags
        FROM tasks
        WHERE due_date = $1
          AND status NOT IN ('x', 'X', 'done', '-', 'cancelled')
        ORDER BY 
            CASE priority
                WHEN 'highest' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                WHEN 'lowest' THEN 5
                ELSE 3
            END,
            id
        "#,
    )
    .bind(&date_str)
    .fetch_all(pool)
    .await?;
    
    Ok(tasks)
}

/// Get all open tasks (for dashboard/overview)
pub async fn get_all_open_tasks(pool: &DbPool) -> Result<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, doc_id, doc_path, doc_title, line_number, indent, status,
               text, assignee, start_date, due_date, notify_date, completed_date,
               raw_line, priority, task_id, dependencies, created_date,
               scheduled_date, recurrence, tags
        FROM tasks
        WHERE status NOT IN ('x', 'X', 'done', '-', 'cancelled')
        ORDER BY due_date NULLS LAST, 
            CASE priority
                WHEN 'highest' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                WHEN 'lowest' THEN 5
                ELSE 3
            END,
            id
        "#,
    )
    .fetch_all(pool)
    .await?;
    
    Ok(tasks)
}

/// Get overdue open tasks (due before today)
pub async fn get_overdue_tasks(pool: &DbPool, today: NaiveDate) -> Result<Vec<Task>> {
    let today_str = today.format("%Y-%m-%d").to_string();
    
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, doc_id, doc_path, doc_title, line_number, indent, status,
               text, assignee, start_date, due_date, notify_date, completed_date,
               raw_line, priority, task_id, dependencies, created_date,
               scheduled_date, recurrence, tags
        FROM tasks
        WHERE due_date < $1
          AND due_date IS NOT NULL
          AND status NOT IN ('x', 'X', 'done', '-', 'cancelled')
        ORDER BY due_date, 
            CASE priority
                WHEN 'highest' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                WHEN 'lowest' THEN 5
                ELSE 3
            END,
            id
        "#,
    )
    .bind(&today_str)
    .fetch_all(pool)
    .await?;
    
    Ok(tasks)
}

/// Format multiple tasks for insertion into a Daily Note section
pub fn format_tasks_for_daily_note(tasks: &[Task]) -> String {
    if tasks.is_empty() {
        return String::new();
    }
    
    tasks
        .iter()
        .map(|t| t.to_obsidian_format())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_formatting() {
        let task = Task {
            id: 1,
            doc_id: None,
            doc_path: "test.md".to_string(),
            doc_title: None,
            line_number: 1,
            indent: Some(0),
            status: "open".to_string(),
            text: "Test task".to_string(),
            assignee: None,
            start_date: None,
            due_date: Some("2026-02-24".to_string()),
            notify_date: None,
            completed_date: None,
            raw_line: "- [ ] Test task".to_string(),
            priority: Some("high".to_string()),
            task_id: Some("task-001".to_string()),
            dependencies: None,
            created_date: Some("2026-02-23".to_string()),
            scheduled_date: None,
            recurrence: None,
            tags: Some("work,urgent".to_string()),
        };

        let formatted = task.to_obsidian_format();
        assert!(formatted.contains("- [ ]"));
        assert!(formatted.contains("Test task"));
        assert!(formatted.contains("⏫")); // high priority
        assert!(formatted.contains("🆔 task-001"));
        assert!(formatted.contains("📅 2026-02-24"));
        assert!(formatted.contains("#work"));
        assert!(formatted.contains("#urgent"));
    }

    #[test]
    fn test_task_is_open() {
        let mut task = Task {
            id: 1,
            doc_id: None,
            doc_path: "test.md".to_string(),
            doc_title: None,
            line_number: 1,
            indent: Some(0),
            status: "open".to_string(),
            text: "Test".to_string(),
            assignee: None,
            start_date: None,
            due_date: None,
            notify_date: None,
            completed_date: None,
            raw_line: "".to_string(),
            priority: None,
            task_id: None,
            dependencies: None,
            created_date: None,
            scheduled_date: None,
            recurrence: None,
            tags: None,
        };

        assert!(task.is_open());
        
        task.status = "x".to_string();
        assert!(!task.is_open());
        
        task.status = "cancelled".to_string();
        assert!(!task.is_open());
    }
}
