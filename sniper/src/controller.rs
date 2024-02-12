use std::path::{PathBuf, Path};
use std::env::set_current_dir;
use std::borrow::ToOwned;

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
pub fn update (model: &mut Sniper, msg: Message) -> Result<Option<Message>> {
    Ok(match msg {
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
                Message::Error(format!("Unable to open {} - unknown file type", path.to_string_lossy()))
            })
        },
        Message::OpenFile(file_path) => {
            opener::open(file_path)?;
            None
        },
        Message::OpenDir(dir_path) => {
            set_current_dir(dir_path)?;
            model.file_list.elems = get_file_names(Path::new("."))?;
            None
        },
        Message::Error(err_string) => {
            model.err_message = err_string;
            None
        },
    })
}

fn get_file_names(path: &Path) -> Result<Vec<String>> {
    Ok(get_files(path)?
        .iter()
        .map(|pb| file_display(pb))
        .collect())
}

/// Convert files to display format
//
//This feels like an impl Display
fn file_display(path: &Path) -> String {
    let raw_s = path.to_string_lossy().to_string();
    raw_s.
        strip_prefix("./")
        .map(ToOwned::to_owned)
        .unwrap_or(raw_s)
}

/// Impurity:
///     I/O - reads file names
///
fn get_files(path: &Path) -> Result<Vec<PathBuf>> {
    [".".into(), "..".into()].map(Ok)
        .into_iter()
            .chain(
                path.read_dir()?
                .map(|entry| Ok(entry?.path()))
                )
            .collect()
}
