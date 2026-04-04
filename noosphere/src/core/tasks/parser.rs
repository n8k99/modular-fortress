//! Obsidian Tasks format parser
//!
//! Parses tasks in the format:
//! - [ ] Task description ⏫ 🆔 id123 ⛔ dep456 ➕ 2025-06-28 ⏰ 2025-06-29 📅 2025-06-30 🔁 every week #tag

use regex::Regex;
use once_cell::sync::Lazy;

use super::types::{Task, TaskStatus, TaskPriority};

// Compiled regex patterns (initialized once)
static TASK_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\s*)- \[([ xX/\-?])\]\s*(.+)$").unwrap()
});

static DATE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap()
});

static PRIORITY_EMOJIS: &[&str] = &["⏫", "🔼", "🔽"];
static TASK_ID_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"🆔\s*(\S+)").unwrap());
static DEPENDENCIES_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"⛔\s*(\S+)").unwrap());
static CREATED_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"➕\s*(\d{4}-\d{2}-\d{2})").unwrap());
static SCHEDULED_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"⏰\s*(\d{4}-\d{2}-\d{2})").unwrap());
static DUE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"📅\s*(\d{4}-\d{2}-\d{2})").unwrap());
static COMPLETED_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"✅\s*(\d{4}-\d{2}-\d{2})").unwrap());
static RECURRENCE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"🔁\s*([^#⏫🔼🔽🆔⛔➕⏰📅✅]+)").unwrap()
});
static TAG_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"#(\w+)").unwrap());
static ASSIGNEE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"@\[\[([^\]]+)\]\]|@(\w+)").unwrap());

// Legacy format patterns
static LEGACY_START_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"📅start:(\d{4}-\d{2}-\d{2})").unwrap());
static LEGACY_DUE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"⏰due:(\d{4}-\d{2}-\d{2})").unwrap());
static LEGACY_DONE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"✅done:(\d{4}-\d{2}-\d{2})").unwrap());

/// Parse a single line for task data
///
/// Returns `Some(Task)` if the line is a valid task, `None` otherwise.
pub fn parse_task_line(
    line: &str,
    line_number: usize,
    doc_id: i32,
    doc_path: &str,
    doc_title: &str,
) -> Option<Task> {
    let captures = TASK_PATTERN.captures(line)?;

    let indent = captures.get(1)?.as_str().len();
    let status_char = captures.get(2)?.as_str().chars().next()?;
    let mut text = captures.get(3)?.as_str().to_string();
    let raw_line = line.to_string();

    let status = TaskStatus::from_char(status_char);

    // Extract priority
    let mut priority = None;
    for emoji in PRIORITY_EMOJIS {
        if text.contains(emoji) {
            priority = TaskPriority::from_emoji(emoji);
            text = text.replace(emoji, "");
            break;
        }
    }

    // Extract task ID
    let task_id = TASK_ID_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    if let Some(ref id) = task_id {
        text = text.replace(&format!("🆔 {}", id), "");
    }

    // Extract dependencies
    let dependencies = DEPENDENCIES_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    if let Some(ref deps) = dependencies {
        text = text.replace(&format!("⛔ {}", deps), "");
    }

    // Extract created date
    let created_date = CREATED_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    if let Some(ref date) = created_date {
        text = text.replace(&format!("➕ {}", date), "");
    }

    // Extract scheduled date
    let mut scheduled_date = SCHEDULED_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    if let Some(ref date) = scheduled_date {
        text = text.replace(&format!("⏰ {}", date), "");
    }

    // Extract due date
    let mut due_date = DUE_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    if let Some(ref date) = due_date {
        text = text.replace(&format!("📅 {}", date), "");
    }

    // Extract completed date
    let mut completed_date = COMPLETED_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());
    if let Some(ref date) = completed_date {
        text = text.replace(&format!("✅ {}", date), "");
    }

    // Extract recurrence
    let recurrence = RECURRENCE_PATTERN.captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string());
    if let Some(ref rec) = recurrence {
        text = text.replace(&format!("🔁 {}", rec), "");
    }

    // Extract tags
    let mut tags = Vec::new();
    for capture in TAG_PATTERN.captures_iter(&text) {
        if let Some(tag) = capture.get(1) {
            tags.push(tag.as_str().to_string());
        }
    }
    let tags_str = if !tags.is_empty() {
        Some(tags.join(","))
    } else {
        None
    };
    // Remove tags from text
    text = TAG_PATTERN.replace_all(&text, "").to_string();

    // Extract assignee
    let assignee = ASSIGNEE_PATTERN.captures(&text)
        .and_then(|c| c.get(1).or_else(|| c.get(2)))
        .map(|m| m.as_str().to_string());
    if assignee.is_some() {
        text = ASSIGNEE_PATTERN.replace(&text, "").to_string();
    }

    // Legacy date formats (for backwards compatibility)
    if scheduled_date.is_none() {
        if let Some(captures) = LEGACY_START_PATTERN.captures(&text) {
            if let Some(date) = captures.get(1) {
                scheduled_date = Some(date.as_str().to_string());
                text = text.replace(&format!("📅start:{}", date.as_str()), "");
            }
        }
    }

    if due_date.is_none() {
        if let Some(captures) = LEGACY_DUE_PATTERN.captures(&text) {
            if let Some(date) = captures.get(1) {
                due_date = Some(date.as_str().to_string());
                text = text.replace(&format!("⏰due:{}", date.as_str()), "");
            }
        }
    }

    if completed_date.is_none() {
        if let Some(captures) = LEGACY_DONE_PATTERN.captures(&text) {
            if let Some(date) = captures.get(1) {
                completed_date = Some(date.as_str().to_string());
                text = text.replace(&format!("✅done:{}", date.as_str()), "");
            }
        }
    }

    // Clean up extra whitespace
    text = text.split_whitespace().collect::<Vec<_>>().join(" ");

    Some(Task {
        doc_id,
        doc_path: doc_path.to_string(),
        doc_title: doc_title.to_string(),
        line_number,
        indent,
        status,
        text,
        assignee,
        priority,
        task_id,
        dependencies,
        created_date,
        scheduled_date,
        due_date,
        completed_date,
        recurrence,
        tags: tags_str,
        raw_line,
    })
}

