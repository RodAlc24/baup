use std::process::Command;
use std::path::Path;
use std::io::{self,prelude::*,BufReader};

pub fn edit(file_path: &str) -> io::Result<()> {
    println!("edit");
    Ok(())
}
