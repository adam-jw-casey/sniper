use ratatelm::widgets::List;
use ratatelm::App;

use crate::{Message, SearchBar, widgets::FileEntry};

#[derive(Debug)]
pub struct Sniper {
    pub file_list: List<FileEntry, Message>,
    pub mode: SniperMode,
    pub search_bar: SearchBar,
    pub message: String,
}

// TODO the Navigating and Searching variants should take String and Input respectively to
// represent either a message or the contents of the search bar.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SniperMode {
    /// The user is navigating through directories.
    /// The variant parameter is a message string.
    Navigating,
    /// The user is searching using the bottom bar.
    Searching,
    /// The app is terminating
    Quit,
}

impl Default for Sniper {
    /// # Impurity
    /// - Performs file I/O via `get_file()`
    fn default() -> Self {
        Self::new(".".into())
    }
}

impl Sniper {
    pub fn new(path: String) -> Self {
        let mut new = Self {
            file_list: List::new(
               vec![],
               "Files".into(),
               None,
           ),
           mode: SniperMode::Navigating,
           search_bar: SearchBar::default(),
           message: String::new(),
        };

        new.update(Message::OpenDir(path.into()))
            .expect("Should be able to open current directory");

        new
    }
}
