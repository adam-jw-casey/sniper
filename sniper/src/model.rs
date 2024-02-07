use ratatelm::widgets::List;
use crate::controller;

#[derive(Debug)]
pub struct Sniper {
    pub file_list: List<String>,
    pub running: bool,
}

impl Default for Sniper {
    fn default() -> Self {
        Sniper {
            file_list: List::new(
                controller::get_files().expect("Fails on I/O error"),
                "Files".into(),
           ),
            running: true,
        }
    }
}
