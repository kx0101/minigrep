use std::env::args;

use minigrep::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).expect("Problem parsing arguments: {err}");

    minigrep::run(config).expect("Application error: {e}");
}
