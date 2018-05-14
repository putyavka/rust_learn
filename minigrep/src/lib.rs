use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments");
		}
		let query = match args.next() {
			Some(s) => s,
			None => return Err("No query string"),
		};
		let filename = match args.next() {
			Some(s) => s,
			None => return Err("No filename string"),
		};
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
	let mut file = File::open(config.filename)?;
	let mut contents = String::new();
	
	file.read_to_string(&mut contents)?;
	
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};
	for line in results {
		println!("{}", line);
	}
	
	Ok(())
}

pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}
pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	contents.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Trust me.";
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
	
	#[test]
	fn case_insensitive() {
		let query = "rUst";
		let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Trust me.";
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
