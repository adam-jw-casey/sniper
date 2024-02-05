#[deny(clippy::pedantic)]

mod tui;

use anyhow::Result;
use ratatui::prelude::Frame;

pub trait App <Model: Default, Message> {
    fn is_running(&self) -> bool;
    /// Setup the state before the application launches
    fn setup(&mut self);
    /// Convert Event to Message
    fn handle_event(model: &Model) -> Result<Option<Message>>;
    /// Update the model based on a message
    fn update(model: &mut Model, msg: Message) -> Option<Message>;
    /// Render the tui based on the model
    // TODO I don't like that state is mutable here, but that's how ratatui renders stateful widgets
    fn view(model: &mut Model, f: &mut Frame);

    fn run (&mut self, model: &mut Model) -> Result<()>{
        tui::install_panic_hook();
        let mut terminal = tui::init_terminal().expect("Should be able to initialize terminal");

        self.setup();

        while self.is_running() {
            // Render the current view
            terminal.draw(|f| Self::view(model, f))?;

            // Handle events and map to a Message
            let mut current_msg = Self::handle_event(model)?;

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = Self::update(model, current_msg.unwrap());
            }
        }

        tui::restore_terminal()?;

        Ok(())
    }
}
