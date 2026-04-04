//! Tasks Due Insertion Pipeline
//!
//! Queries tasks due on a given date and inserts them into
//! the Daily Note's "📋 Tasks Due Today" section.
//!
//! Features:
//! - Intelligent merge: doesn't duplicate existing tasks
//! - Creates daily note if missing
//! - Preserves existing section content
//! - Supports dry-run mode

use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::collections::HashSet;
use tracing::info;

use crate::db::{DbPool, memories, tasks};

/// Result of task insertion operation
#[derive(Debug)]
pub struct InsertionResult {
    /// Number of tasks that were due
    pub tasks_due: usize,
    /// Number of tasks actually inserted (excluding duplicates)
    pub tasks_inserted: usize,
    /// Number of tasks skipped (already in note)
    pub tasks_skipped: usize,
    /// Path to the daily note
    pub daily_note_path: String,
    /// Whether the daily note was created
    pub note_created: bool,
    /// The formatted task lines that were inserted
    pub inserted_lines: Vec<String>,
}

/// Insertion mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsertionMode {
    /// Actually perform the insertion
    Execute,
    /// Just preview what would be inserted
    DryRun,
}

/// Section marker for tasks due
const SECTION_HEADER: &str = "## 📋 Tasks Due Today";

/// Insert tasks due on a given date into the corresponding Daily Note
pub async fn insert_tasks_due_into_daily_note(
    pool: &DbPool,
    date: NaiveDate,
    mode: InsertionMode,
) -> Result<InsertionResult> {
    let date_str = date.format("%Y-%m-%d").to_string();
    info!("Processing tasks due on {} (mode: {:?})", date_str, mode);

    // 1. Get open tasks due on this date
    let tasks = tasks::get_open_tasks_due_on(pool, date).await?;
    info!("Found {} open tasks due on {}", tasks.len(), date_str);

    if tasks.is_empty() {
        return Ok(InsertionResult {
            tasks_due: 0,
            tasks_inserted: 0,
            tasks_skipped: 0,
            daily_note_path: format!("Areas/N8K99Notes/Daily Notes/{}.md", date_str),
            note_created: false,
            inserted_lines: vec![],
        });
    }

    // 2. Get or create the daily note
    let (mut note, note_created) = get_or_create_daily_note(pool, date, mode).await?;
    let daily_note_path = note.path.clone();
    info!("Daily note: {} (created: {})", daily_note_path, note_created);

    // 3. Parse existing content to find tasks already in the section
    let content = note.content.as_deref().unwrap_or("");
    let existing_task_ids = extract_existing_task_ids(content);
    info!("Found {} existing task IDs in note", existing_task_ids.len());

    // 4. Filter out tasks that are already present
    let mut tasks_to_insert = Vec::new();
    let mut tasks_skipped = 0;

    for task in &tasks {
        // Check by task_id if available, otherwise by text hash
        let is_duplicate = if let Some(ref task_id) = task.task_id {
            existing_task_ids.contains(task_id)
        } else {
            // Check if the task text already exists in the section
            content.contains(&task.text)
        };

        if is_duplicate {
            info!("Skipping duplicate task: {}", task.text.chars().take(50).collect::<String>());
            tasks_skipped += 1;
        } else {
            tasks_to_insert.push(task);
        }
    }

    info!("{} tasks to insert, {} skipped as duplicates", tasks_to_insert.len(), tasks_skipped);

    // 5. Format the tasks
    let inserted_lines: Vec<String> = tasks_to_insert
        .iter()
        .map(|t| t.to_obsidian_format())
        .collect();

    // 6. Insert into the daily note (if not dry run)
    if mode == InsertionMode::Execute && !inserted_lines.is_empty() {
        let new_content = insert_into_section(content, &inserted_lines);
        
        memories::update_content(pool, note.id, &new_content, note.frontmatter.as_deref())
            .await
            .context("Failed to update daily note content")?;
        
        note.content = Some(new_content);
        info!("Updated daily note with {} new tasks", inserted_lines.len());
    }

    Ok(InsertionResult {
        tasks_due: tasks.len(),
        tasks_inserted: inserted_lines.len(),
        tasks_skipped,
        daily_note_path,
        note_created,
        inserted_lines,
    })
}

