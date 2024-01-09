use std::env;
mod import;

// S
struct Cli {
    command: String,
    options: Option<String>,
}

// Main function for the program
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Help");
        return;
    }
    
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
    
    import::import(&cli);
}
