use std::time::Duration;
use std::fs::read_dir;

use crossterm::event::{self, Event, KeyCode};

use anyhow::Result;

use ratatelm::App;

use crate::model::Sniper;

#[derive(PartialEq)]
pub enum Message {
    Quit,
    UpdateFiles,
    ScrollUp,
    ScrollDown,
}

/// Convert Event to Message
pub fn handle_event(model: &Sniper) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

/// Handle keypress events
fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q')  => Some(Message::Quit),
        KeyCode::Up         => Some(Message::ScrollUp),
        KeyCode::Down       => Some(Message::ScrollDown),
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
            model.file_list.files = get_files()
        },
        Message::ScrollUp => {
            let selected = model.file_list.state.selected_mut();
            match selected {
                Some(i) => *i -= 1,
                None => *selected = Some(0),
            }
        },
        Message::ScrollDown => {
            let selected = model.file_list.state.selected_mut();
            match selected {
                Some(i) => *i += 1,
                None => *selected = Some(0),
            }
        },
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
