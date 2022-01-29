use std::{env, process};

use base_converter::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });
    
    if let Err(e) = base_converter::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}