use clap::Parser;
use args::BaupArgs;
use args::Com;

mod args;
mod commands {
    pub mod import_export;
    pub mod git_diff;
    pub mod edit_add_rem;
}

// Main function for the program
fn main() {
    // Parse the arguments using the clap utility
    let arguments = BaupArgs::parse();
    let _ = match arguments.command {
        Com::Import(options) => commands::import_export::import("/home/imanol/.baup/files.txt",options),
        Com::Export(options) => commands::import_export::export("/home/imanol/.baup/files.txt",options),
        Com::Diff => commands::git_diff::diff("/home/imanol/.baup/files.txt"),
        Com::Commit(options) => commands::git_diff::commit("/home/imanol/.baup/files.txt", options),
        Com::Push(options) => commands::git_diff::push("/home/imanol/.baup/files.txt", options),
        Com::Pull(options) => commands::git_diff::pull("/home/imanol/.baup/files.txt", options),
        Com::Edit => commands::edit_add_rem::edit("/home/imanol/.baup/files.txt"),
    };
}
