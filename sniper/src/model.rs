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
    pub fn new(path_str: String) -> Self {
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

        new.file_list.on_select(|s| Message::OpenPath(s.to_string()));

        new.update(Message::OpenDir(path_str))
            .expect("Should be able to open current directory");

        new
    }
}
