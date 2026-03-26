use std::io;

use crate::config;
use crate::files::{ensure_dir, ensure_file};

const INDEX_TEMPLATE: &str = r#"# Life

## Operativa
- [Inbox](inbox.txt)
- [Todo](todo.txt)
- [Calendar](calendar.txt)
- [Events](events.txt)
- [Recurring](recurring.txt)
- [Projects](projects.txt)
- [Someday](someday.txt)

## Histórico
- [Historial Todo](historialTodo.txt)
- [Historial Calendar](historialCalendar.txt)

## Notas
- [Notes](notes/)
"#;

pub fn init_life_dir() -> io::Result<()> {
    ensure_dir(&config::life_dir())?;
    ensure_dir(&config::notes_dir())?;

    ensure_file(&config::index_file(), INDEX_TEMPLATE)?;
    ensure_file(&config::inbox_file(), "")?;
    ensure_file(&config::todo_file(), "")?;
    ensure_file(&config::history_todo_file(), "")?;
    ensure_file(&config::calendar_file(), "")?;
    ensure_file(&config::history_calendar_file(), "")?;
    ensure_file(&config::events_file(), "")?;
    ensure_file(&config::recurring_file(), "@anual\n\n@mensual\n\n@bimestral\n\n@semanal\n")?;
    ensure_file(&config::projects_file(), "")?;
    ensure_file(&config::someday_file(), "")?;

    println!("Estructura creada en {}", config::life_dir().display());
    Ok(())
}
