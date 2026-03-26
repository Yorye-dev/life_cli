use chrono::NaiveDate;
use regex::Regex;

use crate::models::{EventEntry, EventKind, TodoTask};

pub fn parse_todo_line(line: &str) -> Option<TodoTask> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('X') {
        return None;
    }

    let re = Regex::new(r"^(?:\(([A-Z])\)\s+)?(\d{4}-\d{2}-\d{2})\s+(.*)$").ok()?;
    let caps = re.captures(line)?;

    let priority = caps.get(1).and_then(|m| m.as_str().chars().next());
    let due_date = NaiveDate::parse_from_str(caps.get(2)?.as_str(), "%Y-%m-%d").ok()?;
    let raw_text = caps.get(3)?.as_str().to_string();

    Some(TodoTask {
        priority,
        due_date,
        raw_text,
    })
}

pub fn parse_event_line(line: &str) -> Option<EventEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let re = Regex::new(r"^(\d{4}-\d{2}-\d{2})\s+\+\s+(.*)$").ok()?;
    let caps = re.captures(line)?;

    let date = NaiveDate::parse_from_str(caps.get(1)?.as_str(), "%Y-%m-%d").ok()?;
    let body = caps.get(2)?.as_str().trim();

    let rendered = format!("+ {}", body);
    let kind = classify_event(body);

    Some(EventEntry {
        date,
        rendered,
        kind,
    })
}

fn classify_event(body: &str) -> EventKind {
    let timed_re = Regex::new(r"^(\d{1,2})(?:-\d{1,4})?\b").unwrap();

    if let Some(caps) = timed_re.captures(body) {
        if let Some(hour_match) = caps.get(1) {
            if let Ok(hour) = hour_match.as_str().parse::<u32>() {
                return EventKind::Timed {
                    start_minutes: hour * 60,
                };
            }
        }
    }

    EventKind::AllDay
}
