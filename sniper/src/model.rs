use ratatui::widgets::ListState;

#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub file_list: FileList,
}

#[derive(Debug, Default)]
pub struct FileList {
    pub files: Vec<String>,
    pub state: ListState,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
