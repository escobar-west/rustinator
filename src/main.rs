use std::env;
use std::process;
use rustinator::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rustinator::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    };
}
