use std::fs::{File, self};
use std::process::Command;
use std::path::Path;
use std::io::{self,prelude::*,BufReader};
use clap::Command;
use colored::Colorize;

use crate::args::*;

pub fn commit(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    let _ = Command::new("git").args(["add","."]).current_dir(path).output();
    let output = Command::new("git").arg("commit").args(arguments.commit_options).current_dir(path).output();

    Ok(())
}

pub fn push(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    let output = Command::new("git").arg("push").args(arguments.commit_options).current_dir(path).output();

    Ok(())
}


pub fn pull(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    // Get path from the file_path str
    let path = match Path::new(file_path).parent(){
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::Other,format!("Error getting the path for the backup"))) ,
    };
    
    let output = Command::new("git").arg("pull").args(arguments.commit_options).current_dir(path).output();

    Ok(())
}


pub fn diff(file_path: &str, arguments: CommitOptions) -> io::Result<()> {
    Ok(())
}
