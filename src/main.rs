use std::env;

use aoc::rogue;
use std::error::Error;

//use aoc::rogue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();
    env::set_var("RUST_BACKTRACE", "1");
    aoc::portal::show_maze(&filename);
}
