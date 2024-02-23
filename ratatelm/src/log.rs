use crossterm::event::Event;

// TODO these should be #cfg[(debug_assertions)]
use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref LOGS: Mutex<Vec<Event>> = Vec::new().into();

    static ref LOGFILE_NAME: Mutex<String> = {
        let mut sys = sysinfo::System::new();
        sys.refresh_all();

        format!(
            "./logs/{}.log",
            sys
                .process(sysinfo::get_current_pid().expect("Should be able to current current pid"))
                .expect("Should be able to get current process")
                .start_time()
        )
    }.into();
}

/// # Panics
/// Panics if unable to write to file or access system process information 
pub fn dump_logs() { // TODO this shouldn't actually be public beyond the crate
    std::fs::write(
        LOGFILE_NAME
            .lock()
            .expect("The logfile name should have been initialized and not yet consumed")
            .clone(),
        format!("{:#?}", LOGS.lock().unwrap())
    ).expect("Should be able to write to file");
}

/// Records the event to a logfile for use in later debugging
#[cfg(debug_assertions)]
pub fn log(event: &Event) {
    LOGS.lock().unwrap().push(event.clone());
}

/// Reads the next event back from the logfile
#[cfg(debug_assertions)]
#[must_use] pub fn delog() -> Event {
    todo!()
}
