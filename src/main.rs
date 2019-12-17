use aoc::intcode;
use aoc::vacuum;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let filename: String = args[1].as_str().parse().unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    let mut program = intcode::load_program(&filename);
    vacuum::view(program);
}
