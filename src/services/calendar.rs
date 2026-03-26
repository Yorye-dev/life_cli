use std::collections::BTreeMap;
use std::io;

use chrono::NaiveDate;

use crate::config;
use crate::dates::{calendar_window, day_header, today};
use crate::files::{read_to_string, write_string};
use crate::models::{EventEntry, EventKind};
use crate::parser::{parse_event_line, parse_todo_line};

pub fn refresh_calendar() -> io::Result<()> {
    let current_day = today();
    let days = calendar_window(current_day);

    let todos_by_day = load_todos();
    let events_by_day = load_events();

    let mut blocks = Vec::new();

    for day in days {
        let mut lines = vec![day_header(day)];

        if let Some(todo_lines) = todos_by_day.get(&day) {
            for todo in todo_lines {
                lines.push(todo.clone());
            }
        }

        if let Some(event_lines) = events_by_day.get(&day) {
            for event in event_lines {
                lines.push(event.clone());
            }
        }

        blocks.push(lines.join("\n"));
    }

    let output = format!("{}\n", blocks.join("\n\n"));
    write_string(&config::calendar_file(), &output)?;
    println!("calendar.txt actualizado");
    Ok(())
}

fn load_todos() -> BTreeMap<NaiveDate, Vec<String>> {
    let mut grouped: BTreeMap<NaiveDate, Vec<(u8, String)>> = BTreeMap::new();

    if let Ok(content) = read_to_string(&config::todo_file()) {
        for line in content.lines() {
            if let Some(task) = parse_todo_line(line) {
                let rendered = match task.priority {
                    Some(priority) => format!("[TODO][{}] {}", priority, task.raw_text),
                    None => format!("[TODO] {}", task.raw_text),
                };

                let order = match task.priority {
                    Some('A') => 0,
                    Some('B') => 1,
                    Some('C') => 2,
                    _ => 3,
                };

                grouped.entry(task.due_date).or_default().push((order, rendered));
            }
        }
    }

    grouped
        .into_iter()
        .map(|(day, mut items)| {
            items.sort_by_key(|(order, text)| (*order, text.clone()));
            (day, items.into_iter().map(|(_, text)| text).collect())
        })
        .collect()
}

fn load_events() -> BTreeMap<NaiveDate, Vec<String>> {
    let mut grouped: BTreeMap<NaiveDate, Vec<EventEntry>> = BTreeMap::new();

    if let Ok(content) = read_to_string(&config::events_file()) {
        for line in content.lines() {
            if let Some(event) = parse_event_line(line) {
                grouped.entry(event.date).or_default().push(event);
            }
        }
    }

    grouped
        .into_iter()
        .map(|(day, mut events)| {
            events.sort_by_key(|event| match &event.kind {
                EventKind::AllDay => (0_u8, 0_u32, event.rendered.clone()),
                EventKind::Timed { start_minutes } => (1_u8, *start_minutes, event.rendered.clone()),
            });

            let rendered = events.into_iter().map(|event| event.rendered).collect();
            (day, rendered)
        })
        .collect()
}
