use std::path::PathBuf;

pub fn life_dir() -> PathBuf {
    if cfg!(windows) {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata).join("life");
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join("life");
    }

    PathBuf::from("./life")
}

pub fn index_file() -> PathBuf {
    life_dir().join("index.md")
}

pub fn inbox_file() -> PathBuf {
    life_dir().join("inbox.txt")
}

pub fn todo_file() -> PathBuf {
    life_dir().join("todo.txt")
}

pub fn history_todo_file() -> PathBuf {
    life_dir().join("historialTodo.txt")
}

pub fn calendar_file() -> PathBuf {
    life_dir().join("calendar.txt")
}

pub fn history_calendar_file() -> PathBuf {
    life_dir().join("historialCalendar.txt")
}

pub fn events_file() -> PathBuf {
    life_dir().join("events.txt")
}

pub fn recurring_file() -> PathBuf {
    life_dir().join("recurring.txt")
}

pub fn projects_file() -> PathBuf {
    life_dir().join("projects.txt")
}

pub fn someday_file() -> PathBuf {
    life_dir().join("someday.txt")
}

pub fn notes_dir() -> PathBuf {
    life_dir().join("notes")
}
