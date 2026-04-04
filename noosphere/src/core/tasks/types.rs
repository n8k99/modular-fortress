//! Task type definitions

use serde::{Deserialize, Serialize};

/// Task status mapping
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Todo,
    Done,
    InProgress,
    NeedsReview,
}

impl TaskStatus {
    pub fn from_char(c: char) -> Self {
        match c {
            'x' | 'X' => TaskStatus::Done,
            '/' | '-' => TaskStatus::InProgress,
            '?' => TaskStatus::NeedsReview,
            _ => TaskStatus::Todo,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            TaskStatus::Done => 'x',
            TaskStatus::InProgress => '/',
            TaskStatus::NeedsReview => '?',
            TaskStatus::Todo => ' ',
        }
    }
}

/// Task priority level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    High,
    Medium,
    Low,
}

impl TaskPriority {
    pub fn from_emoji(emoji: &str) -> Option<Self> {
        match emoji {
            "⏫" => Some(TaskPriority::High),
            "🔼" => Some(TaskPriority::Medium),
            "🔽" => Some(TaskPriority::Low),
            _ => None,
        }
    }

    pub fn to_emoji(&self) -> &'static str {
        match self {
            TaskPriority::High => "⏫",
            TaskPriority::Medium => "🔼",
            TaskPriority::Low => "🔽",
        }
    }
}

/// Parsed task representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub doc_id: i32,
    pub doc_path: String,
    pub doc_title: String,
    pub line_number: usize,
    pub indent: usize,
    pub status: TaskStatus,
    pub text: String,
    pub assignee: Option<String>,
    pub priority: Option<TaskPriority>,
    pub task_id: Option<String>,
    pub dependencies: Option<String>,
    pub created_date: Option<String>,
    pub scheduled_date: Option<String>,
    pub due_date: Option<String>,
    pub completed_date: Option<String>,
    pub recurrence: Option<String>,
    pub tags: Option<String>,
    pub raw_line: String,
}

impl Task {
    /// Convert to SQL parameters for insertion
    pub fn to_sql_params(&self) -> (
        i32, String, String, i32, i32, String, String,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<String>, Option<String>, String
    ) {
        (
            self.doc_id,
            self.doc_path.clone(),
            self.doc_title.clone(),
            self.line_number as i32,
            self.indent as i32,
            match self.status {
                TaskStatus::Todo => "todo",
                TaskStatus::Done => "done",
                TaskStatus::InProgress => "in-progress",
                TaskStatus::NeedsReview => "needs-review",
            }.to_string(),
            self.text.clone(),
            self.assignee.clone(),
            self.priority.as_ref().map(|p| match p {
                TaskPriority::High => "high",
                TaskPriority::Medium => "medium",
                TaskPriority::Low => "low",
            }.to_string()),
            self.task_id.clone(),
            self.dependencies.clone(),
            self.created_date.clone(),
            self.scheduled_date.clone(),
            self.due_date.clone(),
            self.completed_date.clone(),
            self.recurrence.clone(),
            self.tags.clone(),
            self.raw_line.clone(),
        )
    }
}
