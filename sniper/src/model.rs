use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct Sniper {
    pub file_list: FileList,
    pub running: bool,
}

impl Default for Sniper {
    fn default() -> Self {
        Sniper {
            file_list: FileList::default(),
            running: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct FileList {
    pub files: Vec<String>,
    pub state: ListState,
}
