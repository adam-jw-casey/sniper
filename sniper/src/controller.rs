use std::fs::read_dir;

use crossterm::event::{self, KeyCode};

use crate::model::Sniper;

use anyhow::Result;

#[derive(PartialEq, Eq, Clone)]
pub enum Message {
    Quit,
    UpdateFiles,
    OpenFile(String),
    Error(String),
}

/// Handle keypress events
pub const fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('r') => Some(Message::UpdateFiles),
        _ => None,
    }
}

/// Update the model based on a message
///
/// # Impurity
/// `Quit`        - terminates the program
/// `UpdateFiles` - modifies model and can panic
/// `OpenFile`    - interacts with external applications
/// `Error`       - unimplemented, but will display info to user
pub fn update(model: &mut Sniper, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running = false;
            None
        }
        Message::UpdateFiles => {
            model.file_list.elems = get_files().expect("Fails on I/O errors");
            None
        }
        Message::OpenFile(file_name) => match opener::open(file_name) {
            Ok(()) => None,
            Err(e) => Some(Message::Error(e.to_string())),
        },
        Message::Error(err_string) => {dbg!(format!("{err_string} - you need to write a better error handler")); None},
    }
}

/// Impurity:
///     I/O - reads file names
///     Panics
///
pub fn get_files() -> Result<Vec<String>> {
    read_dir(".")?
        .map(|entry| Ok(entry?.file_name().to_string_lossy().to_string()))
        .collect()
}
