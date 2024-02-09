use std::path::{PathBuf, Path};

use crossterm::event::{self, KeyCode};

use crate::model::Sniper;

use anyhow::Result;

#[derive(PartialEq, Eq, Clone)]
pub enum Message  {
    Quit,
    OpenDir(PathBuf),
    OpenFile(PathBuf),
    OpenPath(PathBuf),
    Error(String),
}

/// Handle keypress events
pub fn handle_key (key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('r') => Some(Message::OpenDir(".".into())),
        _ => None,
    }
}

/// Update the model based on a message
///
/// # Impurity
/// This is (deliberately) the most impure function in this codebase.
pub fn update (model: &mut Sniper, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running = false;
            None
        },
        Message::OpenPath(path) => {
            Some(if path.is_dir() {
                Message::OpenDir(path)
            } else if path.is_file() {
                Message::OpenFile(path)
            } else {
                Message::Error("Unable to open {path} - unknown file type".into())
            })
        },
        Message::OpenFile(file_path) => {
            opener::open(file_path).err()
                .map(|e| Message::Error(e.to_string()))
        },
        Message::OpenDir(dir_name) => {
            match get_files(&dir_name){
                Ok(files) => {
                    model.file_list.elems = files.iter().map(|path_buf| path_buf.to_string_lossy().to_string()).collect();
                    None
                },
                Err(e) => Some(Message::Error(e.to_string())),
            }
        },
        Message::Error(err_string) => {
            dbg!(format!("{err_string}"));
            None
        },
    }
}

/// Impurity:
///     I/O - reads file names
///
pub fn get_files(path: &Path) -> Result<Vec<PathBuf>> {
    [".".into(), "..".into()].map(Ok)
        .into_iter()
        .chain(
            path.read_dir()?
                .map(|entry| Ok(entry?.path()))
        )
        .collect()
}
