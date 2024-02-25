use ratatui::prelude::{Frame, Layout, Constraint};
use ratatui::widgets::Paragraph;

use crate::model::{Sniper, SniperMode};
use crate::controller::Message;
use ratatelm::widgets::Widget;

/// Render the tui based on the model
// TODO make this more ergonomic from ratatelm
pub fn view(model: &mut Sniper, f: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ]);

    let [main_window, bottom_bar] = vertical.areas(f.size());

    model.file_list.render(main_window, f);

    match model.mode{
        SniperMode::Navigating => Widget::<Message>::render(&mut Paragraph::new(model.message.clone()), bottom_bar, f),
        SniperMode::Searching => Widget::<Message>::render(&mut model.search_bar, bottom_bar, f),
        SniperMode::Quit => panic!("The application should terminate before reaching here"),
    };
}
