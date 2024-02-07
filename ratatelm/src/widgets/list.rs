use super::Widget;

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

use std::cmp::min;

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

impl <T> Widget for List <T>
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
                    Some(i) => *i = i.saturating_sub(1), // don't go below 0
                    None => *selected = Some(0),
                };
                None
            },
            KeyCode::Down => {
                let selected = self.state.selected_mut();
                match selected {
                    Some(i) => *i = min(*i+1, self.elems.len() - 1), // don't scroll below the number of items
                    None => *selected = Some(0),
                };
                None
            },
            _ => Some(e) // Let the next widget handle the keypress
        }
    }
}
