use std::fs::{File, self};
use std::process::Command;
use std::path::Path;
use std::io::{self,prelude::*,BufReader};
use colored::Colorize;

use crate::args::*;

fn check_if_git_repo(path: &Path) -> bool {
    let output = Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).current_dir(path).output().unwrap();
    String::from_utf8_lossy(&output.stdout).eq(&String::from("true\n"))
}

pub fn commit(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Creates the commit
        let _ = Command::new("git").args(["add","."]).current_dir(path).output();
        let output = Command::new("git").arg("commit").args(arguments.commit_options).current_dir(path).output().unwrap();
        if output.status.success() {
            println!("{} Created commit successfully","[OK]".bold().green());
        }
        else {
            println!("{} Couldn't create commit","[ERROR]".bold().red());
            println!("Error: {}",String::from_utf8(output.stderr).unwrap());
        }
    }
    else {
        println!("{} Directory is not a git repo","[ERROR]".bold().red());
    }
    
    Ok(())
}

pub fn push(file_path: &str, arguments: PushOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Push using git
        let output = Command::new("git").arg("push").args(arguments.commit_options).current_dir(path).output().unwrap();
        if output.status.success() {
            println!("{} Pushed successfully","[OK]".bold().green());
        }
        else {
            println!("{} Couldn't pushed successfully","[ERROR]".bold().red());
            println!("Error: {}",String::from_utf8(output.stderr).unwrap());
        }
    }
    else {
        println!("{} Directory is not a git repo","[ERROR]".bold().red());
    }
    
    Ok(())
}


pub fn pull(file_path: &str, arguments: PullOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    // Check if there is a git repository
    let repo = check_if_git_repo(path);
    if repo == true {
        // Pull using git
        let output = Command::new("git").arg("pull").args(arguments.commit_options).current_dir(path).output().unwrap();
        if output.status.success() {
            println!("{} Pulled successfully","[OK]".bold().green());
        }
        else {
            println!("{} Couldn't pulled successfully","[ERROR]".bold().red());
            println!("Error: {}",String::from_utf8(output.stderr).unwrap());
        }
    }
    else {
        println!("{} Directory is not a git repo","[ERROR]".bold().red());
    }

    Ok(())
}


pub fn diff(file_path: &str) -> io::Result<()> {
    println!("Diff");
    Ok(())
}
