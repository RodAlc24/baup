use std::fs::{File, self};
use std::io::{self,prelude::*,BufReader};
use expanduser;
use fs_extra::dir;
use colored::Colorize;

fn get_files_from_path(path: &str) -> Result<Vec<String>, String>{
    // Expanding user
    let mut expanded_path = match expanduser::expanduser(path){
        Ok(path) => path,
        Err(err) => return Err(format!("Error expanding user: {}", err))
    };
    // Removing (if exists) the * at the end
    if expanded_path.display().to_string().chars().last() == Some('*'){
        expanded_path.pop();
    }
    
    // Process the first part (file/directory/...)
    match fs::metadata(expanded_path.display().to_string()){
        Ok(metadata) => {
            if metadata.is_file() {
                let ret: Vec<String> = vec![expanded_path.display().to_string()];
                return Ok(ret);
            } else if metadata.is_dir() {
                let files = fs::read_dir(expanded_path.display().to_string()).unwrap();
                let mut ret: Vec<String> = vec![String::from("")];
                ret.pop();
                for path in files{
                    ret.push(path.unwrap().path().display().to_string());
                }
                return Ok(ret);
            } else {
                return Err(format!("Path is neither a file nor a directory"));
            }
        }
        Err(e) => {
            return Err(format!("Error accessing metadata: {}", e));
        }
    }
}

pub fn import(file_path : &str) -> io::Result<()> {
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
        
        // Get all files in the directory (for copying)
        let paths = match get_files_from_path(parts[0]){
            Ok(paths) => paths,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        // Getting from locations
        let from_paths: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        // Getting the new location for the files
        let copy_path = format!("{}{}","/home/imanol/Documents/prueba/",parts[1]);
        // Creating (if necessary) the directory for the files
        fs::create_dir_all(copy_path.clone())?;
        let res = fs_extra::copy_items(&from_paths,copy_path,&options);
        match res {
            Ok(_) => {
                println!("{} Copied {} to {}","[OK]".bold().green(),parts[0].bold(),parts[1].bold());
            }
            Err(err) => {
                return Err(io::Error::new(io::ErrorKind::Other, format!("Copy error: {:?}", err)));
            }
        }
    }


    
    Ok(())
}

pub fn export(file_path : &str) -> io::Result<()> {
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
        
    }

    Ok(())

}
