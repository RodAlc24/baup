use std::fs::File;
use std::io::{self,prelude::*,BufReader};
use crate::Cli;

pub fn import(file_path : &str,cli: &Cli) -> io::Result<()> {

    // Opens file and checks if the file is correctly opened
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Loop for every line in the file opened
    for line in reader.lines(){
        println!("{}", line?);
    }

    Ok(())
}
