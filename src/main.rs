use std::{env::args, time::Instant};

use minigrep_elijahkx::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let start = Instant::now();

    let config = Config::build(&args).expect("Problem parsing arguments: {err}");

    minigrep_elijahkx::run(config).expect("Application error: {e}");

    let elapsed_time = Instant::now() - start;
    println!(
        "Elapsed time: {:.6}",
        elapsed_time.as_secs_f64() + elapsed_time.subsec_micros() as f64 * 1e-6
    );
}
