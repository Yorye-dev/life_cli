use std::io;

use crate::config;
use crate::files::append_line;

pub fn add_todo(line: &str) -> io::Result<()> {
    append_line(&config::todo_file(), line)?;
    crate::services::calendar::refresh_calendar()?;
    println!("TODO añadido");
    Ok(())
}
