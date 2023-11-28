
use std::{sync::{Mutex, Arc}, fs};

use crate::{search_case_insensitive, search};

pub fn process_file(
    file_path: &String,
    query: &String,
    ignore_case: bool,
    output: &Arc<Mutex<()>>,
) {
    if let Ok(contents) = fs::read_to_string(file_path) {
        let lines_that_contain_word = if ignore_case {
            search_case_insensitive(query, &contents)
        } else {
            search(query, &contents)
        };

        print_file_results(file_path, query, &lines_that_contain_word, output);
    }
}

fn print_file_results(
    file_path: &String,
    query: &String,
    lines: &[(usize, String)],
    output: &Arc<Mutex<()>>,
) {
    let _guard = output.lock().unwrap();
    println!("\n\x1b[0;32mFile: {}\x1b[0m", file_path);

    if lines.is_empty() {
        println!(
            "Couldn't find the word '{}' in the file '{}'",
            query, file_path
        );
    }

    for (line_number, line) in lines {
        println!("\x1b[0;31mLine {}\x1b[0m: {}", line_number + 1, line);
    }
}

fn read_current_directory_files() -> Result<Vec<String>, std::io::Error> {
    let entries = std::fs::read_dir(".")?;
    let file_paths = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().to_str().map(String::from))
        .collect();

    Ok(file_paths)
}

pub fn parse_arguments(args: &[String]) -> Result<(Vec<String>, bool), &'static str> {
    let mut file_paths = Vec::new();
    let mut ignore_case = false;

    for arg in args {
        match arg.as_str() {
            "-i" => ignore_case = true,
            "." => {
                if let Ok(paths) = read_current_directory_files() {
                    file_paths.extend_from_slice(paths.as_slice());
                } else {
                    return Err("Failed to read the current directory");
                }
            }
            _ => file_paths.push(arg.clone()),
        }
    }

    Ok((file_paths, ignore_case))
}
