use ratatelm::widgets::List;
use ratatelm::App;
use crate::Message;

#[derive(Debug)]
pub struct Sniper {
    pub file_list: List<String, Message>,
    pub running: bool,
    pub err_message: String,
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
           running: true,
           err_message: String::new(),
        };

        new.update(Message::OpenDir(path_str))
            .expect("Should be able to open current directory");

        new
    }
}
