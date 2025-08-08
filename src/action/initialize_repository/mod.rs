use crate::action::Action;
use crate::action::Action::{CreateDirectory, CreateFile};
use std::path::Path;

pub fn initialize_repository(path: Box<Path>) -> Vec<Action> {
    let path = path.join(".git").into_boxed_path();
    vec![
        CreateDirectory {
            path: path.clone(),
            hidden: true,
        },
        CreateDirectory {
            path: path.join("objects").into_boxed_path(),
            hidden: true,
        },
        CreateDirectory {
            path: path.join("refs").into_boxed_path(),
            hidden: true,
        },
        CreateFile {
            path: path.join("HEAD").into_boxed_path(),
            content: b"ref: refs/heads/master".to_vec(),
        },
    ]
}
