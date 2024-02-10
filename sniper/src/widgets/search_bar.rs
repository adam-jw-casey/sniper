use ratatelm::widgets::Widget;
use ratatelm::widgets::EventOrMessage;

use ratatui::widgets::Paragraph;

use tui_input::{Input, backend::crossterm::EventHandler};
use crossterm::event::{Event, KeyCode};

#[derive(Debug, Clone, Default)]
pub struct SearchBar(Input);

impl SearchBar {
    pub fn clear(&mut self) {
        self.0 = Input::default();
    }
}

impl <Message> Widget <Message> for SearchBar{
    fn render(&mut self, area: ratatui::prelude::Rect, frame: &mut ratatui::Frame) {
        frame.render_widget(
            Paragraph::new(format!("/{}", self.0.value())),
            area,
        );
    }

    fn handle_key (&mut self, e: crossterm::event::KeyEvent, _on_err: Box<dyn Fn(String) -> Message>) -> Option<EventOrMessage<Message>> {
        match e.code {
            KeyCode::Esc | KeyCode::Enter => Some(EventOrMessage::Event(e)),
            _ => {
                self.0.handle_event(&Event::Key(e));
                None
            }
        }
    }
}
