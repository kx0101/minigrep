use std::{env::args, process, time::Instant};

use minigrep::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let start = Instant::now();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let elapsed_time = Instant::now() - start;
    println!(
        "Elapsed time: {:.6}",
        elapsed_time.as_secs_f64() + elapsed_time.subsec_micros() as f64 * 1e-6
    );
}
