use minigrep::Config;
use std::{env, process};

// Separation of Concerns for Binary Projects
// 1.split your program into a main.rs and a lib.rs and move you program's logic to lib.rs
// so we can test it and have a main.rs with fewer responsibilities
// 2.as long as your command line parsing logic is small, it can remain in main.rs
// 2.when the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs

// Calling the command line parsing logic with the argument values
// Setting up any other configuration
// Calling a run function in lib.rs
// Handling the error if run returns an error

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // stdout
        // println!("Problem parsing arguments: {}", err);
        // stderr
        eprintln!("Problem parsing arguments: {}", err);
        // return type ! can match any type.
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
