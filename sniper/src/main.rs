#[deny(clippy::pedantic)]

mod model;
use model::Sniper;

mod controller;
use controller::{handle_event, update, Message};

mod view;
use view::view;

use ratatelm::App;
use anyhow::Result;

use ratatui::Frame;

impl App<Message> for Sniper {
    fn is_running(&self) -> bool {
        self.running
    }

    fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(self)
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        update(self, msg)
    }

    fn view(&mut self, f: &mut Frame) {
        view(self, f)
    }
}

fn main() {
    let mut app = Sniper::default();

    app.update(Message::UpdateFiles);

    app.run().expect("This should be fine");
}
