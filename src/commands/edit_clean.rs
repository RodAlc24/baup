use std::{env::var, fs, io, process::Command};

use crate::config::Config;

pub fn edit(config: Config) -> io::Result<()> {
    // Expanding user
    let expanded_path = match expanduser::expanduser(&config.path.unwrap()) {
        Ok(path) => path,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    // Create path if it doesn't exist
    if !expanded_path.parent().unwrap().exists() {
        fs::create_dir_all(expanded_path.parent().unwrap().display().to_string())?;
    }

    // Get the default editor
    let editor = var("EDITOR").unwrap();

    // Opens the file in file_path in the default editor
    let _ = Command::new(editor).arg(expanded_path).status();

    Ok(())
}
