//! Phase-to-staff mappings for development cycles

use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::Office;

/// Development cycle phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Architecture,
    Design,
    Implementation,
    Testing,
    Qa,
    Security,
    Review,
    CodeReview,
    DevOps,
    Ci,
    Deployment,
    Integration,
}

impl Phase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::Architecture => "architecture",
            Phase::Design => "design",
            Phase::Implementation => "implementation",
            Phase::Testing => "testing",
            Phase::Qa => "qa",
            Phase::Security => "security",
            Phase::Review => "review",
            Phase::CodeReview => "code_review",
            Phase::DevOps => "devops",
            Phase::Ci => "ci",
            Phase::Deployment => "deployment",
            Phase::Integration => "integration",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Phase::Architecture | Phase::Design => "📐",
            Phase::Implementation => "🔧",
            Phase::Testing | Phase::Qa => "🧪",
            Phase::Security => "🔒",
            Phase::Review | Phase::CodeReview => "👁️",
            Phase::DevOps | Phase::Ci => "⚙️",
            Phase::Deployment => "🚀",
            Phase::Integration => "📊",
        }
    }
}

/// Phase mappings per office
/// Maps phase name (lowercase) -> staff key
pub static PHASE_MAPPINGS: Lazy<HashMap<Office, HashMap<&'static str, &'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Technical Development Office (Eliana Riviera - CTO)
    let mut tech_dev = HashMap::new();
    tech_dev.insert("architecture", "SamirKhanna");
    tech_dev.insert("design", "SamirKhanna");
    tech_dev.insert("implementation", "DevinPark");
    tech_dev.insert("testing", "DanielleGreen");
    tech_dev.insert("qa", "DanielleGreen");
    tech_dev.insert("security", "SanjayPatel");
    tech_dev.insert("review", "CaseyHan");
    tech_dev.insert("code_review", "CaseyHan");
    tech_dev.insert("devops", "MorganFields");
    tech_dev.insert("ci", "MorganFields");
    tech_dev.insert("deployment", "IsaacMiller");
    tech_dev.insert("deploy", "IsaacMiller");
    tech_dev.insert("integration", "ElisePark");
    map.insert(Office::TechDev, tech_dev);

    // Operations Office (Kathryn Lyonne - COO)
    // TODO: Add mappings when Kathryn comes online
    let operations = HashMap::new();
    map.insert(Office::Operations, operations);

    // Content Office (Sylvia Inkweaver - Chief of Content)
    // TODO: Add mappings when Sylvia comes online
    let content = HashMap::new();
    map.insert(Office::Content, content);

    map
});

/// Get all phases for an office with their assigned staff
pub fn get_office_phases(office: Office) -> Vec<(Phase, &'static str)> {
    PHASE_MAPPINGS
        .get(&office)
        .map(|phases| {
            let mut result = Vec::new();
            if let Some(staff) = phases.get("architecture") {
                result.push((Phase::Architecture, *staff));
            }
            if let Some(staff) = phases.get("implementation") {
                result.push((Phase::Implementation, *staff));
            }
            if let Some(staff) = phases.get("testing") {
                result.push((Phase::Testing, *staff));
            }
            if let Some(staff) = phases.get("security") {
                result.push((Phase::Security, *staff));
            }
            if let Some(staff) = phases.get("review") {
                result.push((Phase::Review, *staff));
            }
            if let Some(staff) = phases.get("devops") {
                result.push((Phase::DevOps, *staff));
            }
            if let Some(staff) = phases.get("deployment") {
                result.push((Phase::Deployment, *staff));
            }
            if let Some(staff) = phases.get("integration") {
                result.push((Phase::Integration, *staff));
            }
            result
        })
        .unwrap_or_default()
}
