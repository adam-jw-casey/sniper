use ratatui::widgets::Widget as BaseWidget;
use ratatui::prelude::{
    Frame,
    Rect,
    Style,
    Modifier,
};
use ratatui::widgets::{
    ListState,
    ListItem,
    List as BaseList,
    Block,
    Borders,
};

use crossterm::event::{KeyEvent, KeyCode};

pub trait Widget {
    /// Render the widget to the screen and update internal state as necessary
    fn render(&mut self, area: Rect, frame: &mut Frame);
    /// Handle keypress events. If there is no reason to consume the event, it should be returned
    /// wrapped in `Some`.
    /// Otherwise, `None` should be returned
    fn handle_key(&mut self, e: KeyEvent) -> Option<KeyEvent>;
}

/// This is a stateless implementation of the custom widget trait
/// that essentially passes through ratatui::Widget.
impl <T: BaseWidget + Clone> Widget for T {
    fn render(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(self.clone(), area);
    }

    /// Stateless, so this is noop
    fn handle_key(&mut self, _: KeyEvent) -> Option<KeyEvent> { None }
}

#[derive(Debug, Default)]
pub struct List<T> {
    pub elems: Vec<T>,
    pub title: String,
    state: ListState,
}

impl <T> List <T> {
    pub fn new(elems: Vec<T>, title: String) -> Self {
        Self {
            elems,
            title,
            state: ListState::default(),
        }
    }
}

impl <T> Widget  for List <T> 
where
for<'a> T: Into<ListItem<'a>> + Clone
{
    fn render(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_stateful_widget(
            BaseList::new(self.elems.clone())
                .block(Block::default().title(self.title.as_str()).borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .repeat_highlight_symbol(true),
            area,
            &mut self.state,
        )
    }

    fn handle_key(&mut self, e: KeyEvent) -> Option<KeyEvent> {
        match e.code {
            KeyCode::Up => {
                let selected = self.state.selected_mut();
                match selected {
                    Some(i) => *i -= 1,
                    None => *selected = Some(0),
                };
                None
            },
            KeyCode::Down => {
                let selected = self.state.selected_mut();
                match selected {
                    Some(i) => *i += 1,
                    None => *selected = Some(0),
                };
                None
            },
            _ => Some(e)
        }
    }
}
