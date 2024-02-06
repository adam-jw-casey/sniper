#[deny(clippy::pedantic)]

mod tui;

use anyhow::Result;
use ratatui::prelude::Frame;

pub trait App <Message> {
    fn is_running(&self) -> bool;
    /// Convert Event to Message
    fn handle_event(&self) -> Result<Option<Message>>;
    /// Update the model based on a message
    fn update(&mut self, msg: Message) -> Option<Message>;
    /// Render the tui based on the model
    // TODO I don't like that state is mutable here, but that's how ratatui renders stateful widgets
    fn view(&mut self, f: &mut Frame);

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
