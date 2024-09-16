use colored::Colorize;
use std::{
    fs::{self, File},
    io::{self, prelude::*, stdin, stdout, BufReader},
    path::{Path, PathBuf},
    process::Command,
};

use crate::args::{DiffOptions, GitOptions};
use crate::config::Config;
use crate::utils;

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

fn run_diff_command(
    to_path: String,
    expanded_origin: PathBuf,
    file_name: &str,
    interactive: bool,
) -> io::Result<()> {
    let output = Command::new("diff")
        .args([
            "-r",
            "-u",
            "--color",
            &to_path,
            &expanded_origin.display().to_string(),
        ])
        .status()
        .expect(&format!(
            "{} Error while calling diff",
            "[ERROR]".bold().red()
        ));
    if output.success() {
        println!(
            "{} No changes in {}",
            "[OK]".bold().green(),
            file_name.bold()
        );
    } else {
        println!(
            "{} There are changes in {}",
            "[OK]".bold().green(),
            file_name.bold()
        );
        if (interactive) {
            print!("Press any key to continue... ");
            stdout().flush()?;
            match stdin().read_line(&mut String::new()) {
                Ok(_) => (),
                Err(err) => {
                    println!("");
                    return Err(err.into());
                }
            }
        }
    }

    Ok(())
}

pub fn git(config: Config, arguments: GitOptions, mut _log_file: &mut File) -> io::Result<()> {
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
    if check_if_git_repo(path) {
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
            .status();
        match output {
            Ok(_) => (),
            Err(res) => {
                println!("{} Error while calling git", "[ERROR]".bold().red());
                utils::write_to_log("GIT", res.to_string(), _log_file);
            }
        }
        // if !output.success() {
        // println!("{} Error while calling git", "[ERROR]".bold().red());
        // utils::write_to_log("GIT", output.to_string(), _log_file);
        // }
    } else {
        println!("{} Directory is not a git repo", "[ERROR]".bold().red());
    }

    Ok(())
}

pub fn diff(config: Config, diff_options: DiffOptions, mut _log_file: &mut File) -> io::Result<()> {
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
                    run_diff_command(to_path, expanded_origin, parts[0], diff_options.interactive)?;
                } else if metadata.is_dir() {
                    let to_path = format!("{}/{}/", file_path.display(), parts[1]);
                    run_diff_command(to_path, expanded_origin, parts[0], diff_options.interactive)?;
                } else {
                    println!("{} Couldn't diff {}", "[ERROR]".bold().red(), parts[0]);
                }
            }
            Err(err) => {
                println!("{} Couldn't diff {}", "[ERROR]".bold().red(), parts[0]);
                utils::write_to_log_with_line("DIFF", line, err.to_string(), _log_file);
            }
        }
    }

    Ok(())
}
