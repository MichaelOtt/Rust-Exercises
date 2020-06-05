use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem handling arguments: {}", e);
        process::exit(1);
    });

    println!("Query: {}, Filename: {}", config.query, config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
    
}



