use clap::{Args,Parser,Subcommand};

#[derive(Debug,Parser)]
#[clap(author, version, about="An easy way to make backups")]
pub struct BaupArgs{
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command{
    /// Imports the config files to the backup directory
    Import(ImportOptions),
    /// Exports the config files from the backup directory
    Export,
    /// Compares the files in the backup directory with the original files
    Diff,
    /// Commits all the changes in the backup directory using git
    Commit(CommitOptions),
    /// Pushes (using git) any commits in the local repository
    Push(PushOptions),
    /// Pulls (using git) any commits to the local repository
    Pull(PullOptions),
    /// Opens the file in which you specify the import/export paths and names
    Edit,
}

#[derive(Debug,Args)]
pub struct ImportOptions{
    /// Imports only one part of the files
    #[arg(short='p',long="partial")]
    pub partial: String,
}

#[derive(Debug, Args)]
pub struct CommitOptions {
    /// Variable number of commit arguments
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub commit_options: Vec<String>,
}

#[derive(Debug, Args)]
pub struct PushOptions {
    /// Variable number of commit arguments
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub commit_options: Vec<String>,
}

#[derive(Debug, Args)]
pub struct PullOptions {
    /// Variable number of commit arguments
    #[arg(num_args = 1.., allow_hyphen_values = true)]
    pub commit_options: Vec<String>,
}
