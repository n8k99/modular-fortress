//! Webhook notification system for EM Staff executives.
//!
//! Provides unified notification infrastructure that any department head
//! can use to direct their staff webhooks.
//!
//! # Example
//!
//! ```rust,no_run
//! use dpn_core::notify::{NotifyClient, Office};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = NotifyClient::from_config("/path/to/config.json")?;
//!
//! // Send by phase (auto-resolves staff)
//! client.send_by_phase(Office::TechDev, "architecture", "📐 Design complete").await?;
//!
//! // Send by staff directly
//! client.send_by_staff(Office::TechDev, "SamirKhanna", "📐 Design complete").await?;
//! # Ok(())
//! # }
//! ```

mod config;
mod phases;

pub use config::{NotifyConfig, WebhookEntry};
pub use phases::{Phase, PHASE_MAPPINGS, get_office_phases};

use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotifyError {
    #[error("Config not found: {0}")]
    ConfigNotFound(String),

    #[error("Failed to parse config: {0}")]
    ConfigParse(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Office not found: {0}")]
    OfficeNotFound(String),

    #[error("Staff not found: {0} in {1}")]
    StaffNotFound(String, String),

    #[error("Phase not found: {0} in {1}")]
    PhaseNotFound(String, String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

/// Known office identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Office {
    /// Technical Development (Eliana Riviera - CTO)
    TechDev,
    /// Operations (Kathryn Lyonne - COO)
    Operations,
    /// Content (Sylvia Inkweaver - Chief of Content)
    Content,
    /// Office of CEO (Sarah Lin)
    Ceo,
}

impl Office {
    pub fn config_key(&self) -> &'static str {
        match self {
            Office::TechDev => "tech_dev_office",
            Office::Operations => "operations_office",
            Office::Content => "content_office",
            Office::Ceo => "office_of_ceo",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "tech_dev_office" => Some(Office::TechDev),
            "operations_office" => Some(Office::Operations),
            "content_office" => Some(Office::Content),
            "office_of_ceo" => Some(Office::Ceo),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Office::TechDev => "Technical Development",
            Office::Operations => "Operations",
            Office::Content => "Content",
            Office::Ceo => "Office of CEO",
        }
    }
}

/// Staff display names
pub fn staff_display_name(key: &str) -> &str {
    match key {
        "SamirKhanna" => "Samir Khanna",
        "DevinPark" => "Devin Park",
        "DanielleGreen" => "Danielle Green",
        "SanjayPatel" => "Sanjay Patel",
        "CaseyHan" => "Casey Han",
        "MorganFields" => "Morgan Fields",
        "IsaacMiller" => "Isaac Miller",
        "ElisePark" => "Elise Park",
        "ElianaRiviera" => "Eliana Riviera",
        "KathrynLyonne" => "Kathryn Lyonne",
        "SylviaInkweaver" => "Sylvia Inkweaver",
        "SarahLin" => "Sarah Lin",
        "VincentJanssen" => "Vincent Janssen",
        "JMaxwellCharbourne" => "J. Maxwell Charbourne",
        "LRMorgenstern" => "L.R. Morgenstern",
        other => other,
    }
}

/// Discord webhook payload
#[derive(Debug, Serialize)]
struct WebhookPayload {
    content: String,
    username: String,
}

/// Notification client for sending webhook messages
pub struct NotifyClient {
    config: NotifyConfig,
    http: Client,
}

impl NotifyClient {
    /// Create a new client from a config file path
    pub fn from_config<P: AsRef<Path>>(path: P) -> Result<Self, NotifyError> {
        let config = NotifyConfig::load(path)?;
        Ok(Self {
            config,
            http: Client::new(),
        })
    }

    /// Create from the default config location
    pub fn from_default_config() -> Result<Self, NotifyError> {
        Self::from_config("/Volumes/Elements/Development/config.json")
    }

    /// Get webhook URL for a staff member in an office
    pub fn get_webhook(&self, office: Office, staff: &str) -> Option<&str> {
        self.config
            .discord
            .webhooks
            .get(office.config_key())
            .and_then(|entry| entry.as_staff_map())
            .and_then(|map| map.get(staff))
            .map(|s| s.as_str())
    }

    /// List all staff in an office
    pub fn list_staff(&self, office: Office) -> Vec<&str> {
        self.config
            .discord
            .webhooks
            .get(office.config_key())
            .and_then(|entry| entry.as_staff_map())
            .map(|map| map.keys().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// List all available offices
    pub fn list_offices(&self) -> Vec<Office> {
        self.config
            .discord
            .webhooks
            .keys()
            .filter_map(|k| Office::from_key(k))
            .collect()
    }

    /// Resolve a phase to a staff member for an office
    pub fn resolve_phase(&self, office: Office, phase: &str) -> Option<&'static str> {
        PHASE_MAPPINGS
            .get(&office)
            .and_then(|phases| phases.get(phase.to_lowercase().as_str()))
            .copied()
    }

    /// Send a notification by staff key
    pub async fn send_by_staff(
        &self,
        office: Office,
        staff: &str,
        message: &str,
    ) -> Result<(), NotifyError> {
        let webhook_url = self
            .get_webhook(office, staff)
            .ok_or_else(|| NotifyError::StaffNotFound(staff.to_string(), office.config_key().to_string()))?;

        let display_name = staff_display_name(staff);
        self.send_webhook(webhook_url, message, display_name).await
    }

    /// Send a notification by phase (auto-resolves to staff)
    pub async fn send_by_phase(
        &self,
        office: Office,
        phase: &str,
        message: &str,
    ) -> Result<(), NotifyError> {
        let staff = self
            .resolve_phase(office, phase)
            .ok_or_else(|| NotifyError::PhaseNotFound(phase.to_string(), office.config_key().to_string()))?;

        self.send_by_staff(office, staff, message).await
    }

    /// Send raw webhook request
    async fn send_webhook(
        &self,
        url: &str,
        message: &str,
        username: &str,
    ) -> Result<(), NotifyError> {
        let payload = WebhookPayload {
            content: message.to_string(),
            username: username.to_string(),
        };

        self.http
            .post(url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_office_config_keys() {
        assert_eq!(Office::TechDev.config_key(), "tech_dev_office");
        assert_eq!(Office::Operations.config_key(), "operations_office");
        assert_eq!(Office::Content.config_key(), "content_office");
        assert_eq!(Office::Ceo.config_key(), "office_of_ceo");
    }

    #[test]
    fn test_office_from_key() {
        assert_eq!(Office::from_key("tech_dev_office"), Some(Office::TechDev));
        assert_eq!(Office::from_key("operations_office"), Some(Office::Operations));
        assert_eq!(Office::from_key("content_office"), Some(Office::Content));
        assert_eq!(Office::from_key("office_of_ceo"), Some(Office::Ceo));
        assert_eq!(Office::from_key("unknown"), None);
    }

    #[test]
    fn test_office_display_name() {
        assert_eq!(Office::TechDev.display_name(), "Technical Development");
        assert_eq!(Office::Operations.display_name(), "Operations");
        assert_eq!(Office::Content.display_name(), "Content");
        assert_eq!(Office::Ceo.display_name(), "Office of CEO");
    }

    #[test]
    fn test_staff_display_names() {
        assert_eq!(staff_display_name("SamirKhanna"), "Samir Khanna");
        assert_eq!(staff_display_name("DevinPark"), "Devin Park");
        assert_eq!(staff_display_name("DanielleGreen"), "Danielle Green");
        assert_eq!(staff_display_name("SanjayPatel"), "Sanjay Patel");
        assert_eq!(staff_display_name("CaseyHan"), "Casey Han");
        assert_eq!(staff_display_name("MorganFields"), "Morgan Fields");
        assert_eq!(staff_display_name("IsaacMiller"), "Isaac Miller");
        assert_eq!(staff_display_name("ElisePark"), "Elise Park");
        assert_eq!(staff_display_name("ElianaRiviera"), "Eliana Riviera");
        assert_eq!(staff_display_name("UnknownPerson"), "UnknownPerson");
    }

    #[test]
    fn test_phase_resolution() {
        // Phase mappings should resolve correctly
        let phases = PHASE_MAPPINGS.get(&Office::TechDev).unwrap();
        assert_eq!(phases.get("architecture"), Some(&"SamirKhanna"));
        assert_eq!(phases.get("implementation"), Some(&"DevinPark"));
        assert_eq!(phases.get("testing"), Some(&"DanielleGreen"));
        assert_eq!(phases.get("security"), Some(&"SanjayPatel"));
        assert_eq!(phases.get("review"), Some(&"CaseyHan"));
        assert_eq!(phases.get("devops"), Some(&"MorganFields"));
        assert_eq!(phases.get("deployment"), Some(&"IsaacMiller"));
        assert_eq!(phases.get("integration"), Some(&"ElisePark"));
    }

    #[test]
    fn test_notify_client_from_default_config() {
        // Test that we can load from the default config path
        let client = NotifyClient::from_default_config();
        assert!(client.is_ok(), "Failed to load from default config: {:?}", client.err());
        
        let client = client.unwrap();
        
        // Verify we can list offices
        let offices = client.list_offices();
        assert!(!offices.is_empty(), "Expected at least one office");
        assert!(offices.contains(&Office::TechDev), "Expected TechDev office");
    }

    #[test]
    fn test_notify_client_list_staff() {
        let client = NotifyClient::from_default_config().expect("Config load failed");
        
        // List staff in TechDev office
        let staff = client.list_staff(Office::TechDev);
        assert!(!staff.is_empty(), "Expected staff in TechDev");
        
        // Should have our known staff members
        assert!(staff.contains(&"DanielleGreen"), "Expected DanielleGreen in staff");
    }

    #[test]
    fn test_notify_client_get_webhook() {
        let client = NotifyClient::from_default_config().expect("Config load failed");
        
        // Get webhook for known staff
        let webhook = client.get_webhook(Office::TechDev, "DanielleGreen");
        assert!(webhook.is_some(), "Expected webhook for DanielleGreen");
        
        let webhook = webhook.unwrap();
        assert!(webhook.starts_with("https://discord.com/api/webhooks/"), 
                "Expected Discord webhook URL, got: {}", webhook);
    }

    #[test]
    fn test_notify_client_resolve_phase() {
        let client = NotifyClient::from_default_config().expect("Config load failed");
        
        // Resolve phases to staff
        let staff = client.resolve_phase(Office::TechDev, "testing");
        assert_eq!(staff, Some("DanielleGreen"));
        
        let staff = client.resolve_phase(Office::TechDev, "architecture");
        assert_eq!(staff, Some("SamirKhanna"));
        
        // Unknown phase should return None
        let staff = client.resolve_phase(Office::TechDev, "unknown_phase");
        assert_eq!(staff, None);
    }
}
