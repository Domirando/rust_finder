// extern crate cliRustApp;
use std::env;
use std::process;
use func::Config;
use std::io;

mod func;

fn main(){
    println!("Please enter a word/phrase you are looking for:"); 
    let mut search_input = String::new();
    io::stdin().read_line(&mut search_input).expect("failed to read line");
    
    println!("Please provide a file name:"); 
    let mut file_input = String::new();
    io::stdin().read_line(&mut file_input).expect("failed to read line");
    
    println!("Do you want me to be case sensitivie? (yes/no)"); 
    let mut sensitivity_input = String::new();
    io::stdin().read_line(&mut sensitivity_input).expect("failed to read line");
    
    let args: Vec<String> = vec![search_input, file_input, sensitivity_input];
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = func::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    } 
}
