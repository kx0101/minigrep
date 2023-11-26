use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }
}
