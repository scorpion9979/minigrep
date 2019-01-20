//! # Minigrep
//!
//! `minigrep` is a basic Rust implementation of grep

use std::fs;
use std::error::Error;
use std::env;

/// A structure to keep the arguments passed to minigrep via the terminal
pub struct Config {
    /// Query to search for in the given file
    pub query: String,
    /// The file to search
    pub filename: String,
    /// Whether the search should be case-sensitive or not
    pub case_sensitive: bool,
}

impl Config {
    /// Creates a new instance of Config using the passed arguments iterator.
    /// The search is configured to be case-sensitive by default.
    ///
    /// # Examples:
    /// 
    /// ```
    /// // would produce an Err if required arguments not provided from terminal
    /// let config = minigrep::Config::new(std::env::args());
    /// ```
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config{query, filename, case_sensitive});
    }
}

/// Run the program with the given Config instance
///
/// # Examples:
/// 
/// ```
/// let (query, filename, case_sensitive) = (String::from("to"), String::from("poem.txt"), false);
/// let config = minigrep::Config {query, filename, case_sensitive,};
/// minigrep::run(config);
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let results = contents.lines().filter(|line| {
        line.contains(&query)
    }).collect();
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let results = contents.lines().filter(|line| {
        line.to_lowercase().contains(&query)
    }).collect();
    results
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
        Pick three.
        Duct tape.".replace("        ", "");

        assert_eq!(
            vec!["safe, fast, productive."],
            search(&query, &contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.".replace("        ", "");

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&query, &contents)
        );
    }
}