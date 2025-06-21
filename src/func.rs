use std::fs::read_to_string;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let query = config.query.trim().split("\\").next().unwrap_or(&config.query);
	let filename = config.filename.trim().split('\\').next().unwrap_or(&config.filename);
	let sensitivity = config.case_sensitive.trim().split("\\").next().unwrap_or(&config.case_sensitive);
	let contents = read_to_string(filename)?;
	let results = if (sensitivity == "yes") {
	    search(query, &contents)
	} else {
	    search_case_insensitive(query, &contents)
	};   
	if results.len() >= 1 {
        println!("\nso here is the finding - {:?}", results);
	}else {
	    println!("\nno matches");
	}
	
	Ok(())
}

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
	    if args.len() < 2 {
			return Err("not enough arguments"); 
		}               
		let query = args[0].clone();
		let filename = args[1].clone();
		let case_sensitive = args[2].clone();
		Ok( Config { query, filename, case_sensitive } )
	}
}	
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}