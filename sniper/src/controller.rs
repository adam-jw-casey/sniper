use std::fs::read_dir;

use crossterm::event::{self, KeyCode};

use crate::model::Sniper;

use anyhow::Result;

#[derive(PartialEq)]
pub enum Message {
    Quit,
    UpdateFiles,
}

/// Handle keypress events
pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('r') => Some(Message::UpdateFiles),
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
            model.file_list.elems = get_files().expect("Fails on I/O errors")
        }
    };
    None
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
