mod command;
use clap::{Parser, Subcommand};

use crate::command::initialize_repository::initialize_repository;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(name = "init")]
    Initialize {
        #[arg(default_value = ".")]
        folder: PathBuf,
    },
}

fn main() {
    let args = Args::parse();
    let result = match &args.command {
        Command::Initialize { folder } => initialize_repository(folder.clone().into_boxed_path()),
    };
    match result {
        Ok(()) => {
            println!("Repository created");
        }
        Err(error) => {
            panic!("{}", error);
        }
    };
}
