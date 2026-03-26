mod cli;
mod config;
mod dates;
mod files;
mod models;
mod parser;
mod services;

use clap::Parser;
use cli::{Cli, Commands, CalendarCommands, TodoCommands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => services::bootstrap::init_life_dir(),
        Commands::Calendar { command } => match command {
            CalendarCommands::Refresh => services::calendar::refresh_calendar(),
        },
        Commands::Todo { command } => match command {
            TodoCommands::Add { line } => services::todo::add_todo(&line),
        },
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
