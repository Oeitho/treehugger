use crate::action::Action;
use crate::action::Action::{CreateDirectory, CreateFile};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::path::Path;

pub enum Object {
    Blob { content: Vec<u8> },
}

impl Object {
    fn content(&self) -> Vec<u8> {
        match self {
            Object::Blob { content } => {
                let mut result = b"blob ".to_vec();
                result.extend(format!("{}", content.len()).as_bytes());
                result.push(0);
                result.extend(content.clone());
                result
            }
        }
    }
}

pub fn hash_object(path: Box<Path>, object: Object) -> Vec<Action> {
    let mut actions = Vec::new();
    let content_byte_array = object.content();
    println!("Content: {content_byte_array:?}");
    let hash = Sha1::digest(content_byte_array.clone());
    let hash = format!("{:x}", hash);
    println!("Hash: [{hash}]");
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

    println!("File content: [{file_content:?}]");

    actions.push(CreateFile {
        path: path.join(second).into_boxed_path(),
        content: file_content,
    });

    actions
}