/// Parse all tasks from document content
pub fn parse_document(
    content: &str,
    doc_id: i32,
    doc_path: &str,
    doc_title: &str,
) -> Vec<Task> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| parse_task_line(line, i, doc_id, doc_path, doc_title))
        .collect()
}

/// Serialize a task back to Obsidian Tasks format
pub fn serialize_task(task: &Task) -> String {
    let status_char = task.status.to_char();
    let indent = " ".repeat(task.indent);
    let mut line = format!("{}- [{}] {}", indent, status_char, task.text);

    // Add priority
    if let Some(ref priority) = task.priority {
        line.push(' ');
        line.push_str(priority.to_emoji());
    }

    // Add task ID
    if let Some(ref task_id) = task.task_id {
        line.push_str(&format!(" 🆔 {}", task_id));
    }

    // Add dependencies
    if let Some(ref deps) = task.dependencies {
        line.push_str(&format!(" ⛔ {}", deps));
    }

    // Add dates
    if let Some(ref date) = task.created_date {
        line.push_str(&format!(" ➕ {}", date));
    }
    if let Some(ref date) = task.scheduled_date {
        line.push_str(&format!(" ⏰ {}", date));
    }
    if let Some(ref date) = task.due_date {
        line.push_str(&format!(" 📅 {}", date));
    }
    if let Some(ref date) = task.completed_date {
        line.push_str(&format!(" ✅ {}", date));
    }

    // Add recurrence
    if let Some(ref rec) = task.recurrence {
        line.push_str(&format!(" 🔁 {}", rec));
    }

    // Add tags
    if let Some(ref tags) = task.tags {
        for tag in tags.split(',') {
            line.push_str(&format!(" #{}", tag));
        }
    }

    // Add assignee
    if let Some(ref assignee) = task.assignee {
        line.push_str(&format!(" @[[{}]]", assignee));
    }

    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_task() {
        let line = "- [ ] Simple task";
        let task = parse_task_line(line, 0, 1, "test.md", "Test").unwrap();
        assert_eq!(task.text, "Simple task");
        assert_eq!(task.status, TaskStatus::Todo);
    }

    #[test]
    fn test_parse_task_with_priority() {
        let line = "- [ ] Important task ⏫";
        let task = parse_task_line(line, 0, 1, "test.md", "Test").unwrap();
        assert_eq!(task.text, "Important task");
        assert_eq!(task.priority, Some(TaskPriority::High));
    }

    #[test]
    fn test_parse_task_with_dates() {
        let line = "- [ ] Task ➕ 2025-01-01 📅 2025-01-15";
        let task = parse_task_line(line, 0, 1, "test.md", "Test").unwrap();
        assert_eq!(task.created_date, Some("2025-01-01".to_string()));
        assert_eq!(task.due_date, Some("2025-01-15".to_string()));
    }

    #[test]
    fn test_serialize_task() {
        let task = Task {
            doc_id: 1,
            doc_path: "test.md".to_string(),
            doc_title: "Test".to_string(),
            line_number: 0,
            indent: 0,
            status: TaskStatus::Todo,
            text: "Test task".to_string(),
            assignee: None,
            priority: Some(TaskPriority::High),
            task_id: Some("task-1".to_string()),
            dependencies: None,
            created_date: None,
            scheduled_date: None,
            due_date: Some("2025-01-15".to_string()),
            completed_date: None,
            recurrence: None,
            tags: Some("work,important".to_string()),
            raw_line: "".to_string(),
        };

        let serialized = serialize_task(&task);
        assert!(serialized.contains("Test task"));
        assert!(serialized.contains("⏫"));
        assert!(serialized.contains("🆔 task-1"));
        assert!(serialized.contains("📅 2025-01-15"));
        assert!(serialized.contains("#work"));
        assert!(serialized.contains("#important"));
    }
}
