use crate::action::Action;
use crate::action::Action::{CreateDirectory, CreateFile};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::path::Path;

pub struct Entry {
    entry_type: EntryType,
    hash: Vec<u8>,
    name: String,
}

pub enum EntryType {
    Blob { mode: FileMode },
    Tree,
}

impl EntryType {
    pub fn to_number(&self) -> Vec<u8> {
        match self {
            EntryType::Blob { mode } => mode.to_number(),
            EntryType::Tree => b"040000".to_vec(),
        }
    }
}

pub enum FileMode {
    NormalFile,
    ExecutableFile,
    SymbolicLink,
}

impl FileMode {
    pub fn to_number(&self) -> Vec<u8> {
        match self {
            FileMode::NormalFile => b"100644".to_vec(),
            FileMode::ExecutableFile => b"100755".to_vec(),
            FileMode::SymbolicLink => b"120000".to_vec(),
        }
    }
}

pub enum Object {
    Blob { content: Vec<u8> },
    Tree { entries: Vec<Entry> },
}

impl Object {
    fn type_as_byte_array(&self) -> Vec<u8> {
        match self {
            Object::Blob { .. } => b"blob ".to_vec(),
            Object::Tree { .. } => b"tree ".to_vec(),
        }
    }

    fn content(&self) -> Vec<u8> {
        match self {
            Object::Blob { content } => content.clone(),
            Object::Tree { entries } => {
                let result: Vec<u8> = entries
                    .iter()
                    .flat_map(|entry| {
                        let mut result = entry.entry_type.to_number();
                        result.extend(b" ");
                        result.extend(entry.name.as_bytes());
                        result.push(0);
                        result.extend(entry.hash.clone());
                        result
                    })
                    .collect();
                result
            }
        }
    }
}

pub fn hash_object(path: Box<Path>, object: Object) -> Vec<Action> {
    let mut actions = Vec::new();
    let content_byte_array = create_content_byte_array(object);
    let hash = Sha1::digest(content_byte_array.clone());
    let hash = format!("{:x}", hash);
    let (first, second) = hash.split_at(2);
    let path = path.join(first).into_boxed_path();

    actions.push(CreateDirectory {
        path: path.clone(),
        hidden: false,
    });

    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    zlib_encoder
        .write_all(content_byte_array.as_slice())
        .expect("TODO: ZLibEncoder failed to encode content.");
    let file_content = zlib_encoder
        .finish()
        .expect("Failed to get zlib-encoded file content");

    actions.push(CreateFile {
        path: path.join(second).into_boxed_path(),
        content: file_content,
    });

    actions
}

fn create_content_byte_array(object: Object) -> Vec<u8> {
    let mut result = object.type_as_byte_array();
    let content = object.content();
    result.extend(format!("{}", content.len()).as_bytes());
    result.push(0);
    result.extend(content.clone());
    result
}
