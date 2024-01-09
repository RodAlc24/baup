use std::fs::File;
use std::io::{self,prelude::*,BufReader};
use crate::Cli;

pub fn import(file_path : &str,cli: &Cli) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines(){
        println!("{}", line?);
    }

    Ok(())
}
