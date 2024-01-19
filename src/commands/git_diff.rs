use std::fs::{File, self};
use std::process::Command;
use std::path::Path;
use std::io::{self,prelude::*,BufReader};
use colored::Colorize;

use crate::args::*;

fn check_if_git_repo(path: &Path) -> bool {
    let output = Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).current_dir(path).output().unwrap();
    println!("Output: {}",String::from_utf8_lossy(&output.stdout));
    println!("Output: {}",String::from_utf8_lossy(&output.stderr));
    true
}

pub fn commit(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    let _ = Command::new("git").args(["add","."]).current_dir(path).output();
    let output = Command::new("git").arg("commit").args(arguments.commit_options).current_dir(path).output().unwrap();
    println!("{}",String::from_utf8_lossy(&output.stdout));

    Ok(())
}

pub fn push(file_path: &str, arguments: PushOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    let output = Command::new("git").arg("push").args(arguments.commit_options).current_dir(path).output().unwrap();
    println!("{}",String::from_utf8_lossy(&output.stdout));

    Ok(())
}


pub fn pull(file_path: &str, arguments: PullOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    let output = Command::new("git").arg("pull").args(arguments.commit_options).current_dir(path).output().unwrap();
    println!("{}",String::from_utf8_lossy(&output.stdout));

    Ok(())
}


pub fn diff(file_path: &str) -> io::Result<()> {
    println!("Diff");
    Ok(())
}
