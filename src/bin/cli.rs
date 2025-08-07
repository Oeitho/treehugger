use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::Write;

use std::path::{Path, PathBuf};
use treehugger::action::Action;
use treehugger::action::initialize_repository::initialize_repository;
use windows::Win32::Storage::FileSystem;

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
    let actions = match &args.command {
        Command::Initialize { folder } => initialize_repository(folder.clone().into_boxed_path()),
    };
    for action in actions {
        match action {
            Action::CreateDirectory { path, hidden } => {
                create_directory(path, hidden).expect("Failed to create directory.")
            }
            Action::CreateFile { path, content } => {
                create_file(path, content).expect("Failed to create file.")
            }
        }
    }
}

fn create_directory(path: Box<Path>, hidden: bool) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path.clone())?;
    #[cfg(target_os = "windows")]
    if hidden {
        make_directory_hidden(path)?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn make_directory_hidden(path: Box<Path>) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        use windows::{Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN, core::PCSTR};
        let directory_path: String = String::from(path.to_str().unwrap()) + "\0";

        FileSystem::SetFileAttributesA(
            PCSTR::from_raw(directory_path.as_ptr()),
            FILE_ATTRIBUTE_HIDDEN,
        )?;
    }
    Ok(())
}

fn create_file(path: Box<Path>, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_slice())?;
    Ok(())
}
