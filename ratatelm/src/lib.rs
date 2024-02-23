//! `ratatelm` is an [elm](https://guide.elm-lang.org/architecture/)-like framework based on [ratatui](https://ratatui.rs/)

mod tui;
#[cfg(debug_assertions)]
mod log;
/// Contains the `Widget` trait and all customs widgets defined in ratatelm
pub mod widgets;

pub use widgets::{Widget, EventOrMessage};

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
    ///
    /// # Errors
    /// This is implemented by the user. Errors should be passed through to the caller.
    fn handle_key(&self, key: event::KeyEvent) -> Result<Option<Message>>;

    /// Update the model based on a message
    ///
    /// # Errors
    /// This is implemented by the user. Errors should be passed through to the caller.
    fn update(&mut self, msg: Message) -> Result<Option<Message>>;

    /// Render the tui based on the model
    // TODO I don't like that state is mutable here, but that's how ratatui renders stateful widgets
    fn view(&mut self, f: &mut Frame);

    /// Get the widget(s) that is(are) focused.
    /// If there are multiple (e.g., a text box inside a frame), the widget on top (in this case,
    /// the text box) is at the beginning of the returned Vec
    fn focused_widget(&mut self) -> &mut dyn Widget<Message>;

    /// This function is called when a minor error occurs that the user should be notified of.
    /// It generates a message that will be handled by the application as normal.
    fn on_err(s: String) -> Message;

    /// Updates the app state based on key events, using the user-implemented `update` method.
    ///
    /// # Errors
    /// Passes through I/O errors from crossterm, and any errors that happen during event handling.
    fn handle_event(&mut self, event: Event) -> Result<()> {

        if let Event::Key(key_event) = event {
            let mut current_msg = self.handle_key_event(key_event)?;

            // Process updates as long as they return a non-None message
            while let Some(msg) = current_msg {
                current_msg = self
                    .update(msg)
                    .unwrap_or_else(|e| Some(Self::on_err(e.to_string())));
            }
        }

        Ok(())
    }

    /// Polls for crossterm events. If a key event is found, returns it.
    /// Waits for up to 250 ms for an event to occur.
    ///
    /// # Errors
    /// Passes through I/O errors from crossterm
    fn get_event() -> Result<Option<Event>> {
        Ok(
            if event::poll(Duration::from_millis(250))? {
                Some(event::read()?)
            } else {
                None
            }
        )
    }

    /// Handle key events
    /// First, pass events to any focused widget(s)
    /// If the event is not consumed, pass to `self::handle_key`()
    ///
    /// # Errors
    /// Passes through errors from the user-implemented `handle_key`
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Message>> {
        Ok(if key.kind == event::KeyEventKind::Press {
            // Offer the key event to the focused widget
            let maybe_key = match self.focused_widget().handle_key(key)?{
                Some(e_or_m) => match e_or_m {
                    // If the widget's `handle_key` returned a key event, continue
                    // processing
                    EventOrMessage::Event(event) => Some(event),
                    // If the widget's `handle_key` returned a message, we're done
                    // processing and can return the message.
                    EventOrMessage::Message(message) => return Ok(Some(message)),
                },
                // This would be tidier with `Option::map`, but the closure would not
                // be able to return from the overall function, which is necessary above.
                None => None,
            };

            // Then give sniper the chance to handle the key event
            match maybe_key{
                Some(key) => self.handle_key(key)?,
                None => None,
            }
        } else { None })
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
            if let Some(event) = Self::get_event()?{
                #[cfg(debug_assertions)] {
                    log::log(&event);
                }

                self.handle_event(event)?;
            }
        }

        tui::restore_terminal()?;

        #[cfg(debug_assertions)]
        log::dump_logs();

        Ok(())
    }

    /// Like `run`, but consumes events from a logfile rather than getting them from crossterm
    ///
    /// # Errors
    /// Passes through errors from `view` and `handle_event`
    fn run_from_logs(&mut self, logfile_name: String) -> Result<()> {

        tui::install_panic_hook();
        let mut terminal = tui::init_terminal()
            .expect("Should be able to initialize terminal");

        let events: Vec<Event> = log::delog(std::path::Path::new(&logfile_name));

        for event in events {
            // Render the current view
            terminal.draw(|f| self.view(f))?;

            // Handle events and map to a Message
            self.handle_event(event)?;
        }

        tui::restore_terminal()?;

        Ok(())
    }
}
