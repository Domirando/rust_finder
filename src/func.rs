use std::fs::read_to_string;
use std::error::Error;
use std::env;
use config::Config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let [query, filename, sensitivity] = config.iter().map(|&x| x.trim().split("\\").next().unwrap_or(&x)).collect();
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_sensitive_search() {
        let query = "world";
        let contents = "\
            Hello world
            helloooo
            ";
        assert_eq!(search(query, contents), vec!["Hello world"]);
    }
    
    #[test]
    fn test_case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
            Rust is good
            it works";
        assert_eq!(search_case_insensitive(query, contents), vec!["Rust is good"]);
    }
    }
