use clap::{Args,Parser,Subcommand};

#[derive(Debug,Parser)]
#[clap(author, version, about)]
pub struct BaupArgs{
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command{
    /// Imports the config files to the backup directory
    import(ImportOptions),
    /// Exports the config files from the backup directory
    export,
    /// Compares the files in the backup directory with the original files
    diff,
    /// Commits all the changes in the backup directory using git
    commit,
    /// Pushes (using git) any commits in the local repository
    push,
    /// Pulls (using git) any commits to the local repository
    pull,
    /// Opens the file in which you specify the import/export paths and names
    edit,
}

#[derive(Debug,Args)]
pub struct ImportOptions{
    /// Imports only one part of the files
    #[arg(short='p',long="partial")]
    pub partial: String,
}
