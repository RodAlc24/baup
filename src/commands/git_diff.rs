use std::fs::{File, self};
use std::process::Command;
use Coloref::Colorize;

mod args;

pub fn commit(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    let path = Path::new(file_path).parent().to_str();
    
    // let output = Command::new("git").args(arguments.commit_options)   

    Ok(())
}
