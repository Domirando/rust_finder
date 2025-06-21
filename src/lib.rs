use std::fs::read_to_string;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("{}", fs::read_to_string(config.filename));
	// let contents = fs::read_to_string(config.filename)?;
	for line in read_to_string(config.filename).unwrap().lines() {
	    println!("{}", line.to_string());
        // result.push(line.to_string())
    }
	// println!("with text:\n{}", result);
	// let results = if config.case_sensitive {
	//     search(&config.query, &contents)
	// } else {
	//     search_case_insensitive(&config.query, &contents)
	// };   
	// for line in search(&config.query, &contents) {
	//     println!("so here is the finding - {}", line)
	// }
	Ok(())
}

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
	    if args.len() < 2 {
			return Err("not enough arguments"); 
		}               
		let query = args[0].clone();
		let filename = args[1].clone();
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
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