mod tui;
pub mod widgets;

use widgets::Widget;

use anyhow::Result;
use ratatui::prelude::Frame;

use crossterm::event::{self, Event};
use std::time::Duration;

pub trait App <Message> {
    /// Check if the app is still running or should quit
    fn is_running(&self) -> bool;

    /// Handle keypress events
    fn handle_key(key: event::KeyEvent) -> Option<Message>;

    /// Update the model based on a message
    fn update(&mut self, msg: Message) -> Option<Message>;

    /// Render the tui based on the model
    // TODO I don't like that state is mutable here, but that's how ratatui renders stateful widgets
    fn view(&mut self, f: &mut Frame);

    /// Get the widget(s) that is(are) focused.
    /// If there are multiple (e.g., a text box inside a frame), the widget on top (in this case,
    /// the text box) is at the beginning of the returned Vec
    fn focused_widgets(&mut self) -> Vec<&mut dyn Widget>;

    /// Convert Event to Message
    /// The file list gets first dibs to consume keypress events
    ///
    /// # Errors
    /// Returns `Err` when unable to read event
    fn handle_event(&mut self) -> Result<Option<Message>> {
        if !event::poll(Duration::from_millis(250))? {
            return Ok(None)
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let mut maybe_key = Some(key);
                for w in &mut self.focused_widgets() {
                    match maybe_key {
                        Some(key) => {maybe_key = w.handle_key(key);},
                        None => break,
                    }
                }

                Ok(maybe_key.and_then(|key| Self::handle_key(key)))
            // TODO The next few lines do not spark joy
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Runs the application to termination or panic
    ///
    /// # Errors
    /// Passes through errors from `self.view()` and `self.handle_event()`
    fn run (&mut self) -> Result<()>{
        tui::install_panic_hook();
        let mut terminal = tui::init_terminal().expect("Should be able to initialize terminal");

        while self.is_running() {
            // Render the current view
            terminal.draw(|f| self.view(f))?;

            // Handle events and map to a Message
            let mut current_msg = self.handle_event()?;

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = self.update(current_msg.unwrap());
            }
        }

        tui::restore_terminal()?;

        Ok(())
    }
}
