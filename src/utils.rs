use chrono::Local;
use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
};

pub struct FileStruct {
    pub file_path: PathBuf,
    pub reader: BufReader<File>,
}

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

pub fn create_file_struct(path: &str) -> Result<FileStruct, io::Error> {
    // Opens file and checks if the file is correctly opened
    let config_file_expanded = expanduser::expanduser(path)?;
    let file = File::open(config_file_expanded.clone())?;
    let reader = BufReader::new(file);

    // Get path from the file_path str
    let file_path = match Path::new(&config_file_expanded).parent() {
        Some(path) => path.to_path_buf(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error getting the path for the backup".to_string(),
            ))
        }
    };
    Ok(FileStruct { file_path, reader })
}
