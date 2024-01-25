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
    if log_file != None {
        let log_file = log_file.unwrap();
        let log_dir = Path::new(&log_file).parent();
        let _ = fs::create_dir_all(log_dir.unwrap());
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file)
            .unwrap();

        // Parse the arguments using the clap utility
        let arguments = BaupArgs::parse();
        let res = match arguments.command {
            Com::Import(options) => commands::import_export::import(config, options),
            Com::Export(options) => commands::import_export::export(config, options),
            Com::Diff => commands::git_diff::diff(config),
            Com::Commit(options) => commands::git_diff::commit(config, options),
            Com::Push(options) => commands::git_diff::push(config, options),
            Com::Pull(options) => commands::git_diff::pull(config, options),
            Com::Edit(options) => commands::edit_clean::edit(config, options),
            Com::Clear(options) => commands::edit_clean::clear(config, options),
        };

        match res {
            Ok(_) => return,
            Err(err) => {
                let time = Local::now().format("%d-%m-%Y %H:%M:%S");
                let message = format!("[{}] [ERROR] <- {:?}", time, err);
                let _ = log_file.write_all(message.as_bytes());
                return;
            }
        }
    }
}
