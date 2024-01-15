use clap::Parser;
use args::BaupArgs;

mod args;
mod commands{pub mod import_export;}

// Main function for the program
fn main() {
    let arguments = BaupArgs::parse();

    // let r = import_export::import("/home/imanol/.baup/files.txt",&cli);
    // println!("{:?}",r);

    println!("{:?}",arguments);
}
