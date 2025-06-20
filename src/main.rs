// extern crate cliRustApp;
use std::env;
use std::process;
use cliRustApp::Config;

fn main() {
    // let a: String = env::args().nth(1).expect("cuz you damn didnt work");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    println!("searching for a sentence containing - '{}'", config.query);
    println!("in file {}", config.filename);
    if let Err(e) = cliRustApp::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    } 
}
