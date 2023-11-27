# MiniGrep

MiniGrep is a simple command-line utility written in Rust for searching text within files. It provides the capability to search for a specified query string in a given file, displaying lines containing the query along with their line numbers.

## Usage

## Command Syntax

```bash
minigrep [query] [file path] [-i]
```

- query: The text string to search for within the file.
- file path: The path to the file in which the search will be performed.
- -i (Optional): Performs a case-insensitive search. If provided, the search will ignore the case of the query string.

## Example

```bash
# Perform a case-sensitive search for the word "hello" in the file "sample.txt"
minigrep hello sample.txt

# Perform a case-insensitive search for the word "world" in the file "sample.txt"
minigrep world sample.txt -i

# Redirects the output to a new file. Error messages are not captured in the output file.
minigrep world sample.txt > output.txt 
```

## How It Works

The program uses the following components:

- run: The main function responsible for executing the search based on the provided configurations.
- search: Performs a case-sensitive search for the query within the file's contents, returning a vector of tuples containing line numbers and matching lines.
- search_case_insensitive: Similar to search, but performs a case-insensitive search.
- Config: A structure that holds the query, file path, and a boolean flag to indicate whether the search should be case-insensitive.
- Config::build: Builds the configuration based on the command-line arguments provided.

## Running Tests

```bash
cargo test
```

## Notes
The program will display lines containing the query string along with their line numbers within the specified file.
If no matching lines are found, it will indicate that the query was not found in the file.
