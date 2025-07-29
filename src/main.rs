use clap::Parser;

#[derive(Parser)]
#[command(name = "life")]
#[command(about = "TODO")]
#[command(version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,

}
#[command(subcommand)]
enum Commands{
    Add {},
    List,
    Done{},
    Remove{}
}

struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
