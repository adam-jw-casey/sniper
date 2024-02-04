mod tui;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
// cargo add anyhow crossterm ratatui
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Default)]
struct Model {
    running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq)]
enum Message {
    Quit,
}

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

/// Render the tui based on the model
fn view(model: &mut Model, f: &mut Frame) {
    f.render_widget(
        todo!(),
        f.size(),
    );
}

/// Convert Event to Message
fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
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
fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
        }
    };
    None
}
