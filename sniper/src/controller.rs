use std::path::{PathBuf, Path};
use std::env::set_current_dir;

use crossterm::event::{self, KeyCode};

use crate::model::{Sniper, SniperMode};
use crate::widgets::FileEntry;

use anyhow::Result;

#[derive(PartialEq, Eq, Clone)]
pub enum Message  {
    Quit,
    OpenDir(PathBuf),
    OpenFile(PathBuf),
    OpenPath(PathBuf),
    Error(String),
    EnterMode(SniperMode),
}

/// Handle keypress events
pub fn handle_key (model: &Sniper, key: event::KeyEvent) -> Option<Message> {
    match &model.mode {
        SniperMode::Navigating => match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('r') => Some(Message::OpenDir(".".into())),
            KeyCode::Char('/') => Some(Message::EnterMode(SniperMode::Searching)),
            _ => None,
        },
        // The actual input handling is handled by the SearchBar widget
        SniperMode::Searching => match key.code {
            KeyCode::Esc | KeyCode::Enter => Some(Message::EnterMode(SniperMode::Navigating)),
            _ => None,
        },
        SniperMode::Quit => panic!("The app should have already terminated before reaching here"),
    }
}

/// Update the model based on a message
///
/// # Impurity
/// This is (deliberately) the most impure function in this codebase.
pub fn update (model: &mut Sniper, msg: Message) -> Result<Option<Message>> {
    Ok(match msg {
        Message::Quit => {
            model.mode = SniperMode::Quit;
            None
        },
        Message::OpenPath(path) => {
            Some(if path.is_dir() {
                Message::OpenDir(path)
            } else if path.is_file() {
                Message::OpenFile(path)
            } else {
                Message::Error(format!("Unable to open {} - unknown file type", path.to_string_lossy()))
            })
        },
        Message::OpenFile(file_path) => {
            opener::open(file_path)?;
            None
        },
        Message::OpenDir(dir_path) => {
            set_current_dir(dir_path)?;
            model.file_list.elems = get_file_entries(Path::new("."))?;
            model.file_list.clear();
            None
        },
        Message::Error(err_string) => {
            model.message = err_string;
            None
        },
        Message::EnterMode(mode) => {
            model.mode = mode;
            match mode {
                SniperMode::Navigating => model.message = String::new(),
                SniperMode::Searching => model.search_bar.clear(),
                //Setting the mode is sufficient.
                //Termination is handled outside this function
                SniperMode::Quit => {},
            }
            None
        },
    })
}

fn get_file_entries(path: &Path) -> Result<Vec<FileEntry>> {
    let mut files: Vec<FileEntry> = get_files(path)?
        .iter()
        .map(|pb| FileEntry::new(pb))
        .collect();
    files.sort();
    Ok(files)
}

/// Impurity:
///     I/O - reads file names
///
fn get_files(path: &Path) -> Result<Vec<PathBuf>> {
    [".".into(), "..".into()].map(Ok)
        .into_iter()
            .chain(
                path.read_dir()?
                .map(|entry| Ok(entry?.path()))
                )
            .collect()
}

#[cfg(test)]
mod tests {

    use super::{get_files, get_file_entries};
    use std::path::Path;

    // Get all files in this and surrounding (parent and children) directories,
    // and checks that all are returned in alphabetical order.
    #[test]
    fn test_files_sorted() {
        get_files(Path::new(".")).expect("Should be able to do file I/O")
            .iter()
             // Filter to folders in "."
            .filter(|pb| pb.is_dir())
             // Map to all the files in that folder
            .map(|dir| get_file_entries(dir).expect("Should be able to do file I/O"))
            .for_each(|files| {
                let mut files_sorted = files.clone();
                files_sorted.sort();
                assert_eq!(files, files_sorted);
            });
    }
}
