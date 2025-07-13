mod command;
use crate::command::initialize_repository::initialize_repository;
use std::{env, path::{Path, PathBuf}};

fn main() {
    let result = initialize_repository(get_path());
    match result {
        Ok(()) => {
            println!("Repository created");
        }
        Err(error) => {
            panic!("{}", error);
        }
    };
}

fn get_path() -> Box<Path> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    let path: PathBuf = [path.as_str(), ".git"].iter().collect();
    path.into_boxed_path()
}
