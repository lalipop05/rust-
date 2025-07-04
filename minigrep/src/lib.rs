use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Could not find query to search for.")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Could not find file path to search in.")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insnesitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
    
}

pub fn search_case_insnesitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insnesitive(query, contents));
    }
}