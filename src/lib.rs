use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");

    println!("With text:\n {}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: [query] [file path]");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        })
    }
}
