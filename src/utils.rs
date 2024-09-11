use chrono::Local;
use std::{fs::File, io::Write};

pub fn write_to_log(command: &str, message: String, mut _log_file: &mut File) {
    let message = format!(
        "[{}][{}] <- {:?}\n",
        Local::now().format("%d-%m-%Y %H:%M:%S"),
        command,
        message
    );
    let _ = _log_file.write_all(message.as_bytes());
}

pub fn write_to_log_with_line(
    command: &str,
    line: String,
    message: String,
    mut _log_file: &mut File,
) {
    let message = format!(
        "[{}][{}][{}] <- {:?}\n",
        Local::now().format("%d-%m-%Y %H:%M:%S"),
        command,
        line,
        message
    );
    let _ = _log_file.write_all(message.as_bytes());
}
