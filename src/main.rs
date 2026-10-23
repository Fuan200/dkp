mod generator;
mod git;
mod prompts;
use clap::{Parser, Subcommand};
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::New { name } => {
            let platform = prompts::select_platform()?;
            let initalize_git = prompts::confirm_git_init()?;
            generator::create_project(&name, &platform)?;

            if initalize_git {
                git::initialize(&name)?;
            }

            println!("Project created: {name} ({})", platform.label());
        }
    }
    Ok(())
}
