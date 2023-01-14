use std::{fs, error::Error};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String], ignore_case: bool) -> Config<'a> {
        Config {
            query: &args[1],
            file_path: &args[2],
            ignore_case
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_res = if config.ignore_case {
        search_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };

    for line in search_res {
        println!("{line}");
    }

    Ok(())
}


pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = vec![];

    for line in contents.lines() {
        let m_line = line.trim();

        if m_line.contains(query) {
            vec.push(m_line);
        }
    }

    vec
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        let m_line = line.trim();

        if m_line.to_lowercase().contains(&query) {
            vec.push(m_line);
        }
    }

    vec
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
