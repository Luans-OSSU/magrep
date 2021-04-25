use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    let matches = if config.is_case_sensitive {
        search(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    for line in matches {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line)
        }
    }

    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line)
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_queried() {
        let query = "mete";
        let contents = "\
The starred and stately nights seemed haughty dames in jewelled velvets, 
nursing at home in lonely pride, the memory of their absent conquering Earls, 
the golden helmeted suns!";

        assert_eq!(vec!["the golden helmeted suns!"], search(query, contents))
    }

    #[test]
    fn multiple_results_queried() {
        let query = "ea";
        let contents = "\
Falstaff sweats to death and
lards the lean earth as he walks along.";

        assert_eq!(
            vec![
                "Falstaff sweats to death and",
                "lards the lean earth as he walks along."
            ],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive_query() {
        let query = "fal";
        let contents = "\
Falstaff sweats to death and
lards the lean earth as he walks falong.";

        assert_eq!(
            vec!["lards the lean earth as he walks falong."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive_query() {
        let query = "Fal";
        let contents = "\
Falstaff sweats to death and
lards the lean earth as he walks falong.";

        assert_eq!(
            vec![
                "Falstaff sweats to death and",
                "lards the lean earth as he walks falong."
            ],
            search_case_insensitive(query, contents)
        )
    }
}
