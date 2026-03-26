use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "life")]
#[command(version)]
#[command(about = "Sistema life en texto plano")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Crea ~/life y los archivos base
    Init,

    /// Comandos de calendario
    Calendar {
        #[command(subcommand)]
        command: CalendarCommands,
    },

    /// Comandos de TODO
    Todo {
        #[command(subcommand)]
        command: TodoCommands,
    },
}

#[derive(Subcommand)]
pub enum CalendarCommands {
    /// Reconstruye calendar.txt con hoy + 7 días
    Refresh,
}

#[derive(Subcommand)]
pub enum TodoCommands {
    /// Añade una línea a todo.txt y refresca calendar.txt
    Add {
        /// Línea completa del TODO, ej: "(A) 2026-03-26 #h llamar a juan"
        line: String,
    },
}
