use std::env;
use clap::Parser;
use args::BaupArgs;

mod args;
mod import_export;

// Struct for the arguments
struct Cli {
    command: String,
    options: Option<String>,
}

// Main function for the program
fn main() {
    let arguments = BaupArgs::parse();
    
    // Collects the arguments
    let args: Vec<String> = env::args().collect();

    // If no arguments, opens help menu
    if args.len() == 1{
        println!("Help");
        return;
    }
    
    // Parse arguments and creates cli variable for storing them
    let command = args[1].clone();
    let options = if args.len() == 2{
        None 
    }
    else {
        Some(args[2..].join(" "))
    };

    let cli = Cli {
        command,
        options,
    };
    
    let r = import_export::import("/home/imanol/.baup/files.txt",&cli);
    println!("{:?}",r);
}
