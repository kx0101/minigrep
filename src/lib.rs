use std::{error::Error, fs, thread};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut handles = vec![];

    for file_path in &config.file_paths {
        let query = config.query.clone();
        let ignore_case = config.ignore_case;
        let file_path = file_path.clone();

        let handle = thread::spawn(move || {
            if let Ok(contents) = fs::read_to_string(&file_path) {

                let lines_that_contain_word = if ignore_case {
                    search_case_insensitive(&query, &contents)
                } else {
                    search(&query, &contents)
                };

                println!("\nFile: {}", &file_path);

                if lines_that_contain_word.is_empty() {
                    println!(
                        "Couldn't find the word '{}' in the file '{}'",
                        &query, &file_path
                    );
                }

                for (line_number, line) in lines_that_contain_word {
                    println!("Line {}: {}", line_number + 1, line);
                }

            } else {
                eprintln!("Failed to read file: {}", &file_path);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Failed to join thread");
    }

    Ok(())
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<(usize, String)> {
    let query = query.to_lowercase();
    let contents = contents.to_lowercase();

    search(&query, &contents)
}

pub fn search(query: &str, contents: &str) -> Vec<(usize, String)> {
    let mut matches = Vec::new();
    let n = contents.len();
    let m = query.len();
    let mut skip_table = [m; 256];

    for (i, &c) in query.as_bytes().iter().enumerate().take(m - 1) {
        skip_table[c as usize] = m - i - 1;
    }

    let mut i = 0;
    while i <= n - m {
        let mut j = m - 1;

        while query.as_bytes()[j] == contents.as_bytes()[i + j] {
            if j == 0 {
                let line_start = contents[..i].rfind('\n').map_or(0, |pos| pos + 1);
                let line_end = contents[i..].find('\n').map_or(n, |pos| i + pos);

                matches.push((
                    contents[..i].matches('\n').count(),
                    contents[line_start..line_end].to_string(),
                ));

                break;
            }

            j -= 1;
        }

        i += skip_table[contents.as_bytes()[i + m - 1] as usize];
    }

    matches
}

#[derive(Clone)]
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

        let expected = vec![(1, String::from("safe, fast, productive."))];

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

        let expected = vec![(0, String::from("rust:")), (3, String::from("trust me."))];

        assert_eq!(expected, search_case_insensitive(query, contents));
    }

    #[test]
    fn build_config_too_few_args() {
        let args = vec!["minigrep".to_string(), "query".to_string()];

        let config = Config::build(&args);

        assert!(config.is_err());
    }
}
