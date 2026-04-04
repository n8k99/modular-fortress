//! ICS calendar parser
//!
//! Extracts events from .ics feeds for stagehand show/venue tracking.

use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use ical::parser::ical::component::IcalCalendar;
use ical::IcalParser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Parsed ICS event with relevant show information
#[derive(Debug, Clone)]
pub struct IcsEvent {
    /// Event summary (show name)
    pub summary: String,
    /// Event location (venue)
    pub location: Option<String>,
    /// Event start date
    pub date: NaiveDate,
    /// Event start time (call time)
    pub start_time: Option<NaiveTime>,
    /// Event end time
    pub end_time: Option<NaiveTime>,
    /// Event description
    pub description: Option<String>,
    /// Original UID from ICS
    pub uid: Option<String>,
}

impl IcsEvent {
    /// Create a stagehand-compatible display title
    pub fn display_title(&self) -> String {
        match &self.location {
            Some(loc) => format!("{} @ {}", self.summary, loc),
            None => self.summary.clone(),
        }
    }
}

/// Parse an ICS string and extract events
pub fn parse_ics(ics_content: &str) -> Result<Vec<IcsEvent>> {
    let reader = BufReader::new(ics_content.as_bytes());
    let parser = IcalParser::new(reader);
    
    let mut events = Vec::new();
    
    for calendar_result in parser {
        let calendar = calendar_result.context("Failed to parse ICS calendar")?;
        events.extend(extract_events_from_calendar(&calendar)?);
    }
    
    Ok(events)
}

/// Parse an ICS file and extract events
pub fn parse_ics_file<P: AsRef<Path>>(path: P) -> Result<Vec<IcsEvent>> {
    let file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open ICS file: {:?}", path.as_ref()))?;
    let reader = BufReader::new(file);
    let parser = IcalParser::new(reader);
    
    let mut events = Vec::new();
    
    for calendar_result in parser {
        let calendar = calendar_result.context("Failed to parse ICS calendar")?;
        events.extend(extract_events_from_calendar(&calendar)?);
    }
    
    Ok(events)
}

fn extract_events_from_calendar(calendar: &IcalCalendar) -> Result<Vec<IcsEvent>> {
    let mut events = Vec::new();
    
    for event in &calendar.events {
        let mut summary = None;
        let mut location = None;
        let mut dtstart = None;
        let mut dtend = None;
        let mut description = None;
        let mut uid = None;
        
        for prop in &event.properties {
            match prop.name.as_str() {
                "SUMMARY" => summary = prop.value.clone(),
                "LOCATION" => location = prop.value.clone(),
                "DTSTART" => dtstart = prop.value.clone(),
                "DTEND" => dtend = prop.value.clone(),
                "DESCRIPTION" => description = prop.value.clone(),
                "UID" => uid = prop.value.clone(),
                _ => {}
            }
        }
        
        // Skip events without summary or start date
        let summary = match summary {
            Some(s) => s,
            None => continue,
        };
        
        let dtstart = match dtstart {
            Some(d) => d,
            None => continue,
        };
        
        // Parse date/time from DTSTART
        let (date, start_time) = parse_ics_datetime(&dtstart)?;
        
        // Parse end time if available
        let end_time = if let Some(end) = dtend {
            parse_ics_datetime(&end).ok().and_then(|(_, t)| t)
        } else {
            None
        };
        
        events.push(IcsEvent {
            summary,
            location,
            date,
            start_time,
            end_time,
            description,
            uid,
        });
    }
    
    Ok(events)
}

/// Parse ICS datetime formats
/// Supports: YYYYMMDD, YYYYMMDDTHHMMSS, YYYYMMDDTHHMMSSZ
fn parse_ics_datetime(value: &str) -> Result<(NaiveDate, Option<NaiveTime>)> {
    let value = value.trim();
    
    // Date only: YYYYMMDD
    if value.len() == 8 && !value.contains('T') {
        let date = NaiveDate::parse_from_str(value, "%Y%m%d")
            .with_context(|| format!("Failed to parse date: {}", value))?;
        return Ok((date, None));
    }
    
    // DateTime: YYYYMMDDTHHMMSS or YYYYMMDDTHHMMSSZ
    let clean = value.trim_end_matches('Z');
    
    if clean.contains('T') {
        let dt = NaiveDateTime::parse_from_str(clean, "%Y%m%dT%H%M%S")
            .with_context(|| format!("Failed to parse datetime: {}", value))?;
        return Ok((dt.date(), Some(dt.time())));
    }
    
    anyhow::bail!("Unrecognized ICS datetime format: {}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_only() {
        let (date, time) = parse_ics_datetime("20260215").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 2, 15).unwrap());
        assert!(time.is_none());
    }

    #[test]
    fn test_parse_datetime() {
        let (date, time) = parse_ics_datetime("20260215T180000").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 2, 15).unwrap());
        assert_eq!(time, Some(NaiveTime::from_hms_opt(18, 0, 0).unwrap()));
    }

    #[test]
    fn test_parse_datetime_utc() {
        let (date, time) = parse_ics_datetime("20260215T180000Z").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 2, 15).unwrap());
        assert_eq!(time, Some(NaiveTime::from_hms_opt(18, 0, 0).unwrap()));
    }
}
