use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct TodoTask {
    pub priority: Option<char>,
    pub due_date: NaiveDate,
    pub raw_text: String,
}

#[derive(Debug, Clone)]
pub enum EventKind {
    AllDay,
    Timed { start_minutes: u32 },
}

#[derive(Debug, Clone)]
pub struct EventEntry {
    pub date: NaiveDate,
    pub rendered: String,
    pub kind: EventKind,
}
