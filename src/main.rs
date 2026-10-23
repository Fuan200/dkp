use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dk", version, about = "devkitPro")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::New { name } => {
            println!("{name}");
        }
    }
}
