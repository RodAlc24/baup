use args::BaupArgs;
use args::Com;
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use std::{fs, fs::OpenOptions, io::Write, path::Path};

mod args;
mod config;
mod commands {
    pub mod edit_clean;
    pub mod git_diff;
    pub mod import_export;
}

// Main function for the program
fn main() {
    // Gets the config from the file
    let config = config::get_config();

    // Opens the log file in append mode
    let log_file = match expanduser::expanduser("~/.cache/baup/baup.log") {
        Ok(file) => Some(file.to_string_lossy().into_owned()),
        Err(_) => {
            println!("{} Couldn't open the log file", "[ERROR]".bold().red());
            None
        }
    };
    if log_file.is_some() {
        let log_file_dir = log_file.unwrap();
        let log_dir = Path::new(&log_file_dir).parent();
        let _ = fs::create_dir_all(log_dir.unwrap());
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&log_file_dir)
            .unwrap();

        // Parse the arguments using the clap utility
        let arguments = BaupArgs::parse();
        let res = match arguments.command {
            Com::Import(options) => commands::import_export::import(config, options, log_file),
            Com::Export(options) => commands::import_export::export(config, options, log_file),
            Com::Diff(options) => commands::git_diff::diff(config, options, log_file),
            Com::Commit(options) => commands::git_diff::commit(config, options, log_file),
            Com::Push(options) => commands::git_diff::push(config, options, log_file),
            Com::Pull(options) => commands::git_diff::pull(config, options, log_file),
            Com::Edit(options) => commands::edit_clean::edit(config, options, log_file),
            Com::Clear(options) => commands::edit_clean::clear(config, options, log_file),
        };

        match res {
            Ok(_) => (),
            Err(err) => {
                let mut log_file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(log_file_dir)
                    .unwrap();
                let time = Local::now().format("%d-%m-%Y %H:%M:%S");
                let message = format!("[{}] [ERROR] <- {:?}", time, err);
                let _ = log_file.write_all(message.as_bytes());
            }
        }
    }
}
