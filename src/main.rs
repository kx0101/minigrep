use std::{env::args, fs};

fn main() {
    let args: Vec<String> = args().collect();

    let query = &args[1];
    let file = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file);

    let contents = fs::read_to_string(file).expect("Failed to read file");

    println!("With text:\n {}", contents);
}
