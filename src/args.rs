use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about = "An easy way to make backups")]
pub struct BaupArgs {
    #[clap(subcommand)]
    pub command: Com,
}

#[derive(Debug, Subcommand)]
pub enum Com {
    /// Imports the config files to the backup directory
    Import(ImportOptions),
    /// Exports the config files from the backup directory
    Export(ExportOptions),
    /// Calls the git command in the backups folder
    Git(GitOptions),
    /// Opens the file in which you specify the import/export paths and names
    Edit(EditOptions),
    /// Compares the files in the backup directory with the original files
    Diff(DiffOptions),
    /// Deletes files in the backups folder
    Clear(ClearOptions),
}

#[derive(Debug, Args)]
pub struct ImportOptions {
    /// Imports only one part of the files
    #[arg(short = 'p', long = "partial", value_name = "DIR", required = false)]
    pub partial: Option<String>,
    /// Automatically creates a commit with the name of the files that have changes
    #[arg(short = 'c', long = "auto-commit", required = false)]
    pub auto_commit: bool,
    /// Creates a zip file with all the files after backing them up
    #[arg(short = 'z', long = "zip", required = false)]
    pub create_zip: bool,
}

#[derive(Debug, Args)]
pub struct ExportOptions {
    /// Exports only one part of the files
    #[arg(short = 'p', long = "partial", value_name = "DIR", required = false)]
    pub partial: Option<String>,
}

#[derive(Debug, Args)]
pub struct GitOptions {
    /// Variable number of arguments for the git command
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub git_options: Vec<String>,
}

#[derive(Debug, Args)]
pub struct EditOptions {
    /// Open config file instead of backups file
    #[arg(short = 'c', long = "config", required = false)]
    pub open_config: bool,
}

#[derive(Debug, Args)]
pub struct DiffOptions {
    /// Exports only one part of the files
    #[arg(short = 'p', long = "partial", value_name = "DIR", required = false)]
    pub partial: Option<String>,
    /// Interactive waits for user input when there are changes
    #[arg(short = 'i', long = "interactive", required = false)]
    pub interactive: bool,
}

#[derive(Debug, Args)]
pub struct ClearOptions {
    /// Deltes only one part of the files
    #[arg(short = 'p', long = "partial", value_name = "DIR", required = false)]
    pub partial: Option<String>,
}
