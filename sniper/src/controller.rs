use std::time::Duration;
use std::fs::read_dir;

use crossterm::event::{self, Event, KeyCode};

use anyhow::Result;

use crate::model::Sniper;

use ratatelm::widgets::Widget;

#[derive(PartialEq)]
pub enum Message {
    Quit,
    UpdateFiles,
}

/// Convert Event to Message
/// The file list gets first dibs to consume keypress events
pub fn handle_event(model: &mut Sniper) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(
                    model.file_list.handle_key(key)
                      .and_then(handle_key)
                );
            }
        }
    }
    Ok(None)
}

/// Handle keypress events
fn handle_key(key: event::KeyEvent) -> Option<Message> {
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
