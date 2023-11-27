use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path).expect("Failed to read file");

    let lines_that_contain_word = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if lines_that_contain_word.is_empty() {
        println!(
            "Couldn't find the word {} in the file {}",
            &config.query, &config.file_path
        );
    }

    for (line_number, line) in lines_that_contain_word {
        println!("line {}: {}", line_number + 1, line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: [query] [file path] \n\n Optional arguments: \n\n -i \t Case insensitive search");
        }

        let query = &args[1];
        let file_path = &args[2];
        let mut ignore_case = false;

        if args.iter().any(|arg| arg == "-i") {
            ignore_case = true;
        }

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn build_config() {
        let args = vec![
            "minigrep".to_string(),
            "query".to_string(),
            "file_path".to_string(),
        ];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file_path");
    }

    #[test]
    fn build_config_too_few_args() {
        let args = vec!["minigrep".to_string(), "query".to_string()];

        let config = Config::build(&args);

        assert!(config.is_err());
    }
}
