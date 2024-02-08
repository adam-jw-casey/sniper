use ratatui::prelude::Frame;

use crate::model::Sniper;
use ratatelm::widgets::Widget;

/// Render the tui based on the model
// TODO make this more ergonomic from ratatelm
pub fn view(model: &mut Sniper, f: &mut Frame) {
    model.file_list.render(f.size(), f);
}
