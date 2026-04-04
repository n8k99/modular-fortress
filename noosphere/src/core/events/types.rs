//! Event type definitions

use serde::{Deserialize, Serialize};

/// Event type icons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Calendar,
    Deadline,
    Goal,
    Reminder,
    Health,
    Music,
    Gaming,
    Maintenance,
    Theatre,
    Call,
}

impl EventType {
    pub fn from_icon(icon: &str) -> Option<Self> {
        match icon {
            "📆" => Some(EventType::Calendar),
            "📋" => Some(EventType::Deadline),
            "🎯" => Some(EventType::Goal),
            "🔔" => Some(EventType::Reminder),
            "🏥" => Some(EventType::Health),
            "🎸" => Some(EventType::Music),
            "🎲" => Some(EventType::Gaming),
            "🔧" => Some(EventType::Maintenance),
            "🎭" => Some(EventType::Theatre),
            "📞" => Some(EventType::Call),
            _ => None,
        }
    }

    pub fn to_icon(&self) -> &'static str {
        match self {
            EventType::Calendar => "📆",
            EventType::Deadline => "📋",
            EventType::Goal => "🎯",
            EventType::Reminder => "🔔",
            EventType::Health => "🏥",
            EventType::Music => "🎸",
            EventType::Gaming => "🎲",
            EventType::Maintenance => "🔧",
            EventType::Theatre => "🎭",
            EventType::Call => "📞",
        }
    }
}

/// Parsed event representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEvent {
    pub id: String,
    pub icon: String,
    pub event_type: EventType,
    pub title: String,
    pub date: String,
    pub time: Option<String>,
    pub duration: Option<String>,
    pub daily_note: Option<String>,
    pub recurrence: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl ParsedEvent {
    /// Convert to SQL parameters for upsert
    pub fn to_sql_params(&self) -> (
        String, String, String, String, String,
        Option<String>, Option<String>, Option<String>, Option<String>,
    ) {
        (
            self.id.clone(),
            self.icon.clone(),
            format!("{:?}", self.event_type).to_lowercase(),
            self.title.clone(),
            self.date.clone(),
            self.time.clone(),
            self.duration.clone(),
            self.daily_note.clone(),
            self.recurrence.clone(),
        )
    }
}
