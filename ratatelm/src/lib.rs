#[deny(clippy::pedantic)]

mod tui;
pub use tui::

mod view;
pub use view::view;

mod model;
pub use model::{Model, RunningState};

mod controller;
pub use controller::{handle_event, update};
pub use controller::Message;
