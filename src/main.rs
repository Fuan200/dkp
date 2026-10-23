mod generator;
use clap::{Parser, Subcommand};
use std::io;

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

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::New { name } => {
            generator::create_project(&name)?;
            println!("Project created: {name}");
        }
    }
    Ok(())
}
