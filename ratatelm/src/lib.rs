//! `ratatelm` is an [elm](https://guide.elm-lang.org/architecture/)-like framework based on [ratatui](https://ratatui.rs/)

mod tui;
/// Contains the `Widget` trait and all customs widgets defined in ratatelm
pub mod widgets;

use widgets::{Widget, EventOrMessage};

use anyhow::Result;
use ratatui::prelude::Frame;

use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

/// The `App` trait is the entry point to a `ratatelm` application.
///
/// To use, implement `App` for your type, then, in you main function, initialize your `App` and
/// call `run`
pub trait App <Message> {
    /// Check if the app is still running or should quit
    fn is_running(&self) -> bool;

    /// Handle keypress events
    fn handle_key(key: event::KeyEvent) -> Option<Message>;

    /// Update the model based on a message
    /// # Errors
    /// This is implemented by the user.
    fn update(&mut self, msg: Message) -> Result<Option<Message>>;

    /// Render the tui based on the model
    // TODO I don't like that state is mutable here, but that's how ratatui renders stateful widgets
    fn view(&mut self, f: &mut Frame);

    /// Get the widget(s) that is(are) focused.
    /// If there are multiple (e.g., a text box inside a frame), the widget on top (in this case,
    /// the text box) is at the beginning of the returned Vec
    fn focused_widgets(&mut self) -> Vec<&mut dyn Widget<Message>>;

    /// This function is called when a minor error occurs that the user should be notified of.
    /// It generates a message that will be handled by the application as normal.
    fn on_err(s: String) -> Message;

    /// Convert Event to Message
    ///
    /// # Errors
    /// Returns `Err` when unable to read event
    fn handle_event(&mut self) -> Result<Option<Message>> {
        if !event::poll(Duration::from_millis(250))? {
            return Ok(None)
        }

        Ok(if let Event::Key(key) = event::read()? {
            self.handle_key_event(key)
        } else {
            None
        })
    }

    /// Handle key events
    /// First, pass events to any focused widget(s)
    /// If the event is not consumed, pass to `self::handle_key`()
    fn handle_key_event(&mut self, key: KeyEvent) -> Option<Message> {
        (key.kind == event::KeyEventKind::Press).then_some({
            let mut maybe_key = Some(key);

            // Iterate over any focused widget(s), checking in order if they wish to consume the
            // key event. This is guaranteed to run at least once.
            for w in &mut self.focused_widgets() {
                match maybe_key {
                    Some(key) => {
                        maybe_key = match w.handle_key(key, Box::new(|s| {Self::on_err(s)})){
                            Some(e_or_m) => match e_or_m {
                                // If the widget's `handle_key` returned a key event, continue
                                // processing
                                EventOrMessage::Event(event) => Some(event),
                                // If the widget's `handle_key` returned a message, we're done
                                // processing and can return the message.
                                EventOrMessage::Message(message) => return Some(message),
                            },
                            // This would be tidier with `Option::map`, but the closure would not
                            // be able to return from the overall function, which is necessary above.
                            None => None,
                        }
                    },
                    // If the key event has been consumed, exit the loop
                    None => break,
                }
            }

            maybe_key.and_then(|key| Self::handle_key(key))?
        })
    }

    /// Runs the application to termination or panic
    ///
    /// # Errors
    /// Passes through errors from `self.view()` and `self.handle_event()`
    fn run (&mut self) -> Result<()>{
        tui::install_panic_hook();
        let mut terminal = tui::init_terminal()
            .expect("Should be able to initialize terminal");

        while self.is_running() {
            // Render the current view
            terminal.draw(|f| self.view(f))?;

            // Handle events and map to a Message
            let mut current_msg = self.handle_event()?;

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = self
                    // The unwrap is ok because we've already checked this is some
                    .update(current_msg.unwrap())
                    .unwrap_or_else(|e| Some(Self::on_err(e.to_string())));
            }
        }

        tui::restore_terminal()?;

        Ok(())
    }
}
