use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &file_contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, filename: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();

    for line in filename.lines() {
        if line.contains(query) {
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
}
