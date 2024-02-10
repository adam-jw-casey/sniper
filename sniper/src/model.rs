use ratatelm::widgets::List;
use ratatelm::App;
use crate::Message;

#[derive(Debug)]
pub struct Sniper {
    pub file_list: List<String, Message>,
    pub mode: SniperMode,
    pub message: String,
}

#[derive(Debug, Copy, Clone)]
pub enum SniperMode {
    Navigating,
    Searching,
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
           message: String::new(),
        };

        new.update(Message::OpenDir(path.into()))
            .expect("Should be able to open current directory");

        new
    }
}
