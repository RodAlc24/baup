use chrono::Local;
use colored::Colorize;
use std::{
    collections::HashSet,
    env::var,
    fs,
    fs::File,
    io::{self, prelude::*, BufReader, Write},
    path::Path,
    process::Command,
};

use crate::config::Config;
use crate::{
    args::{ClearOptions, EditOptions},
    utils,
};

pub fn edit(config: Config, edit_options: EditOptions, mut _log_file: &mut File) -> io::Result<()> {
    // Get the default editor
    let editor = get_editor();
    if editor.eq("") {
        println!("{} NO editor found", "[ERROR]".bold().red());
    } else {
        let expanded_path = match expanduser::expanduser(if edit_options.open_config {
            "~/.config/baup/config.toml"
        } else {
            &config.path
        }) {
            Ok(path) => path,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Path expansion failed: {err}"),
                ))
            }
        };
        // Create path if it doesn't exist
        handle_path(&expanded_path)?;
        // Opens the file in file_path in the default editor
        Command::new(editor).arg(expanded_path).status()?;
    }
    Ok(())
}

pub fn clear(
    config: Config,
    clear_options: ClearOptions,
    mut _log_file: &mut File,
) -> io::Result<()> {
    let file_str = utils::create_file_struct(&config.path)?;

    // Checks for the partial flag
    if let Some(ref partial) = clear_options.partial {
        match fs::remove_dir_all(format!("{}/{}", file_str.file_path.display(), partial)) {
            Ok(_) => {
                println!(
                    "{} Deleted the {} directory",
                    "[OK]".bold().green(),
                    partial.bold()
                );
                return Ok(());
            }
            Err(err) => {
                println!(
                    "{} Couldn't delete the {} directory",
                    "[ERROR]".bold().red(),
                    partial.bold()
                );
                return Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", err)));
            }
        }
    }

    // Creates a HashSet to handle duplicates
    let mut deleted_directories: HashSet<String> = HashSet::new();

    // Loop for every line in the file opened
    for line in file_str.reader.lines() {
        let line = line?;
        // Check if line is empty or a comment (starts with '#')
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        // Divide the line through the ';'
        let parts: Vec<&str> = line.split(';').collect();

        // Get the original lenght of the HashSet
        let orig_len = deleted_directories.len();
        deleted_directories.insert(parts[1].to_string());

        // If the new len is different to the original the name of the directory is new
        if orig_len != deleted_directories.len() {
            // Delete files
            match fs::remove_dir_all(format!("{}/{}", file_str.file_path.display(), parts[1])) {
                Ok(_) => {
                    println!(
                        "{} Deleted the {} directory",
                        "[OK]".bold().green(),
                        parts[1].bold()
                    );
                }
                Err(err) => {
                    println!(
                        "{} Couldn't delete the {} directory",
                        "[ERROR]".bold().red(),
                        parts[1].bold()
                    );
                    let message = format!(
                        "[{}][CLEAR][{}] <- {:?}\n",
                        Local::now().format("%d-%m-%Y %H:%M:%S"),
                        line,
                        err
                    );
                    let _ = _log_file.write_all(message.as_bytes());
                }
            }
        }
    }

    Ok(())
}

fn get_editor() -> String {
    let editor = match var("EDITOR") {
        // Checks for existance of the EDITOR
        Ok(editor) => editor,
        Err(_) => {
            // In case of error it checks for the nano editor
            let output = Command::new("which").arg("nano").output();
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    if stdout.ne("") {
                        stdout
                    } else {
                        // In case of error it checks for the vim editor
                        let output = Command::new("which").arg("vim").output();
                        match output {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                                if stdout.ne("") {
                                    stdout
                                } else {
                                    "".to_string()
                                }
                            }
                            Err(_) => "".to_string(),
                        }
                    }
                }
                Err(_) => "".to_string(),
            }
        }
    }
    .trim_end_matches('\n')
    .to_string();
    editor
}

fn handle_path(expanded_path: &Path) -> Result<(), io::Error> {
    if let Some(parent) = expanded_path.parent() {
        if !parent.exists() {
            // Create directory if it doesn't exist
            fs::create_dir_all(parent)?;
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path"));
    }
    Ok(())
}