/// Get existing daily note or create one if missing
async fn get_or_create_daily_note(
    pool: &DbPool,
    date: NaiveDate,
    mode: InsertionMode,
) -> Result<(memories::Memory, bool)> {
    let date_str = date.format("%Y-%m-%d").to_string();
    
    // Try to find existing daily note
    if let Some(note) = memories::get_daily_note(pool, date).await? {
        return Ok((note, false));
    }

    // Check by path pattern
    let path = format!("Areas/N8K99Notes/Daily Notes/{}.md", date_str);
    if let Ok(note) = memories::get_by_path(pool, &path).await {
        return Ok((note, false));
    }

    // Create new daily note
    let title = format!("{} Daily Log", date_str);
    let content = generate_daily_note_template(&date_str);

    if mode == InsertionMode::Execute {
        let id = memories::create(pool, &path, Some(&title), &content, None)
            .await
            .context("Failed to create daily note")?;
        
        let note = memories::get_by_id(pool, id).await?;
        info!("Created new daily note: {}", path);
        Ok((note, true))
    } else {
        // For dry run, return a mock note
        Ok((
            memories::Memory {
                id: 0,
                path: path.clone(),
                title: Some(title),
                content: Some(content),
                frontmatter: None,
                size_bytes: None,
                note_type: Some("daily".to_string()),
                note_date: Some(date),
                modified_at: None,
                created_at: None,
                compression_tier: "daily".to_string(),
                compressed_from: None,
            },
            true,
        ))
    }
}

/// Generate the daily note template
fn generate_daily_note_template(date_str: &str) -> String {
    format!(
        r#"# 📅 {} Daily Log

## 🎯 What I Did Today

## 📋 Tasks Completed Today

## 📋 Tasks Due Today

## 🚀 New Tasks Created Today

## 📦 Deliverables Created Today

## 🔗 Files Created/Updated Today

## 🚧 Projects Left Midstream

## 💡 Insights & Learnings

## 🔄 Git Commits Today

## 📊 Performance Metrics

- **Energy Level**: 🔋🔋🔋🔋🔋 (1-5 batteries)
- **Focus Quality**: 🎯🎯🎯🎯🎯 (1-5 targets)
"#,
        date_str
    )
}

/// Extract existing task IDs from note content
fn extract_existing_task_ids(content: &str) -> HashSet<String> {
    let mut ids = HashSet::new();
    
    // Look for 🆔 markers
    for line in content.lines() {
        if let Some(idx) = line.find("🆔 ") {
            let after_marker = &line[idx + "🆔 ".len()..];
            // Extract until next space or emoji
            let id: String = after_marker
                .chars()
                .take_while(|c| !c.is_whitespace() && !is_task_emoji(*c))
                .collect();
            if !id.is_empty() {
                ids.insert(id);
            }
        }
    }
    
    ids
}

/// Check if character is a task-related emoji
fn is_task_emoji(c: char) -> bool {
    matches!(c, '⏫' | '🔺' | '🔼' | '🔽' | '⏬' | '📅' | '⏳' | '🛫' | '➕' | '🔁' | '⛔' | '#')
}

/// Insert task lines into the "Tasks Due Today" section
fn insert_into_section(content: &str, new_lines: &[String]) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut found_section = false;
    let mut inserted = false;

    for (i, line) in lines.iter().enumerate() {
        result.push(line.to_string());

        // Found the section header
        if line.trim() == SECTION_HEADER || line.trim() == "## 📋 Tasks Due Today" {
            found_section = true;
            continue;
        }

        // We're in the section - find where to insert
        if found_section && !inserted {
            // Check if next line is empty or a new section
            let next_is_section = i + 1 < lines.len() && lines[i + 1].starts_with("## ");
            let current_is_empty = line.trim().is_empty();

            // Insert after empty line following header, or at end of section
            if current_is_empty || next_is_section {
                // Add new tasks
                for task_line in new_lines {
                    result.push(task_line.clone());
                }
                // Add blank line if we're before a section
                if next_is_section && !new_lines.is_empty() {
                    result.push(String::new());
                }
                inserted = true;
            }
        }
    }

    // If we found the section but never inserted (section was at end of file)
    if found_section && !inserted {
        result.push(String::new());
        for task_line in new_lines {
            result.push(task_line.clone());
        }
    }

    // If we never found the section, add it
    if !found_section {
        result.push(String::new());
        result.push(SECTION_HEADER.to_string());
        result.push(String::new());
        for task_line in new_lines {
            result.push(task_line.clone());
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_task_ids() {
        let content = r#"
## Tasks Due Today

- [ ] Test task ⏫ 🆔 task-001 📅 2026-02-24
- [ ] Another task 🆔 task-002 #work
"#;
        let ids = extract_existing_task_ids(content);
        assert!(ids.contains("task-001"));
        assert!(ids.contains("task-002"));
    }

    #[test]
    fn test_insert_into_section() {
        let content = r#"# Daily Log

## 📋 Tasks Due Today

## Other Section
"#;
        let new_lines = vec![
            "- [ ] New task 📅 2026-02-24".to_string(),
        ];
        
        let result = insert_into_section(content, &new_lines);
        assert!(result.contains("- [ ] New task 📅 2026-02-24"));
    }

    #[test]
    fn test_insert_without_section() {
        let content = r#"# Daily Log

## Other Section
"#;
        let new_lines = vec![
            "- [ ] Task 📅 2026-02-24".to_string(),
        ];
        
        let result = insert_into_section(content, &new_lines);
        assert!(result.contains("## 📋 Tasks Due Today"));
        assert!(result.contains("- [ ] Task"));
    }
}
