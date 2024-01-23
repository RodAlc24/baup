use colored::Colorize;
use expanduser;
use fs_extra::dir;
use std::{
    fs::{self, File},
    io::{self, prelude::*, BufReader},
    path::Path,
    process::Command,
};

use crate::args::{CommitOptions, ExportOptions, ImportOptions};
use crate::commands::git_diff;
use crate::config::Config;

pub fn import(config: Config, import_options: ImportOptions) -> io::Result<()> {
    // Options for copying
    let options = dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: false,
        content_only: false,
        depth: 0,
    };

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

    // Executes the hook (if exists)
    if config.hooks.import_hook != None {
        let hook_file =
            expanduser::expanduser(config.hooks.import_hook.as_ref().unwrap().to_string()).unwrap();
        Command::new("sh")
            .arg(hook_file)
            .status()
            .expect("Error executing the hook");
    }

    // Loop for every line in the file opened
    for line in reader.lines() {
        let line = line?;
        // Check if line is empty or a comment (starts with '#')
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        // Divide the line through the ';'
        let parts: Vec<&str> = line.split(';').collect();

        // Checks for the partial flag
        if let Some(ref partial) = import_options.partial {
            if partial.ne(parts[1]) {
                continue;
            }
        }

        // Get all files in the directory (for copying)
        let paths = match get_files_from_path(parts[0]) {
            Ok(paths) => paths,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        // Getting from locations
        let from_paths: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        // Getting the new location for the files
        let copy_path = format!("{}/{}", file_path.display(), parts[1]);
        // Creating (if necessary) the directory for the files
        fs::create_dir_all(copy_path.clone())?;
        // Copying the files
        let res = fs_extra::copy_items(&from_paths, copy_path, &options);
        match res {
            Ok(_) => {
                println!(
                    "{} Copied {} to {}",
                    "[OK]".bold().green(),
                    parts[0].bold(),
                    parts[1].bold()
                );
            }
            Err(err) => {
                println!(
                    "{} Couldn't copy {} to {}",
                    "[ERROR]".bold().red(),
                    parts[0].bold(),
                    parts[1].bold()
                )
            }
        }
    }

    // Create the commit if the auto-commit option is enabled
    if import_options.auto_commit || config.auto_commit {
        let commit_msg = get_changed_files(&file_path.display().to_string());
        if commit_msg.ne("") {
            let options = CommitOptions {
                commit_options: vec![String::from("-m"), commit_msg],
            };
            git_diff::commit(config, options)?;
        } else {
            println!("{} There are no changes to commit", "[OK]".bold().green());
        }
    }

    Ok(())
}

