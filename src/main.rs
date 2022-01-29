use std::{env, process};

use bconv::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });
    
    if let Err(e) = bconv::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}