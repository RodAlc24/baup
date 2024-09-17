use args::BaupArgs;
use args::Com;
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use std::io;
use std::{fs, fs::OpenOptions, io::Write, path::Path};

mod args;
mod config;
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
        let res = match arguments.command {
            Com::Import(options) => commands::import_export::import(config, options, &mut log_file),
            Com::Export(options) => commands::import_export::export(config, options, &mut log_file),
            Com::Diff(options) => commands::git_diff::diff(config, options, &mut log_file),
            Com::Git(options) => commands::git_diff::git(config, options, &mut log_file),
            Com::Edit(options) => commands::edit_clean::edit(config, options, &mut log_file),
            Com::Clear(options) => commands::edit_clean::clear(config, options, &mut log_file),
        };

        match res {
            Ok(_) => (),
            Err(err) => {
                let time = Local::now().format("%d-%m-%Y %H:%M:%S");
                let message = format!("[{}] [ERROR] <- {:?}\n", time, err);
                let _ = log_file.write_all(message.as_bytes());
            }
        }
    } else {
        println!("{} Couldn't open the log file", "[ERROR]".bold().red());
    }
    Ok(())
}
