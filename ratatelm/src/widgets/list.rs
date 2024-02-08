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

#[derive(Default)]
/// This `Widget` can be used to display list of items.
/// You can scroll up and down with the arrow keys, and optionally select an item with `Enter` if
/// you set an appropriate callback with `on_select`
pub struct List<T> {
    /// The items displayed by the `List`
    pub elems: Vec<T>,
    /// A title to show at the top of the `List`
    pub title: String,
    /// Function called when an item is selected.
    /// Called with the single argument of the item selected
    #[allow(clippy::type_complexity)]
    select_callback: Option<Box<dyn Fn(&T)>>,
    state: ListState,
}

impl <T> List<T> {
    /// Create a new `List` with the passed elements and title
    #[must_use] pub fn new (elems: Vec<T>, title: String) -> Self {
        Self {
            elems,
            title,
            state: ListState::default(),
            select_callback: None,
        }
    }

    /// Set a callback
    #[allow(clippy::return_self_not_must_use)]
    pub fn on_select<F: Fn(&T) + 'static> (mut self, select_callback: F) -> Self {
        self.select_callback = Some(Box::new(select_callback));
        self
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
        );
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
                    // don't scroll below the number of items
                    Some(i) => *i = min(*i+1, self.elems.len() - 1),
                    None => *selected = Some(0),
                };
                None
            },
            KeyCode::Enter => {
                match &self.select_callback {
                    Some(cb) => match self.state.selected() {
                        Some(index) => cb(&self.elems[index]),
                        None => unimplemented!("TBD what to do if the user presses enter with no item selected"),
                    },
                    None => {},
                };
                None
            }
            _ => Some(e) // Let the next widget handle the keypress
        }
    }
}

impl <T: std::fmt::Debug> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("elems", &self.elems)
            .field("title", &self.title)
            .field("state", &self.state)
            .field("select_callback", &self.select_callback.as_ref().map(|_f| "Anonymous function"))
            .finish()
    }
}
