pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: String,
}

impl Config {
	pub fn new(search: String, file: String, case_sensitive: String) -> Result<Config, &str> {
	    if args.len() < 2 {
			return Err("not enough arguments"); 
		}               
		let query = args[0].clone();
		let filename = args[1].clone();
		let case_sensitive = args[2].clone();
		Ok( Config { query, filename, case_sensitive } )
	}
}