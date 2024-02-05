use ratatui::{
    prelude::{Frame, Modifier, Style},
    widgets::{List, Block, Borders},
};

use crate::model::Model;

/// Render the tui based on the model
// TODO I don't like that state is mutable here, but that's how ratatui renders stateful widgets
pub fn view(model: &mut Model, f: &mut Frame) {
    f.render_stateful_widget(
        List::new(model.file_list.files.clone())
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        f.size(),
        &mut model.file_list.state,
    );
}