use std::fs::{File, self};
use std::io::{self,prelude::*,BufReader};
use std::process::Command;
use crate::Cli;
use expanduser;
use fs_extra::dir;

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
        
        let mut expanded_path = expanduser::expanduser(parts[0])?;

        if expanded_path.display().to_string().chars().last() == Some('*'){
            expanded_path.pop();
        }
        
        // Process the first part (file/directory/...)
        match fs::metadata(expanded_path.display().to_string()){
            Ok(metadata) => {
                if metadata.is_file() {
                    println!("It's a file!");
                } else if metadata.is_dir() {
                    println!("It's a dir!");
                    let files = fs::read_dir(expanded_path.display().to_string()).unwrap();
                    for path in files{
                        println!("{:?}",path.unwrap().path().display());
                    }
                } else {
                    println!("It's neither a file nor a directory.");
                }
            }
            Err(e) => {
                eprintln!("Error getting metadata: {}", e);
            }
        }
        println!("{} {}",parts[0],parts[1]);
    }


    // let mut from_paths = Vec::new();
    // from_paths.push("/home/imanol/.config/zathura/zathurarc");    
    // from_paths.push("/home/imanol/.config/helix/");    
    // let res = fs_extra::copy_items(&from_paths,"/home/imanol/Documents/prueba",&options);
    
    Ok(())
}
