use super::{Widget,EventOrMessage};

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
use anyhow::Result;

use std::cmp::min;
use std::io::{Error, ErrorKind};

#[derive(Default)]
/// This `Widget` can be used to display list of items.
/// You can scroll up and down with the arrow keys, and optionally select an item with `Enter` if
/// you set an appropriate callback with `on_select`
pub struct List<Elem, Message> {
    /// The items displayed by the `List`
    pub elems: Vec<Elem>,
    /// A title to show at the top of the `List`
    pub title: String,
    /// Function called when an item is selected.
    /// Called with the single argument of the item selected
    #[allow(clippy::type_complexity)]
    select_message: Option<Box<dyn Fn(Elem) -> Message>>,
    state: ListState,
}

impl <Elem, Message> List <Elem, Message> {
    /// Create a new `List` with the passed elements and title
    #[must_use] pub fn new (elems: Vec<Elem>, title: String, select_message: Option<Box<dyn Fn(Elem) -> Message>> ) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self { elems, title, select_message, state }
    }

    /// Set a callback for selecting an item
    pub fn on_select (&mut self, select_message: impl Fn(Elem) -> Message + 'static) {
        self.select_message = Some(Box::new(select_message));
    }
}

impl <Elem, Message> Widget<Message> for List<Elem, Message>
where
for<'a> Elem: Clone + Into<ListItem<'a>> + Widget<Message>
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

    fn handle_key (&mut self, mut key: KeyEvent) -> Result<Option<EventOrMessage<Message>>> {
        if let Some(index) = self.state.selected() {
            key = match self.elems[index].handle_key(key)? {
                Some(EventOrMessage::Event(key)) => key,
                Some(EventOrMessage::Message(m)) => return Ok(Some(EventOrMessage::Message(m))),
                None => return Ok(None),
            };
        }

        Ok(match key.code {
            KeyCode::Up => {
                let selected = self.state.selected_mut();
                match selected {
                    // don't go below 0
                    Some(i) => *i = i.saturating_sub(1),
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
                // If there is no selection handler, nothing to do
                match self.select_message.as_ref() {
                    Some(select) => {
                        // If nothing is selected, warn the user if possible
                        self.state.selected().map_or_else(
                            // If there is no defined error handler, print with the dbg! macro
                            || Err(Error::new(ErrorKind::NotFound, "No item selected")),
                            |index| Ok(Some(EventOrMessage::Message(select(self.elems[index].clone())))),
                        )?
                    },
                    None => None,
                }
            }
            _ => Some(EventOrMessage::Event(key)) // Let the next widget handle the keypress
        })
    }
}

impl <Elem: std::fmt::Debug, Message> std::fmt::Debug for List<Elem, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("elems", &self.elems)
            .field("title", &self.title)
            .field("state", &self.state)
            .field("select_message", &self.select_message.as_ref().map(|_f| "Anonymous function"))
            .field("err_message", &self.select_message.as_ref().map(|_f| "Anonymous function"))
            .finish()
    }
}
