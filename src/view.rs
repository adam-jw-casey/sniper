use ratatui::{
    prelude::Frame,
    widgets::Paragraph,
};

use crate::model::Model;

/// Render the tui based on the model
pub fn view(model: &mut Model, f: &mut Frame) {
    f.render_widget(
        Paragraph::new("hello, world"),
        f.size(),
    );
}
