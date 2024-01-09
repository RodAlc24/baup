use crate::Cli;

pub fn import(cli: &Cli) {
    match &cli.options {
        Some(opts) => println!("Command: {:?} ; Options: {}", cli.command, opts),
        None => println!("Command: {:?} ; Options: <none>", cli.command),
    };   
}
