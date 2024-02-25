mod list;
pub use list::List;

use ratatui::widgets::Widget as BaseWidget;
use ratatui::prelude::{
    Frame,
    Rect,
};

use crossterm::event::KeyEvent;
use anyhow::Result;

/// Simple wrapper for either a `KeyEvent` or user-defined Message type
pub enum EventOrMessage <Message> {
    /// Self-explanatory
    Event(KeyEvent),
    /// Self-explanatory
    Message(Message),
}

/// The `Widget` trait allows a type to be rendered and to handle keypress events within an `App`
/// application. It is automatically implemented for all `ratatui::widgets::Widget` types.
pub trait Widget <Message> {
    /// Render the widget to the screen and update internal state as necessary
    fn render(&mut self, area: Rect, frame: &mut Frame);

    /// Handle keypress events. If there is no reason to consume the event, it should be returned
    /// wrapped in `Some`.
    /// Otherwise, `None` should be returned
    ///
    /// # Errors
    /// Any errors that occur should be propagated
    fn handle_key (&mut self, e: KeyEvent) -> Result<Option<EventOrMessage<Message>>>;
}

/// This is a stateless implementation of the custom widget trait
/// that essentially passes through `ratatui::Widget`.
impl <T: BaseWidget + Clone, Message> Widget <Message> for T {
    fn render(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(self.clone(), area);
    }

    /// Stateless, so this is noop
    fn handle_key (&mut self, _: KeyEvent) -> Result<Option<EventOrMessage<Message>>> { Ok(None) }
}
