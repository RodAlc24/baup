use fs_extra::dir;
use std::fs::File;
use std::io::{self,prelude::*,BufReader};
use crate::Cli;

pub fn import(file_path : &str,cli: &Cli) -> io::Result<()> {

    // Options for copying
    let options = dir::CopyOptions{
    overwrite: true,
    skip_exist: false,
    buffer_size: 64000,
    copy_inside: false,
    content_only: false,
    depth: 0,
    };
    
    // Opens file and checks if the file is correctly opened
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Loop for every line in the file opened
    for line in reader.lines(){
        let line = line?;
        // Check if line is empty or a comment (starts with '#')
        if line.trim().is_empty() || line.trim().starts_with('#'){
            continue;
        }
        // Divide the line through the ';'
        let parts: Vec<&str> = line.split(';').collect();
        
        println!("{} {}", parts[0],parts[1]);
    }


    // let mut from_paths = Vec::new();
    // from_paths.push("/home/imanol/.config/zathura/zathurarc");    
    // from_paths.push("/home/imanol/.config/helix/");    
    // let res = fs_extra::copy_items(&from_paths,"/home/imanol/Documents/prueba",&options);
    
    Ok(())
}
