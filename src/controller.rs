use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::model::{Model, RunningState};

#[derive(PartialEq)]
pub enum Message {
    Quit,
}

/// Convert Event to Message
pub fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
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
        _ => None,
    }
}

/// Update the model based on a message
pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
    };
    None
}
