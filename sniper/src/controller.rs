use std::fs::read_dir;

use crossterm::event::{self, KeyCode};

use crate::model::Sniper;

#[derive(PartialEq)]
pub enum Message {
    Quit,
    UpdateFiles,
}

/// Handle keypress events
pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q')  => Some(Message::Quit),
        _ => None,
    }
}

/// Update the model based on a message
pub fn update(model: &mut Sniper, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running = false;
        }
        Message::UpdateFiles => {
            model.file_list.elems = get_files()
        }
    };
    None
}

/// Impurity:
///     I/O - reads file names
///     Panics
///
// TODO should return a result
pub fn get_files() -> Vec<String> {
    read_dir(".")
        .expect("fails on IO error")
        .map(|entry| entry.expect("fails on IO error").file_name().to_string_lossy().to_string())
        .collect()
}
