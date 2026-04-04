//! Config loading for webhook notifications

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use super::NotifyError;

/// Root config structure (partial - just what we need)
#[derive(Debug, Deserialize)]
pub struct NotifyConfig {
    pub discord: DiscordConfig,
}

#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub webhooks: HashMap<String, WebhookEntry>,
}

/// Webhook entry can be either a direct URL or a map of staff -> URL
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WebhookEntry {
    /// Direct webhook URL (e.g., "combat": "https://...")
    Direct(String),
    /// Staff map (e.g., "tech_dev_office": { "SamirKhanna": "https://...", ... })
    StaffMap(HashMap<String, String>),
}

impl WebhookEntry {
    /// Get the staff map, or None if this is a direct URL
    pub fn as_staff_map(&self) -> Option<&HashMap<String, String>> {
        match self {
            WebhookEntry::StaffMap(map) => Some(map),
            WebhookEntry::Direct(_) => None,
        }
    }

    /// Get the direct URL, or None if this is a staff map
    pub fn as_direct(&self) -> Option<&str> {
        match self {
            WebhookEntry::Direct(url) => Some(url),
            WebhookEntry::StaffMap(_) => None,
        }
    }
}

impl NotifyConfig {
    /// Load config from a file path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, NotifyError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(NotifyError::ConfigNotFound(path.display().to_string()));
        }

        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }
}
