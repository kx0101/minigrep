use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file_path in &config.file_paths {
        let contents = fs::read_to_string(file_path).expect("Failed to read file");

        let lines_that_contain_word = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        if lines_that_contain_word.is_empty() {
            println!(
                "Couldn't find the word {} in the file {}",
                &config.query, file_path
            );
        }

        println!("\n\nFile: {}", file_path);
        for (line_number, line) in lines_that_contain_word {
            println!("line {}: {}", line_number + 1, line);
        }
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
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: [query] [file path(s)] \n\n Optional arguments: \n\n -i \t Case insensitive search");
        }

        let query = &args[1];
        let mut file_paths = args[2..].to_vec();
        let mut ignore_case = false;

        if let Some(index) = args.iter().position(|arg| arg == "-i") {
            ignore_case = true;
            file_paths.remove(index - 2);
        }

        if args.iter().any(|arg| arg == "-i") {
            ignore_case = true;
        }

        Ok(Config {
            query: query.to_string(),
            file_paths,
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

        let expected = vec![(1, "safe, fast, productive.")];

        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec![(0, "Rust:"), (3, "Trust me.")];

        assert_eq!(expected, search_case_insensitive(query, contents));
    }

    #[test]
    fn build_config_too_few_args() {
        let args = vec!["minigrep".to_string(), "query".to_string()];

        let config = Config::build(&args);

        assert!(config.is_err());
    }
}
