mod args;

use chrono::Local;
use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
};

pub fn log_error(
    command: &str,
    message: String,
    mut _log_file: &mut File,
) -> Result<(), io::Error> {
    // Formats the message
    let message = format!("[{}][ERROR][{}] <- {}\n", get_time(), command, message);

    // Writes the message to the file
    _log_file.write_all(message.as_bytes())?;
    Ok(())
}

pub fn log_warning(
    command: &str,
    message: String,
    mut _log_file: &mut File,
) -> Result<(), io::Error> {
    // Formats the message
    let message = format!("[{}][WARNING][{}] <- {}\n", get_time(), command, message);

    // Writes the message to the file
    _log_file.write_all(message.as_bytes())?;
    Ok(())
}

pub fn log_debug(
    command: &str,
    message: String,
    mut _log_file: &mut File,
    debug: bool,
) -> Result<(), io::Error> {
    // Checks for the debug flag
    if !debug {
        Ok(())
    }

    // Formats the message
    let message = format!("[{}][DEBUG][{}] <- {}\n", get_time(), command, message);

    // Writes the message to the file
    _log_file.write_all(message.as_bytes())?;
    Ok(())
}

fn get_time() {
    Local::now().format("%d-%m-%Y %H:%M:%S")
}
