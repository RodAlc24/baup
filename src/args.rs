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
    /// Commits all the changes in the backup directory using git
    Commit(CommitOptions),
    /// Pushes (using git) any commits in the local repository
    Push(PushOptions),
    /// Pulls (using git) any commits to the local repository
    Pull(PullOptions),
    /// Opens the file in which you specify the import/export paths and names
    Edit(EditOptions),
    /// Compares the files in the backup directory with the original files
    Diff,
}

#[derive(Debug, Args)]
pub struct ImportOptions {
    /// Imports only one part of the files
    #[arg(short = 'p', long = "partial", value_name = "DIR", required = false)]
    pub partial: Option<String>,
    /// Automatically creates a commit with the name of the files that have changes
    #[arg(short = 'c', long = "auto-commit", required = false)]
    pub auto_commit: bool,
}

#[derive(Debug, Args)]
pub struct ExportOptions {
    /// Exports only one part of the files
    #[arg(short = 'p', long = "partial", value_name = "DIR", required = false)]
    pub partial: Option<String>,
}

#[derive(Debug, Args)]
pub struct CommitOptions {
    /// Variable number of arguments for the git commit command
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub commit_options: Vec<String>,
}

#[derive(Debug, Args)]
pub struct PushOptions {
    /// Variable number of arguments for the git push command
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub commit_options: Vec<String>,
}

#[derive(Debug, Args)]
pub struct PullOptions {
    /// Variable number of arguments for the git pull command
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub commit_options: Vec<String>,
}

#[derive(Debug, Args)]
pub struct EditOptions {
    /// Open config file instead of backups file
    #[arg(short = 'c', long = "config", required = false)]
    pub open_config: bool,
}
