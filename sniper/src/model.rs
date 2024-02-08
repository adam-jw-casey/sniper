use ratatelm::widgets::List;
use ratatelm::App;
use crate::Message;

#[derive(Debug)]
pub struct Sniper {
    pub file_list: List<String, Message>,
    pub running: bool,
}

impl  Default for Sniper {
    /// # Impurity
    /// - Performs file I/O via `get_file()`
    fn default() -> Self {
        let mut new = Self {
            file_list: List::new(
               vec![],
               "Files".into(),
               None,
           ),
            running: true,
        };

        new.update(Message::OpenDir(".".into()));

        new
    }
}
