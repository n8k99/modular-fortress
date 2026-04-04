//! Narrative templates for day replay
//!
//! Provides time-of-day classification and narrative phrase templates.

use chrono::{NaiveTime, Timelike};

/// Time of day classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeOfDay {
    EarlyMorning, // 05:00 - 08:59
    Morning,      // 09:00 - 11:59
    Afternoon,    // 12:00 - 16:59
    Evening,      // 17:00 - 20:59
    Night,        // 21:00 - 04:59
}

impl TimeOfDay {
    pub fn from_time(time: NaiveTime) -> Self {
        let hour = time.hour();
        match hour {
            5..=8 => TimeOfDay::EarlyMorning,
            9..=11 => TimeOfDay::Morning,
            12..=16 => TimeOfDay::Afternoon,
            17..=20 => TimeOfDay::Evening,
            _ => TimeOfDay::Night,
        }
    }

    pub fn header(&self) -> &'static str {
        match self {
            TimeOfDay::EarlyMorning => "## Early Morning",
            TimeOfDay::Morning => "## Morning",
            TimeOfDay::Afternoon => "## Afternoon",
            TimeOfDay::Evening => "## Evening",
            TimeOfDay::Night => "## Night",
        }
    }
    
    pub fn order(&self) -> u8 {
        match self {
            TimeOfDay::EarlyMorning => 0,
            TimeOfDay::Morning => 1,
            TimeOfDay::Afternoon => 2,
            TimeOfDay::Evening => 3,
            TimeOfDay::Night => 4,
        }
    }
}

impl std::fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TimeOfDay::EarlyMorning => "early morning",
            TimeOfDay::Morning => "morning",
            TimeOfDay::Afternoon => "afternoon",
            TimeOfDay::Evening => "evening",
            TimeOfDay::Night => "night",
        };
        write!(f, "{}", s)
    }
}

/// Narrative templates for generating prose
pub struct NarrativeTemplate;

impl NarrativeTemplate {
    /// Intro phrases for notes
    pub fn note_intros() -> &'static [&'static str] {
        &[
            "Worked on",
            "Made notes about",
            "Updated",
            "Created documentation for",
            "Wrote about",
        ]
    }
    
    /// Intro phrases for logs
    pub fn log_intros() -> &'static [&'static str] {
        &[
            "Logged",
            "Recorded",
            "Noted",
            "Captured",
        ]
    }
    
    /// Intro phrases for gigs/stagehand
    pub fn gig_intros() -> &'static [&'static str] {
        &[
            "Had a gig at",
            "Worked a show at",
            "Stagehand call at",
            "Call at",
        ]
    }
    
    /// Intro phrases for memories
    pub fn memory_intros() -> &'static [&'static str] {
        &[
            "Remembered",
            "Recalled",
            "Made a note to remember",
            "Stored a memory about",
        ]
    }
    
    /// Get a random intro for a source type
    pub fn random_intro(source: &super::collector::EntrySource, seed: usize) -> &'static str {
        let intros = match source {
            super::collector::EntrySource::Memory => Self::note_intros(),
            super::collector::EntrySource::DailyLog => Self::log_intros(),
            super::collector::EntrySource::StagehandNote => Self::gig_intros(),
            super::collector::EntrySource::MemoryEntry => Self::memory_intros(),
        };
        intros[seed % intros.len()]
    }
    
    /// Transition phrases between sections
    pub fn transitions() -> &'static [&'static str] {
        &[
            "Later,",
            "Then,",
            "After that,",
            "Moving on,",
            "Next,",
            "Also,",
        ]
    }
    
    /// Get a transition phrase
    pub fn transition(seed: usize) -> &'static str {
        let transitions = Self::transitions();
        transitions[seed % transitions.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_of_day() {
        assert_eq!(
            TimeOfDay::from_time(NaiveTime::from_hms_opt(6, 30, 0).unwrap()),
            TimeOfDay::EarlyMorning
        );
        assert_eq!(
            TimeOfDay::from_time(NaiveTime::from_hms_opt(10, 0, 0).unwrap()),
            TimeOfDay::Morning
        );
        assert_eq!(
            TimeOfDay::from_time(NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            TimeOfDay::Afternoon
        );
        assert_eq!(
            TimeOfDay::from_time(NaiveTime::from_hms_opt(19, 0, 0).unwrap()),
            TimeOfDay::Evening
        );
        assert_eq!(
            TimeOfDay::from_time(NaiveTime::from_hms_opt(23, 0, 0).unwrap()),
            TimeOfDay::Night
        );
    }
    
    #[test]
    fn test_order() {
        assert!(TimeOfDay::Morning.order() < TimeOfDay::Afternoon.order());
        assert!(TimeOfDay::Afternoon.order() < TimeOfDay::Evening.order());
    }
}
