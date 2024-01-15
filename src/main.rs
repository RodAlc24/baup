use clap::Parser;
use args::BaupArgs;
use args::Command;

mod args;
mod commands{pub mod import_export;}

// Main function for the program
fn main() {
    // Parse the arguments using the clap utility
    let arguments = BaupArgs::parse();

    match arguments.command {
        Command::Import(options) => println!("Import ; {:?}", options),
        Command::Export => println!("Export"),
        Command::Diff => println!("Diff"),
        Command::Commit(options) => println!("Commit ; {:?}", options),
        Command::Push(options) => println!("Push ; {:?}", options),
        Command::Pull(options) => println!("Pull ; {:?}", options),
        Command::Edit => println!("Edit"),
    }

    let r = commands::import_export::import("/home/imanol/.baup/files.txt");
    println!("{:?}",r);

    // println!("{:?}",arguments.command);
}
