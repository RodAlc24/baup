use chrono::Local;
use colored::Colorize;
use std::{
    fs::{self, File},
    io::{self, prelude::*, BufReader, Write},
    path::Path,
    process::Command,
};

use crate::args::{CommitOptions, DiffOptions, PullOptions, PushOptions};
use crate::config::Config;

fn check_if_git_repo(path: &Path) -> bool {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output();
    match output {
        Ok(output) => return String::from_utf8_lossy(&output.stdout).eq(&String::from("true\n")),
        Err(_) => return false,
    }
}

pub fn commit(config: Config, arguments: CommitOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(&config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Creates the commit
        let _ = Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .output();
        let output = Command::new("git")
            .arg("commit")
            .args(arguments.commit_options)
            .current_dir(path)
            .status()
            .unwrap();
        if output.success() {
            println!("{} Created commit successfully", "[OK]".bold().green());
        } else {
            println!("{} Couldn't create commit", "[ERROR]".bold().red());
            let message = format!(
                "[{}][COMMIT] <- {:?}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                output
            );
            let _ = _log_file.write_all(message.as_bytes());
        }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn push(config: Config, arguments: PushOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(&config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Push using git
        let output = Command::new("git")
            .arg("push")
            .args(arguments.commit_options)
            .current_dir(path)
            .status()
            .unwrap();
        if output.success() {
            println!("{} Pushed successfully", "[OK]".bold().green());
        } else {
            println!("{} Couldn't pushed successfully", "[ERROR]".bold().red());
            let message = format!(
                "[{}][PUSH] <- {:?}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                output
            );
            let _ = _log_file.write_all(message.as_bytes());
        }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn pull(config: Config, arguments: PullOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(&config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Pull using git
        let output = Command::new("git")
            .arg("pull")
            .args(arguments.commit_options)
            .current_dir(path)
            .status()
            .unwrap();
        if output.success() {
            println!("{} Pulled successfully", "[OK]".bold().green());
        } else {
            println!("{} Couldn't pulled successfully", "[ERROR]".bold().red());
            let message = format!(
                "[{}][PULL] <- {:?}",
                Local::now().format("%d-%m-%Y %H:%M:%S"),
                output
            );
            let _ = _log_file.write_all(message.as_bytes());
        }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn diff(config: Config, diff_options: DiffOptions, mut _log_file: File) -> io::Result<()> {
    // Opens file and checks if the file is correctly opened
    let config_file_expanded = expanduser::expanduser(&config.path)?;
    let file = File::open(config_file_expanded.clone())?;
    let reader = BufReader::new(file);

    // Get path from the file_path str
    let file_path = match Path::new(&config_file_expanded).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error getting the path for the backup"),
            ))
        }
    };

    // Loop for every line in the file
    for line in reader.lines() {
        let line = line?;

        // Check if line is empty or a comment (starts with '#')
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        // Divide the line through the ';'
        let parts: Vec<&str> = line.split(';').collect();
        let expanded_origin = expanduser::expanduser(parts[0])?;

        // Checks for the partial flag
        if let Some(ref partial) = diff_options.partial {
            if partial.ne(parts[1]) {
                continue;
            }
        }

        // Checking wheter the origin is a file or directory
        match fs::metadata(expanded_origin.clone()) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let file_name: Vec<&str> = parts[0].split('/').collect();
                    let to_path = format!(
                        "{}/{}/{}",
                        file_path.display().to_string(),
                        parts[1],
                        file_name.last().unwrap()
                    );
                    Command::new("diff")
                        .args([
                            "-u",
                            "--color",
                            &to_path,
                            &expanded_origin.display().to_string(),
                        ])
                        .status()
                        .unwrap();
                } else if metadata.is_dir() {
                    println!("Dir");
                } else {
                    println!("{} Couldn't diff {}", "[ERROR]".bold().red(), parts[0]);
                }
            }
            Err(err) => {
                println!("{} Couldn't diff {}", "[ERROR]".bold().red(), parts[0]);
                let message = format!(
                    "[{}][DIFF][{}] <- {:?}",
                    Local::now().format("%d-%m-%Y %H:%M:%S"),
                    line,
                    err
                );
                let _ = _log_file.write_all(message.as_bytes());
            }
        }
    }

    Ok(())
}
