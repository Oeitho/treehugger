use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
#[cfg(target_os = "windows")]
use windows::Win32::Storage::FileSystem;


pub fn initialize_repository(path: Box<Path>) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        return Err(Box::from("Folder already exists."))
    }
    create_hidden_directory(path.clone())?;
    create_directory(path.join("objects").into_boxed_path())?;
    create_directory(path.join("refs").into_boxed_path())?;
    create_head_file(path.clone())?;
    Result::Ok(())
}

fn create_hidden_directory(path: Box<Path>) -> Result<(), Box<dyn std::error::Error>> {
    create_directory(path.clone())?;
    #[cfg(target_os = "windows")]
    make_directory_hidden(path)?;
    Ok(())
}

fn create_directory(path: Box<Path>) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn make_directory_hidden(path: Box<Path>) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        use windows::{core::PCSTR, Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN};
        let directory_path: String = String::from(path.to_str().unwrap()) + "\0";

        FileSystem::SetFileAttributesA(PCSTR::from_raw(directory_path.as_ptr()), FILE_ATTRIBUTE_HIDDEN)?;
    }
    Ok(())
}

fn create_head_file(path: Box<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut path: PathBuf = PathBuf::from(path.clone());
    path.push("HEAD");
    let mut file = File::create(path)?;
    file.write_all(b"ref: refs/heads/master")?;
    Result::Ok(())
}
