mod utils;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use utils::{parse_arguments, process_file};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let output = Arc::new(Mutex::new(()));
    let query = config.query;

    std::thread::scope(|scope| {
        for file_path in &config.file_paths {
            scope.spawn(|| {
                let ignore_case = config.ignore_case;
                let file_path = file_path.clone();
                let output_mutex = Arc::clone(&output);

                process_file(&file_path, &query, ignore_case, &output_mutex);
            });
        }
    });

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<(usize, &'a str)> {
    let start = Instant::now();

    let mut matches: Vec<(usize, &str)> = Vec::new();
    let n = contents.len();
    let m = query.len();
    let mut skip_table = [m; 256];

    let cmp = if case_insensitive {
        |a: u8, b: u8| a.eq_ignore_ascii_case(&b)
    } else {
        |a: u8, b: u8| a == b
    };

    for (i, &c) in query.as_bytes().iter().enumerate().take(m - 1) {
        skip_table[c as usize] = m - i - 1;
    }

    let contents_bytes = contents.as_bytes();
    let query_bytes = query.as_bytes();

    let mut i = 0;
    while i <= n - m {
        let mut j = m - 1;

        while cmp(query_bytes[j], contents_bytes[i + j]) {
            if j == 0 {
                let line_start = contents[..i].rfind('\n').map_or(0, |pos| pos + 1);
                let line_end = contents[i..].find('\n').map_or(n, |pos| i + pos);

                matches.push((
                    contents[..i].matches('\n').count(),
                    &contents[line_start..line_end],
                ));

                break;
            }

            j -= 1;
        }

        i += skip_table[contents_bytes[i + m - 1] as usize];
    }

    let elapsed_time = Instant::now() - start;
    println!(
        "Elapsed time: {:.6} for {} search",
        elapsed_time.as_secs_f64() + elapsed_time.subsec_micros() as f64 * 1e-6,
        if case_insensitive { "case_insensitive" } else { "regular" }
    );

    matches
}

// pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<(usize, &'a str)> {
//     let mut matches: Vec<(usize, &str)> = Vec::new();
//     let n = contents.len();
//     let m = query.len();
//     let mut skip_table = [m; 256];
//
//     let cmp = if case_insensitive {
//         |a: u8, b: u8| a.eq_ignore_ascii_case(&b)
//     } else {
//         |a: u8, b: u8| a == b
//     };
//
//     for (i, &c) in query.as_bytes().iter().enumerate().take(m - 1) {
//         skip_table[c as usize] = m - i - 1;
//     }
//
//     let mut i = 0;
//     while i <= n - m {
//         let mut j = m - 1;
//
//         let query = query.as_bytes();
//         let contents = contents.as_bytes();
//
//         while cmp(query[j], contents[i + j]) {
//             if j == 0 {
//                 let line_start = contents[..i].rfind('\n').map_or(0, |pos| pos + 1);
//                 let line_end = contents[i..].find('\n').map_or(n, |pos| i + pos);
//
//                 matches.push((
//                     contents[..i].matches('\n').count(),
//                     &contents[line_start..line_end],
//                 ));
//
//                 break;
//             }
//
//             j -= 1;
//         }
//
//         i += skip_table[contents[i + m - 1] as usize];
//     }
//
//     matches
// }

#[derive(Clone)]
pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: [query] [file path(s) or . for current directory] \n\n Optional arguments: \n\n -i \t Case insensitive search");
        }

        let query = args[1].clone();
        let (file_paths, ignore_case) = parse_arguments(&args[2..]).unwrap_or((vec![], false));

        Ok(Config {
            query,
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

        let expected = vec![(1, ("safe, fast, productive."))];

        assert_eq!(expected, search(query, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec![(0, ("rust:")), (3, ("trust me."))];

        assert_eq!(expected, search(query, contents, true));
    }

    #[test]
    fn build_config_too_few_args() {
        let args = vec!["minigrep".to_string()];

        let config = Config::build(&args);

        assert!(config.is_err());
    }
}
