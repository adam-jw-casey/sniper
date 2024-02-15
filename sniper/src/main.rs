mod model;
use model::{Sniper, SniperMode};

mod controller;
use controller::{update, handle_key, Message};

mod view;
use view::view;

mod widgets;
use widgets::SearchBar;

use ratatelm::{App, Widget};

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
        !matches!(self.mode, SniperMode::Quit)
    }

    fn update(&mut self, msg: Message) -> Result<Option<Message>> {
        update(self, msg)
    }

    fn view(&mut self, f: &mut Frame) {
        view(self, f);
    }

    fn handle_key(&self, key: event::KeyEvent) -> Result<Option<Message>> {
        Ok(handle_key(self, key))
    }

    fn focused_widget(&mut self) -> &mut dyn Widget<Message> {
        match self.mode{
            SniperMode::Navigating => &mut self.file_list,
            SniperMode::Searching =>  &mut self.search_bar,
            SniperMode::Quit => panic!("App should terminate before reaching here"),
        }
    }

    fn on_err(s: String) -> Message {
        Message::Error(s)
    }
}

fn main() {
    let args = Args::parse();

    let mut app = Sniper::new(args.path);

    app.run().expect("This should be fine");
}

#[cfg(test)]
mod tests {
    use super::Sniper;
    use ratatelm::App;
    use crossterm::event::{Event, KeyEvent, KeyCode, KeyModifiers};

    // This test addresses an issue that occured where the selection cursor was on the nth file/dir
    // in a directory, and the dir was changed to. The target dir has fewer items that source, so
    // the cursor is now at an invalid index
    #[test]
    fn test_change_dir_fewer_files_in_target_than_previous_selection_does_not_panic() {
        let mut app = Sniper::new("./test".into()); // This is a test directory with a known file structure

        let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));

        // The dir has ., .., two files, then a subdir
        // This scrolls down to the subdir
        app.handle_event(down.clone()).unwrap();
        app.handle_event(down.clone()).unwrap();
        app.handle_event(down.clone()).unwrap();
        app.handle_event(down.clone()).unwrap();

        let enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
        app.handle_event(enter).unwrap();

        // This line should not panic
        app.handle_event(down).unwrap();
    }
}