pub fn export(config: Config, export_options: ExportOptions) -> io::Result<()> {
    // Options for copying
    let options = dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: false,
        content_only: false,
        depth: 0,
    };

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

    // Executes the hook (if exists)
    if config.hooks.export_hook != None {
        let hook_file =
            expanduser::expanduser(config.hooks.export_hook.as_ref().unwrap().to_string()).unwrap();
        Command::new("sh")
            .arg(hook_file)
            .status()
            .expect("Error executing the hook");
    }

    // Loop for every line in the file opened
    for line in reader.lines() {
        let line = line?;
        // Check if line is empty or a comment (starts with '#')
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        // Divide the line through the ';'
        let parts: Vec<&str> = line.split(';').collect();
        let from_paths = match get_files_from_path(&format!("{}/{}", file_path.display(), parts[1]))
        {
            Ok(paths) => paths,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        // Checks for the partial flag
        if let Some(ref partial) = export_options.partial {
            if partial.ne(parts[1]) {
                continue;
            }
        }

        // Expanding user
        let expanded_path = match expanduser::expanduser(parts[0]) {
            Ok(path) => path,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        let is_dir = expanded_path.display().to_string().chars().last().unwrap();

        // Creating, if necessary, the directory for the file or directory
        if !expanded_path.exists() {
            if is_dir == '/' {
                println!("d");
                fs::create_dir_all(&expanded_path)?;
            } else {
                fs::create_dir_all(Path::new(&expanded_path).parent().unwrap())?;
                File::create(&expanded_path)?;
            }
        }

        // Copying files
        match fs::metadata(expanded_path.display().to_string()) {
            Ok(metadata) => {
                if metadata.is_file() {
                    // Get file location
                    let filename = Path::new(parts[0]);
                    let from_path = format!(
                        "{}/{}/{}",
                        file_path.display(),
                        parts[1],
                        filename.file_name().unwrap().to_str().unwrap()
                    );
                    let res = fs::copy(from_path, expanded_path.display().to_string());
                    match res {
                        Ok(_) => {
                            println!(
                                "{} Copied {} to {}",
                                "[OK]".bold().green(),
                                parts[1].bold(),
                                parts[0].bold()
                            );
                        }
                        Err(err) => {
                            println!(
                                "{} Couldn't copy {} to {}",
                                "[ERROR]".bold().red(),
                                parts[1].bold(),
                                parts[0].bold()
                            );
                            println!("Error: {}", err);
                        }
                    }
                } else if metadata.is_dir() {
                    // Creating, if necessary the directory for the file
                    fs::create_dir_all(expanded_path.clone())?;
                    let res = fs_extra::copy_items(&from_paths, expanded_path, &options);
                    match res {
                        Ok(_) => {
                            println!(
                                "{} Copied {} to {}",
                                "[OK]".bold().green(),
                                parts[1].bold(),
                                parts[0].bold()
                            );
                        }
                        Err(err) => {
                            println!(
                                "{} Couldn't copy {} to {}",
                                "[ERROR]".bold().red(),
                                parts[1].bold(),
                                parts[0].bold()
                            );
                            println!("Error: {}", err);
                        }
                    }
                } else {
                    println!(
                        "{} {} is not a file nor a directory",
                        "[ERROR]".bold().red(),
                        parts[1].bold()
                    );
                }
            }
            Err(err) => {
                return Err(io::Error::new(io::ErrorKind::Other, err));
            }
        }
    }

    Ok(())
}

fn get_files_from_path(path: &str) -> Result<Vec<String>, String> {
    // Expanding user
    let expanded_path = match expanduser::expanduser(path) {
        Ok(path) => path,
        Err(err) => return Err(format!("Error expanding user: {}", err)),
    };

    // Process the first part (file/directory/...)
    match fs::metadata(expanded_path.display().to_string()) {
        Ok(metadata) => {
            if metadata.is_file() {
                let ret: Vec<String> = vec![expanded_path.display().to_string()];
                return Ok(ret);
            } else if metadata.is_dir() {
                let files = fs::read_dir(expanded_path.display().to_string()).unwrap();
                let mut ret: Vec<String> = vec![String::from("")];
                ret.pop();
                for path in files {
                    ret.push(path.unwrap().path().display().to_string());
                }
                return Ok(ret);
            } else {
                return Err(format!("Path is neither a file nor a directory"));
            }
        }
        Err(e) => {
            return Err(format!("Error accessing metadata: {}", e));
        }
    }
}

fn get_changed_files(directory: &str) -> String {
    // Get the changed files
    let output = Command::new("sh")
        .arg("-c")
        .arg("git diff --name-only HEAD")
        .current_dir(directory)
        .output()
        .expect("Failed to execute command");

    // Get an array with all the files with changes
    let changed_files = String::from_utf8(output.stdout).unwrap();
    let files: Vec<&str> = changed_files.split('\n').collect();

    // Process the first three filenames
    let mut result = String::new();

    for file in files.iter().take(3) {
        result.push_str(file);
        result.push_str(" ; ");
    }

    // If there are more than 3 files, append "..."
    if files.len() > 3 {
        result.push_str("...");
    }

    return result.trim_end_matches(" ; ").to_string();
}
