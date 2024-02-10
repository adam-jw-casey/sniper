mod model;
use model::{Sniper, SniperMode};

mod controller;
use controller::{update, handle_key, Message};

mod view;
use view::view;

use ratatelm::App;

use crossterm::event;
use ratatui::Frame;

use anyhow::Result;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Path to the directory to open in. "." by default
    #[arg(short, long, default_value=".")]
    path: String,
}

impl App<Message> for Sniper {
    fn is_running(&self) -> bool {
        matches!(self.mode, SniperMode::Quit)
    }

    fn update(&mut self, msg: Message) -> Result<Option<Message>> {
        update(self, msg)
    }

    fn view(&mut self, f: &mut Frame) {
        view(self, f);
    }

    fn handle_key(key: event::KeyEvent) -> Option<Message> {
        handle_key(key)
    }

    fn focused_widgets(&mut self) -> Vec<&mut dyn ratatelm::widgets::Widget<Message>> {
        vec![&mut self.file_list]
    }

    fn on_err(s: String) -> Message {
        Message::Error(s)
    }
}

fn main() {

    let args = Args::parse();

    let mut app = Sniper::new(args.path);
    app.file_list.on_select(|s| Message::OpenPath(s.into()));

    app.run().expect("This should be fine");
}
