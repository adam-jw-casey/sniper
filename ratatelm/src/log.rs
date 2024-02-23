use crossterm::event::Event;
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
                .expect("Should be able to get current process.")
                .start_time()
        )
    }.into();
}

/// # Panics
/// Panics if unable to write to file or access system process information 
pub fn dump_logs() {
    std::fs::write(
        LOGFILE_NAME
            .lock()
            .expect("The logfile name should have been initialized and not yet consumed.")
            .clone(),
        serde_json::ser::to_string(&*LOGS.lock().expect("Should be able to get lock on log mutex."))
            .expect("Should be able to serialize logs")
    ).expect("Should be able to write to file.");
}

/// Records the event to a logfile for use in later debugging
pub fn log(event: &Event) {
    LOGS
        .lock()
        .expect("Should be able to get lock on log mutex.")
        .push(event.clone());
}

/// Reads the next event back from the logfile
#[must_use] pub fn delog (path: &std::path::Path) -> Vec<Event> {
    let file = std::fs::File::open(path).expect("Should be able to open logfile.");
    let reader = std::io::BufReader::new(file);

    serde_json::from_reader(reader).expect("Should be able to deserialize logfile.")
}
