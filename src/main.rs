use std::env;
use std::process;

mod assembler;
mod config;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Input file: {}", config.filename);

    if let Err(e) = assembler::assembly(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
