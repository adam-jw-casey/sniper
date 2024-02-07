mod list;
pub use list::List;

use ratatui::widgets::Widget as BaseWidget;
use ratatui::prelude::{
    Frame,
    Rect,
};

use crossterm::event::KeyEvent;

pub trait Widget {
    /// Render the widget to the screen and update internal state as necessary
    fn render(&mut self, area: Rect, frame: &mut Frame);
    /// Handle keypress events. If there is no reason to consume the event, it should be returned
    /// wrapped in `Some`.
    /// Otherwise, `None` should be returned
    fn handle_key(&mut self, e: KeyEvent) -> Option<KeyEvent>;
}

/// This is a stateless implementation of the custom widget trait
/// that essentially passes through `ratatui::Widget`.
impl <T: BaseWidget + Clone> Widget for T {
    fn render(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(self.clone(), area);
    }

    /// Stateless, so this is noop
    fn handle_key(&mut self, _: KeyEvent) -> Option<KeyEvent> { None }
}
