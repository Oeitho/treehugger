use std::path::Path;

pub enum Action {
    CreateDirectory { path: Box<Path>, hidden: bool },
    CreateFile { path: Box<Path>, content: Vec<u8> },
}

pub mod hash_object;
pub mod initialize_repository;
