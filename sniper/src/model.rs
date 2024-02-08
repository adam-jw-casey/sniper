use ratatelm::widgets::List;
use crate::{controller, Message};

#[derive(Debug)]
pub struct Sniper {
    pub file_list: List<String, Message>,
    pub running: bool,
}

impl Default for Sniper {
    fn default() -> Self {
        Self {
            file_list: List::new(
                controller::get_files().expect("Fails on I/O error"),
                "Files".into(),
           ),
            running: true,
        }
    }
}
