mod generator;
mod git;
mod prompts;
use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(
    name = "dkp",
    version,
    about = "CLI for creating devkitPro homebrew projects"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Create a new devkitPro project: dkp new <name>")]
    New {
        #[arg(value_name = "name", help = "Name of the project directory")]
        name: String,
    },
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

            println!(
                "\nProject created successfully!!!\n\nProject: {name}\nPlatform: {}\n\nNext steps:\n\n    $ cd {name}\n    $ make\n",
                platform.label()
            );
        }
    }
    Ok(())
}
