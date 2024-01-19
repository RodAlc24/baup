use clap::Parser;
use args::BaupArgs;
use args::Com;

mod args;
mod commands {
    pub mod import_export;
    // pub mod git_diff;
}

// Main function for the program
fn main() {
    // Parse the arguments using the clap utility
    let arguments = BaupArgs::parse();

    match arguments.command {
        Com::Import(options) => println!("Import ; {:?}", options),
        Com::Export => println!("Export"),
        Com::Diff => println!("Diff"),
        Com::Commit(options) => println!("Commit ; {:?}", options),
        Com::Push(options) => println!("Push ; {:?}", options),
        Com::Pull(options) => println!("Pull ; {:?}", options),
        Com::Edit => println!("Edit"),
    }

    // let output = Command::new("ls").args(["-a"]).current_dir("/home/imanol/.baup").output();
    // println!("{:?}",String::from_utf8_lossy(&output.unwrap().stdout));

    // let r = commands::import_export::import("/home/imanol/.baup/files.txt");
    // println!("{:?}",r);

    // println!("{:?}",arguments.command);
}
