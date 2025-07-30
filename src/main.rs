use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "life", version, about = "CLI estilo life.txt")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
}


fn main() {
    print!("Vivo?")
}
