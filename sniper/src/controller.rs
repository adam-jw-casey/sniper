use std::fs::read_dir;

use crossterm::event::{self, KeyCode};

use crate::model::Sniper;

use anyhow::Result;

#[derive(PartialEq, Eq, Clone)]
pub enum Message {
    Quit,
    OpenDir(&'static str),
    OpenFile(String),
    Error(String),
}

/// Handle keypress events
pub const fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('r') => Some(Message::OpenDir(".")),
        _ => None,
    }
}

/// Update the model based on a message
///
/// # Impurity
/// This is (deliberately) the most impure function in this codebase.
pub fn update(model: &mut Sniper, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running = false;
            None
        }
        Message::OpenDir(dir_name) => {
            match get_files(dir_name){
                Ok(files) => {
                    model.file_list.elems = files;
                    None
                },
                Err(e) => Some(Message::Error(e.to_string())),
            }
        }
        Message::OpenFile(file_name) => {
            opener::open(file_name).err()
                .map(|e| Message::Error(e.to_string()))
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
pub fn get_files(path: &str) -> Result<Vec<String>> {
    [".".into(), "..".into()].map(Ok)
        .into_iter()
        .chain(
            read_dir(path)?
                .map(|entry| Ok(entry?.file_name().to_string_lossy().to_string()))
        )
        .collect()
}
