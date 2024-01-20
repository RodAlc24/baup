use std::{
    io,
    process::Command,
    env::var,
};


pub fn edit(file_path: &str) -> io::Result<()> {
    // Expanding user
    let expanded_path = match expanduser::expanduser(file_path){
        Ok(path) => path,
        Err(err) =>return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    // Get the default editor
    let editor = var("EDITOR").unwrap();
    
    // Opens the file in file_path in the default editor
    let _ = Command::new(editor).arg(expanded_path).status();
    
    Ok(())
}
