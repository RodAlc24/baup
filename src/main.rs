use args::BaupArgs;
use args::Com;
use clap::Parser;
use colored::Colorize;
use std::io;
use std::{fs, fs::OpenOptions, path::Path};

mod args;
mod config;
mod utils;
mod commands {
    pub mod edit_clean;
    pub mod git_diff;
    pub mod import_export;
}

// Main function for the program
fn main() -> io::Result<()> {
    // Gets the config from the file
    let config = config::get_config();

    // Expands the location for the baup log file
    let log_file = match expanduser::expanduser("~/.cache/baup/baup.log") {
        Ok(file) => Some(file),
        Err(_) => None,
    };
    if log_file.is_some() {
        // If there is no error, creates (if necessary) the directory for the log file
        let log_file_dir = log_file.unwrap();
        let log_dir = match Path::new(&log_file_dir).parent() {
            Some(path) => path,
            None => {
                println!("{} Couldn't open the log file", "[ERROR]".bold().red());
                return Ok(());
            }
        };
        let _ = fs::create_dir_all(log_dir);
        // Opens the log file in append mode
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&log_file_dir)?;

        // Parse the arguments using the clap utility
        let arguments = BaupArgs::parse();
        let command: &str;
        let res = match arguments.command {
            Com::Import(options) => {
                command = "IMPORT";
                commands::import_export::import(config, options, &mut log_file)
            }
            Com::Export(options) => {
                command = "EXPORT";
                commands::import_export::export(config, options, &mut log_file)
            }
            Com::Diff(options) => {
                command = "DIFF";
                commands::git_diff::diff(config, options, &mut log_file)
            }
            Com::Git(options) => {
                command = "GIT";
                commands::git_diff::git(config, options, &mut log_file)
            }
            Com::Edit(options) => {
                command = "EDIT";
                commands::edit_clean::edit(config, options, &mut log_file)
            }
            Com::Clear(options) => {
                command = "CLEAR";
                commands::edit_clean::clear(config, options, &mut log_file)
            }
        };

        match res {
            Ok(_) => (),
            Err(err) => {
                println!(
                    "{} Error calling the command: {}",
                    "[ERROR]".bold().red(),
                    command
                );
                utils::write_to_log(command, err.to_string(), &mut log_file);
            }
        }
    } else {
        println!("{} Couldn't open the log file", "[ERROR]".bold().red());
    }
    Ok(())
}
