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

use crate::args::{ClearOptions, EditOptions};
use crate::config::Config;

pub fn edit(config: Config, edit_options: EditOptions, mut _log_file: &mut File) -> io::Result<()> {
    // Get the default editor
    let editor = get_editor();
    if editor.eq("") {
        println!("{} NO editor found", "[ERROR]".bold().red());
    } else {
        if edit_options.open_config {
            // Expanding user
            let expanded_path = match expanduser::expanduser("~/.config/baup/config.toml") {
                Ok(path) => path,
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            };

            // Create path if it doesn't exist
            if !expanded_path.parent().unwrap().exists() {
                fs::create_dir_all(expanded_path.parent().unwrap().display().to_string())?;
            }
            // Opens the file in file_path in the default editor
            let _ = Command::new(editor).arg(expanded_path).status();
        } else {
            // Expanding user
            let expanded_path = match expanduser::expanduser(config.path) {
                Ok(path) => path,
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            };

            // Create path if it doesn't exist
            if !expanded_path.parent().unwrap().exists() {
                fs::create_dir_all(expanded_path.parent().unwrap().display().to_string())?;
            }

            // Opens the file in file_path in the default editor
            let _ = Command::new(editor).arg(expanded_path).status();
        }
    }
    Ok(())
}

pub fn clear(
    config: Config,
    clear_options: ClearOptions,
    mut _log_file: &mut File,
) -> io::Result<()> {
    // Opens file and checks if the file is correctly opened
    let config_file_expanded = expanduser::expanduser(config.path)?;
    let file = File::open(config_file_expanded.clone())?;
    let reader = BufReader::new(file);

    // Get path from the file_path str
    let file_path = match Path::new(&config_file_expanded).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error getting the path for the backup".to_string(),
            ))
        }
    };

    // Checks for the partial flag
    if let Some(ref partial) = clear_options.partial {
        match fs::remove_dir_all(format!("{}/{}", file_path.display(), partial)) {
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
    for line in reader.lines() {
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
            match fs::remove_dir_all(format!("{}/{}", file_path.display(), parts[1])) {
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
            let output = Command::new("which").arg("vim").output();
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    if stdout.ne("") {
                        stdout
                    } else {
                        let output = Command::new("which").arg("nano").output();
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
