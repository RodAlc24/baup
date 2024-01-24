use std::{env::var, fs, io, process::Command};

use crate::args::EditOptions;
use crate::config::Config;

pub fn edit(config: Config, edit_options: EditOptions) -> io::Result<()> {
    // Get the default editor
    let editor = var("EDITOR").unwrap();

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
        let expanded_path = match expanduser::expanduser(&config.path) {
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

    Ok(())
}
