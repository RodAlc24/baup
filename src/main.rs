use clap::Parser;
use args::BaupArgs;

mod args;
mod commands{pub mod import_export;}

// Main function for the program
fn main() {
    // Parse the arguments using the clap utility
    let arguments = BaupArgs::parse();

    let r = commands::import_export::import("/home/imanol/.baup/files.txt");
    println!("{:?}",r);

    println!("{:?}",arguments.command);
}
