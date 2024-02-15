use ratatelm::widgets::Widget;
use ratatelm::widgets::EventOrMessage;

use ratatui::widgets::ListItem;

use anyhow::Result;

use std::path::{PathBuf, Path};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileEntry {
    path: PathBuf
}

impl FileEntry {
    pub fn new(path: &Path) -> Self {
        Self{path: path.to_path_buf()}
    }
}

impl <Message> Widget <Message> for FileEntry {
    fn render(&mut self, _: ratatui::prelude::Rect, _: &mut ratatui::Frame) {
        unimplemented!("This should not be used as a widget. The impl Widget is just to  provide the handle_key method.")
    }

    fn handle_key (&mut self, e: crossterm::event::KeyEvent) -> Result<Option<EventOrMessage<Message>>> {
        Ok(match e.code {
            _ => Some(EventOrMessage::Event(e))
        })
    }
}

impl Ord for FileEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&FileEntry> for String {
    fn from(val: &FileEntry) -> Self {
        let raw_s = val.path.to_string_lossy().to_string();
        raw_s.
            strip_prefix("./")
            .map(ToOwned::to_owned)
            .unwrap_or(raw_s)
    }
}

impl ToString for FileEntry {
    fn to_string(&self) -> String {
        String::from(self)
    }
}

impl <'a> From<FileEntry> for ListItem<'a> {
    fn from(val: FileEntry) -> Self {
        ratatui::text::Text::from(val.to_string()).into()
    }
}

impl From<FileEntry> for PathBuf {
    fn from(val: FileEntry) -> Self {
        val.path
    }
}