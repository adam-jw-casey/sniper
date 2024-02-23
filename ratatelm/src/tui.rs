use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{io::stdout, panic};
use anyhow::Result;

use crate::log::dump_logs;

pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}



pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal().unwrap();
        // TODO dealing with logging really shouldn't be in the tui module
        #[cfg(debug_assertions)]
        dump_logs();
        original_hook(panic_info);
    }));
}
