use chrono::Local;
use colored::Colorize;
use std::{
    fs::{self, File},
    io::{self, prelude::*, BufReader, Write},
    path::Path,
    process::Command,
};

use crate::args::{DiffOptions, GitOptions};
use crate::config::Config;

fn check_if_git_repo(path: &Path) -> bool {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output();
    match output {
        Ok(output) => return String::from_utf8_lossy(&output.stdout).eq(&String::from("true\n")),
        Err(_) => false,
    }
}

pub fn git(config: Config, arguments: GitOptions, mut _log_file: File) -> io::Result<()> {
    // Get path from the file_path str
    let config_path = expanduser::expanduser(config.path)?;
    let path = match Path::new(&config_path).parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error getting the path for the backup".to_string(),
            ))
        }
    };

    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo {
        // If the command to execute is a commit it adds every file
        if arguments.git_options[0].eq("commit") {
            let _ = Command::new("git")
                .args(["add", "."])
                .current_dir(path)
                .output();
        }
        // Executes git command
        let output = Command::new("git")
            .args(arguments.git_options)
            .current_dir(path)
            .status()
            .unwrap();
        if !output.success() {
            println!("{} Error while calling git", "[ERROR]".bold().red());
            let message = format!(
                "[{}][GIT] <- {:?}\n",
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
                        file_path.display(),
                        parts[1],
                        file_name.last().unwrap()
                    );
                    let output = Command::new("diff")
                        .args([
                            "-u",
                            "--color",
                            &to_path,
                            &expanded_origin.display().to_string(),
                        ])
                        .status()
                        .unwrap();
                    if output.success() {
                        println!(
                            "{} No changes in {}",
                            "[OK]".bold().green(),
                            parts[0].bold()
                        );
                    } else {
                        println!(
                            "{} There are changes in {}",
                            "[OK]".bold().green(),
                            parts[0].bold()
                        );
                    }
                } else if metadata.is_dir() {
                    let to_path = format!("{}/{}/", file_path.display(), parts[1]);
                    let output = Command::new("diff")
                        .args([
                            "-r",
                            "-u",
                            "--color",
                            &to_path,
                            &expanded_origin.display().to_string(),
                        ])
                        .status()
                        .unwrap();
                    if output.success() {
                        println!(
                            "{} No changes in {}",
                            "[OK]".bold().green(),
                            parts[0].bold()
                        );
                    } else {
                        println!(
                            "{} There are changes in {}",
                            "[OK]".bold().green(),
                            parts[0].bold()
                        );
                    }
                } else {
                    println!("{} Couldn't diff {}", "[ERROR]".bold().red(), parts[0]);
                }
            }
            Err(err) => {
                println!("{} Couldn't diff {}", "[ERROR]".bold().red(), parts[0]);
                let message = format!(
                    "[{}][DIFF][{}] <- {:?}\n",
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
