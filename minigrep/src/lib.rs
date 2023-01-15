use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build<T: Iterator<Item=String>>(mut args: T, ignore_case: bool) -> Option<Config> {
        let _ = args.next()?;
        let query = args.next()?;
        let file_path = args.next()?;

        Some(
            Config {
                query,
                file_path,
                ignore_case
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_res = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in search_res {
        println!("{line}");
    }

    Ok(())
}


pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .map(|line| line.trim())
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();

    contents
    .lines()
    .map(|line| line.trim())
    .filter(|line| line.to_lowercase().contains(query))
    .collect()
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
        let query = "dUCt";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_insensitive(query, contents));
    }
}
