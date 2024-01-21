use args::BaupArgs;
use args::Com;
use clap::Parser;

mod args;
mod config;
mod commands {
    pub mod edit_add_rem;
    pub mod git_diff;
    pub mod import_export;
}

// Main function for the program
fn main() {
    // Gets the config from the file
    let config = config::get_config();

    // Parse the arguments using the clap utility
    let arguments = BaupArgs::parse();
    let _ = match arguments.command {
        Com::Import(options) => commands::import_export::import(config, options),
        Com::Export(options) => commands::import_export::export(config, options),
        Com::Diff => commands::git_diff::diff(config),
        Com::Commit(options) => commands::git_diff::commit(config, options),
        Com::Push(options) => commands::git_diff::push(config, options),
        Com::Pull(options) => commands::git_diff::pull(config, options),
        Com::Edit => commands::edit_add_rem::edit(config),
    };
}
